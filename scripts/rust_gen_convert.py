# %% import and const def
from pathlib import Path
import polars as pl

workspace_dir = Path(__file__).parent.parent
csv_filename = workspace_dir / 'src/convert/convert.csv'
# https://github.com/enigo-rs/enigo/blob/v0.3.0/src/keycodes.rs
enigo__keycodes_rs = workspace_dir / 'scripts/cache/enigo-0.3.0' / 'raw.githubusercontent.com--enigo-rs--enigo--refs--tags--v0.3.0--src--keycodes.rs'
# https://github.com/enigo-rs/enigo/blob/v0.3.0/src/macos/macos_impl.rs
enigo__macos_impl_rs = workspace_dir / 'scripts/cache/enigo-0.3.0' / 'raw.githubusercontent.com--enigo-rs--enigo--refs--tags--v0.3.0--src--macos--macos_impl.rs'

# https://github.com/microsoft/windows-rs/blob/63/crates/libs/sys/src/Windows/Win32/UI/Input/KeyboardAndMouse/mod.rs
windows__keyboard_and_mouse = workspace_dir / 'scripts/cache/windows-63' / 'raw.githubusercontent.com--microsoft--windows-rs--refs--tags--63--crates--libs--sys--src--Windows--Win32--UI--Input--KeyboardAndMouse--mod.rs'

# %% functions
columns = [
  ("hut",        ";hut::HUT"              , "todo!(HUT)"),
  ("hut_value",  "hkcode"                 , ""),
  ("winput",     "winput::Vk"             , "na!(Vk)"),
  ("vk",         "windows::VIRTUAL_KEY"   , "todo!(VK_)"),
  ("vk_value",   "vkcode"                 , ""),
  ("make1_value","make1code"            , ""),
  ("enigo",      "enigo::Key"             , "todo!(Enigo)"),
  ("enigo_attr", "#[cfg(enigo::Key)]"     , ""),
  ("keysym",     "xkeysym::Keysym"        , "todo!(Keysym)"),
  ("keysym_value","keysym_code"           , ""),
  ("cg",         "macos::KeyCode"         , "todo!(CG)"),
  ("cg_value",   "cgcode"                 , ""),
]
col_map = {col[0]: col[1] for col in columns}
col_default = {col[0]: col[2] for col in columns}
col_map_rev = {col[1]: col[0] for col in columns}
def load_csv(filename: str):
  df = pl.read_csv(filename)
  df = df.select([
    pl.col(col).alias(col_map_rev.get(col.strip(), col.strip())).str.strip_chars(' ') for col in df.columns if col.strip()
  ])
  return df

def save_csv(df: pl.DataFrame, filename: Path):
  adj = 0
  def get_col_name(col: str, length: int):
    nonlocal adj
    col_name = col_map.get(col, col)
    padding = length - len(col_name) + adj
    adj = min(0, padding)
    if col.endswith('code') or col.endswith('_value'):
      col_name = " " * (padding - 1) + col_name + " " * min(padding, 1)
    else:
      col_name = " " * min(padding, 1) + col_name + " " * (padding - 1)
    return col_name
  def pad_left_col(col: str, length: int):
    if col.endswith('code') or col.endswith('_value'):
      return pl.col(col).str.pad_start(length - 1 if length >= 10 else length - 2).str.pad_end(length)
    return pl.col(col).str.pad_end(length - 1 if length >= 10 else length - 2).str.pad_start(length)
  filename = str(filename) + ".new"
  df = df_fill_na(df)
  max_col_lengths = df.select([
    pl.col(col).str.len_chars().max().alias(col) for col in df.columns
  ]).to_dicts()[0]
  max_col_lengths = {
    k: int((v + 1) * 1.1 + 2) if v > 10 else v + 4 for k, v in max_col_lengths.items()
  }

  df.select([
    pad_left_col(col, max_col_lengths[col]).alias(get_col_name(col, max_col_lengths[col])) for col in df.columns
  ]+[pl.lit(None).alias(" ")]).write_csv(filename)

def parse_rs_blocking(code: str, blocking: tuple[str, str] | None = None) -> list[str]:
  result = []
  if blocking is None:
    return code.splitlines()
  starts = False
  indent_of_starts = 0
  for line in code.splitlines():
    if not starts:
      if line.strip().startswith(blocking[0]):
        indent_of_starts = len(line) - len(line.lstrip())
        starts = True
      continue
    elif line.strip().startswith(blocking[1]):
      if indent_of_starts == len(line) - len(line.lstrip()):
        break
    result.append(line)
  return result

def parse_rs_match(code: str, starts_with: str = "", blocking: tuple[str, str] | None = None) -> list[tuple[str, str]]:
  result = []
  for line in parse_rs_blocking(code, blocking):
    if "=>" not in line: continue
    if not line.strip().startswith(starts_with):
      continue
    [key, value] = line.strip().strip(',').split("=>")
    if '|' in key:
      for k in key.split('|'):
        result.append((k.strip(), value.strip()))
    else:
      result.append((key.strip(), value.strip()))
  return result

def parse_rs_attr(code: str, line_starts: str = "#[cfg(", line_not_starts: list[str] = ["#[", "//"], blocking: tuple[str, str] | None = None):
  result = []
  attr = None
  for line in parse_rs_blocking(code, blocking):
    line = line.strip()
    if len(line) == 0:
      continue
    if line.startswith(line_starts):
      attr = line
    for i in line_not_starts:
      if line.startswith(i):
        break
    else:
      result.append((line, attr))
      attr = None
  return result

def parse_rs_const(code: str, line_starts: str = "const ", line_contains: str = "=", blocking: tuple[str, str] | None = None) -> list[tuple[str, str]]:
  result = []
  for line in parse_rs_blocking(code, blocking):
    line = line.strip()
    if line_starts in line and line_contains in line:
      line = line.removeprefix("pub ").removeprefix("const ")
      [key, value] = line.split("=", 1)
      key = key.split(":")[0].strip()
      value = value.split(";")[0].strip()
      result.append((key, value))
  return result

def df_check_key_map(df: pl.DataFrame, df_map: pl.DataFrame):
  col_key = df_map.columns[0]
  col_value = df_map.columns[1]
  col_key_right = f"{col_key}_right"
  col_value_right = f"{col_value}_right"
  mismatched_keys = df.with_columns(
    pl.col(col_key).str.strip_chars('*').alias("_key"),
  ).join(df_map, left_on="_key", right_on=col_key, how="right", nulls_equal=True).select([col_key, col_value, col_key_right, col_value_right]).filter(pl.col(col_value).eq_missing(pl.col(col_value_right)).not_())
  if not mismatched_keys.is_empty():
    print(f"Mismatch found in {col_key} => {col_value} mapping:")
    print(mismatched_keys)
  return mismatched_keys

def df_insert_key_map(df: pl.DataFrame, df_map: pl.DataFrame, after: str, default: str | None = None, force: bool = False):
  col_key = df_map.columns[0]
  col_value = df_map.columns[1]
  if default is None:
    default = col_default.get(col_value, f"todo!({col_value})")
  if force:
    print(f"Force insert {col_value} after {after}")
  if not force and col_value in df.columns:
    return df
  df1 = df.select(pl.col(col_key).str.strip_chars('*')).join(df_map, how="full", on=col_key).select(col_value).fill_null(pl.lit(default))
  df = df.select(df.columns)
  if col_value in df.columns:
    df.replace_column(df.get_column_index(col_value), df1[col_value])
  else:
    index = df.get_column_index(after)
    print(df.columns, after, index)
    df = df.insert_column(index, df1[col_value])
  return df

def df_fill_na(df: pl.DataFrame):
  return df.select([
    pl.col(col).fill_null(pl.lit(col_default.get(col, f"todo!({col})"))).alias(col) for col in df.columns
  ])

def df_as_null(df: pl.DataFrame, na_list = None, columns: list[str] | None = None):
  if na_list is None:
    na_list = ["todo!", "na!", "n!", "none!"]
  def make_cond(col: str, na_list: list[str]):
    cond = pl.col(col).str.strip_chars(' ').str.len_chars().eq(0)
    for na in na_list:
      cond = cond.or_(pl.col(col).str.starts_with(na))
    return cond
  if columns is None:
    columns = df.columns
  for col in columns:
    df.get_column_index(col)
  return df.select([
    pl.when(
      make_cond(col, na_list)
    ).then(None).otherwise(pl.col(col)).alias(col)
    if col in columns else pl.col(col)
    for col in df.columns
  ])

attr_map = {
  '#[cfg(target_os = "windows")]': 'windows',
  # '#[cfg(target_os = "linux")]': 'linux',
  '#[cfg(all(unix, not(target_os = "macos")))]': 'linux',
  '#[cfg(target_os = "macos")]': 'macos',
  '#[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]': 'windows|linux'
}

# %% load csv
df = load_csv(csv_filename)
save_csv(df, csv_filename)

# %% load and check vk => vk_value from windows__keyboard_and_mouse
with open(windows__keyboard_and_mouse) as rsfile:
  content = rsfile.read()

data = parse_rs_const(content, line_starts="pub const VK_", line_contains=": VIRTUAL_KEY =")
map_vk_values = {k: "0x%02X" % int(v.removesuffix('u16')) for k,v in data}
df_map_vk_values = pl.DataFrame({
  "vk": map_vk_values.keys(),
  "vk_value": map_vk_values.values(),
})
df = df_insert_key_map(df, df_map_vk_values, after="vk", force=True)
df_check_key_map(df, df_map_vk_values)

# %% load and check enigo => enigo_attr from enigo__keycodes_rs
with open(enigo__keycodes_rs) as rsfile:
  content = rsfile.read()
enigo_attrs = parse_rs_attr(content, blocking=("pub enum Key {", "}"))
map_enigo_attrs = {"Enigo::" + k.strip(","): "#[" + attr_map[v] + "]" for k, v in enigo_attrs if v}
df_map_enigo_attrs = pl.DataFrame({
  "enigo": map_enigo_attrs.keys(),
  "enigo_attr": map_enigo_attrs.values(),
})

df = df_fill_na(df.join(df_map_enigo_attrs.select('enigo'), on="enigo", coalesce=True, how="full", maintain_order="left_right"))
df = df_insert_key_map(df, df_map_enigo_attrs, after="enigo")
df_check_key_map(df, df_map_enigo_attrs)

# %% load and check enigo => [keysym, vk, cg] from enigo__keycodes_rs/enigo__macos_impl_rs
with open(enigo__keycodes_rs) as rsfile:
  content = rsfile.read()

data = parse_rs_match(content, blocking=('impl From<Key> for xkeysym::Keysym {', '}'))
key_map_enigo_keysym = {k.replace("Key::", "Enigo::"): v for k,v in data if "Key::" in k and "(" not in k}
df_map_enigo_keysym = pl.DataFrame({
  "enigo": key_map_enigo_keysym.keys(),
  "keysym": key_map_enigo_keysym.values()
})
df_check_key_map(df, df_map_enigo_keysym)

data = parse_rs_match(content, blocking=('impl TryFrom<Key> for windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY {', '}'))
key_map_enigo_vk = {k.replace("Key::", "Enigo::"): v for k,v in data if "Key::" in k and "(" not in k}
df_map_enigo_vk = pl.DataFrame({
  "enigo": key_map_enigo_vk.keys(),
  "vk": key_map_enigo_vk.values()
})
df_check_key_map(df, df_map_enigo_vk)

with open(enigo__macos_impl_rs) as rsfile:
  content = rsfile.read()
data = parse_rs_match(content, blocking=('impl TryFrom<Key> for core_graphics::event::CGKeyCode {', '}'))
def fix_macos_impl_value(v: str):
  # if v.startswith("ANSI_"): return "KeyCode::" + v
  if v.startswith("return Err(())"): return None
  if v.startswith("ANSI_"): return "KeyCodeExt::kVK_ANSI_" + v.removeprefix("ANSI_").title().replace("_", "")
  try: return f"CGKeyCode({int(v)}).0"
  except: pass
  return v
key_map_enigo_cg = {k.replace("Key::", "Enigo::"): fix_macos_impl_value(v) for k,v in data if "Key::" in k and "(" not in k} | \
  {f"Enigo::Num{i}": f"KeyCodeExt::kVK_ANSI_{i}" for i in range(10)} | {f"Enigo::{chr(X)}": f"KeyCodeExt::kVK_ANSI_{chr(X)}" for X in range(ord('A'), ord('Z')+1)}

df_map_enigo_cg = pl.DataFrame({
  "enigo": key_map_enigo_cg.keys(),
  "cg": key_map_enigo_cg.values()
})
df = df_insert_key_map(df, df_map_enigo_cg, after="keysym")
df_check_key_map(df, df_map_enigo_cg)

# %%
save_csv(df, csv_filename)

# %% generating example_gen_numeric.rs
# set all columns starts with 'todo!', 'na!', 'n!' or 'none!' to None
df_test = df_as_null(df)
def build_code(df: pl.DataFrame, col: str, format: str, value_prefix: str = "", value_suffix: str = "", default: str | None = "    ", max_len: int | None = None):
  result = []
  max_len = df[col].str.len_chars().max() + 1 if max_len is None else max_len
  for i in df[col]:
    i: str | None = i
    if i:
      line = f'writeln!(w, "{{:{max_len}}}, {format},", "{i}", {value_prefix}{i.strip('*')}{value_suffix})?;'
    if i and not i.endswith('*'):
      result.append("          " + line)
    elif not i:
      result.append(f'  if any {{ writeln!(w, "{{:{max_len}}}, {{}},", "{i}", "{default}")?; }}')
    else:
      result.append("  if any {" + line + "}")
  return "\n".join(result).removeprefix("  ")

output_all = False

lines = []
main_lines = []
main_lines.append("""
  let path = &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/cache/numeric");
  let file = |name: &str| std::fs::File::create(path.join(name)).unwrap();
  std::fs::create_dir_all(path).unwrap();
""".removeprefix("\n"))

name = "hut_04"
lines.append(f"""
fn print_{name}(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {{
  {"use vkey::deps::hut::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad};"}
  {build_code(df_test, "hut", "0x{:X}", value_suffix=".usage_value()")}
  Ok(())
}}
""")
main_lines.append(f"""
  println!("\\n\\n============== {name.upper().replace("_", " ")} ===============");
  print_{name}(&mut file("{name}.txt"), true).ok();
  print_{name}(&mut std::io::stdout(), false).ok();
""")

name = "windows"
lines.append(f"""
fn print_{name}(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {{
  {"use vkey::mirror::windows as keys;"}
  {build_code(df_test, "vk", "0x{:02X}", value_prefix="keys::", value_suffix=".0")}
  Ok(())
}}
""")
main_lines.append(f"""
  println!("\\n\\n============== {name.upper().replace("_", " ")} ===============");
  print_{name}(&mut file("{name}.txt"), true).ok();
  print_{name}(&mut std::io::stdout(), false).ok();
""")

name = "keysym"
lines.append(f"""
fn print_{name}(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {{
  {"use vkey::deps::xkeysym::Keysym;"}
  {build_code(df_test, "keysym", "0x{:04X}", value_suffix=".raw()")}
  Ok(())
}}
""")
main_lines.append(f"""
  println!("\\n\\n============== {name.upper().replace("_", " ")} ===============");
  print_{name}(&mut file("{name}.txt"), true).ok();
  print_{name}(&mut std::io::stdout(), false).ok();
""")

name = "macos"
lines.append(f"""
fn print_{name}(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {{
  {"use vkey::mirror::{macos::KeyCode, macos_ext::{CGKeyCode, KeyCodeExt}};"}
  {build_code(df_test, "cg", "0x{:02X}", value_prefix="CGKeyCode::from(", value_suffix=").0")}
  Ok(())
}}
""")
main_lines.append(f"""
  println!("\\n\\n============== {name.upper().replace("_", " ")} ===============");
  print_{name}(&mut file("{name}.txt"), true).ok();
  print_{name}(&mut std::io::stdout(), false).ok();
""")

name = "win_makecode1"
lines.append(f"""
#[cfg(windows)]
fn print_{name}(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {{
  {"use vkey::deps::windows::{MapVirtualKeyExW, MAPVK_VK_TO_VSC_EX};"}
  {"let makecode = |vkey| unsafe { MapVirtualKeyExW(vkey, MAPVK_VK_TO_VSC_EX, None) };"}
  {build_code(df_test, "vk_value", "0x{:02X}", value_prefix="makecode(", value_suffix=")")}
  Ok(())
}}
""")
main_lines.append(f"""
  #[cfg(windows)] {{
  println!("\\n\\n============== {name.upper().replace("_", " ")} ===============");
  print_{name}(&mut file("{name}.txt"), true).ok();
  print_{name}(&mut std::io::stdout(), false).ok();
  }}
""")

with open(workspace_dir / 'scripts/example_gen_numeric.rs', 'w', newline='\n') as f:
  f.write("fn main() {\n")
  for line in main_lines:
    f.write(line + "\n")
  f.write("}\n")

  f.write("\n\n")
  for line in lines:
    f.write(line + "\n")

# %%
"""
# %%
!cargo run --example gen_numeric

# %%
"""
# %% load and check *_value from cache/numeric/*.txt
df = load_csv(csv_filename)

numeric_dict = [
  ("hut", "hut_value", "scripts/cache/numeric/hut_04.txt"),
  ("vk", "vk_value", "scripts/cache/numeric/windows.txt"),
  ("keysym", "keysym_value", "scripts/cache/numeric/keysym.txt"),
  ("cg", "cg_value", "scripts/cache/numeric/macos.txt"),
  ("vk_value", "make1_value", "scripts/cache/numeric/win_makecode1.txt"),
]

def load_code(col_key: str, col_value: str, filename: str):
  df_code = pl.read_csv(filename, has_header=False)
  df_code = df_code.select([
    pl.col('column_1').str.strip_chars(' ').alias(col_key),
    pl.col('column_2').str.strip_chars(' ').alias(col_value),
  ])
  return df_code

def check_row_eq(df_left: pl.DataFrame, df_right: pl.DataFrame, col_key: str):
  df_left = df_left.select(pl.col(col_key))
  df_right = df_right.select(pl.col(col_key))
  df_left = df_left.hstack(df_as_null(df_left.select(_key=col_key)))
  df_right = df_right.hstack(df_as_null(df_right.select(_key=col_key), na_list=["None"]))

  mismatched = df_left.hstack(df_right.select(
    pl.col(col).alias(f"{col}_right") for col in df_right.columns
  )).filter(pl.col("_key").eq_missing(pl.col("_key_right")).not_())
  if not mismatched.is_empty():
    print(f"Mismatch found in {col_key} row:")
    print(mismatched)
  return mismatched

def insert_or_check(df: pl.DataFrame, df_code: pl.DataFrame, after: str | None = None, force: bool = False):
  col_key = df_code.columns[0]
  col_code = df_code.columns[1]
  after = col_key if after is None else after

  check_row_eq(df, df_code, col_key)
  if col_code in df.columns:
    check_row_eq(df, df_code, col_code)
    if force:
      df.replace_column(df.get_column_index(col_code), df_code[col_code])
  else:
    df.insert_column(df.get_column_index(col_key) + 1, df_code[col_code])

for col_key, col_code, filename in numeric_dict:
  df_code = load_code(col_key, col_code, workspace_dir / filename)
  insert_or_check(df, df_code, after=col_key)

df = df_as_null(df, na_list=["0x00"], columns=["make1_value"])
save_csv(df, csv_filename)
# %%

# %%
