# %%
from pathlib import Path
import polars as pl

workspace_dir = Path(__file__).parent.parent
csv_filename = workspace_dir / 'src/convert/convert.csv'
# https://github.com/enigo-rs/enigo/blob/v0.3.0/src/keycodes.rs
enigo__keycodes_rs = workspace_dir / 'scripts/cache/enigo-0.3.0' / 'raw.githubusercontent.com--enigo-rs--enigo--refs--tags--v0.3.0--src--keycodes.rs'
# https://github.com/enigo-rs/enigo/blob/v0.3.0/src/macos/macos_impl.rs
enigo__macos_impl_rs = workspace_dir / 'scripts/cache/enigo-0.3.0' / 'raw.githubusercontent.com--enigo-rs--enigo--refs--tags--v0.3.0--src--macos--macos_impl.rs'

# %%
columns = [
  ("hut",        ";hut::HUT"              , "todo!(HUT)"),
  ("winput",     "winput::Vk"             , "na!(Vk)"),
  ("vk",         "windows::VIRTUAL_KEY"   , "todo!(VK_)"),
  ("vk_value",   "vkcode"                 , "n!()"),
  ("hut_value",  "hkcode"                 , "n!()"),
  ("enigo",      "enigo::Key"             , "todo!(Enigo)"),
  ("enigo_attr", "#[cfg(enigo::Key)]"     , ""),
  ("keysym",     "xkeysym::Keysym"        , "todo!(Keysym)"),
  ("cg",         "macos::KeyCode"         , "todo!(CG)"),
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
    col_name = " " * min(padding, 1) + col_name + " " * (padding - 1)
    return col_name
  filename = str(filename) + ".new"
  max_col_lengths = df.select([
    pl.col(col).str.len_chars().max().alias(col) for col in df.columns
  ]).to_dicts()[0]
  max_col_lengths = {
    k: int((v + 1) * 1.1 + 2) if v > 10 else v + 4 for k, v in max_col_lengths.items()
  }

  df.select([
    (pl.col(col).str.pad_end(max_col_lengths[col] - 1 if max_col_lengths[col] >= 10 else max_col_lengths[col] - 2).str.pad_start(max_col_lengths[col])
      .alias(get_col_name(col, max_col_lengths[col]))) for col in df.columns
  ]+[pl.lit(None).alias(" ")]).write_csv(filename)

def parse_rs_match(code: str, starts_with: str = "", blocking: tuple[str, str] | None = None) -> list[tuple[str, str]]:
  result = []
  starts = blocking is None
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

def df_check_key_map(df: pl.DataFrame, df_map: pl.DataFrame):
  col_key = df_map.columns[0]
  col_value = df_map.columns[1]
  col_value_right = f"{col_value}_right"
  mismatched_keys = df.join(df_map, on=col_key, how="right", nulls_equal=True).select([col_key, col_value, col_value_right]).filter(pl.col(col_value).eq_missing(pl.col(col_value_right)).not_())
  if not mismatched_keys.is_empty():
    print(f"Mismatch found in {col_key} -> {col_value} mapping:")
    print(mismatched_keys)
  return mismatched_keys

def df_insert_key_map(df: pl.DataFrame, df_map: pl.DataFrame, after: str, default: str | None = None, force: bool = False):
  col_key = df_map.columns[0]
  col_value = df_map.columns[1]
  if default is None:
    default = col_default.get(col_value, f"todo!({col_value})")
  if not force and col_value in df.columns:
    return df
  df1 = df.select(col_key).join(df_map, how="full", on=col_key).select(col_value).fill_null(pl.lit(default))
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

def parse_rs_attr(code: str, line_starts: str = "#[cfg(", line_not_starts: list[str] = ["#[", "//"], blocking: tuple[str, str] | None = None):
  result = []
  starts = blocking is None
  indent_of_starts = 0
  attr = None
  for line in code.splitlines():
    if not starts:
      if line.strip().startswith(blocking[0]):
        indent_of_starts = len(line) - len(line.lstrip())
        starts = True
      continue
    elif line.strip().startswith(blocking[1]):
      if indent_of_starts == len(line) - len(line.lstrip()):
        break
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
attr_map = {
  '#[cfg(target_os = "windows")]': 'windows',
  # '#[cfg(target_os = "linux")]': 'linux',
  '#[cfg(all(unix, not(target_os = "macos")))]': 'linux',
  '#[cfg(target_os = "macos")]': 'macos',
  '#[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]': 'windows|linux'
}

# %%
df = load_csv(csv_filename)
save_csv(df, csv_filename)

with open(enigo__keycodes_rs) as rsfile:
  content = rsfile.read()
enigo_attrs = parse_rs_attr(content, blocking=("pub enum Key {", "}"))
map_enigo_attrs = {"Enigo::" + k.strip(","): "#[" + attr_map[v] + "]" for k, v in enigo_attrs if v}
df_map_enigo_attrs = pl.DataFrame({
  "enigo": map_enigo_attrs.keys(),
  "enigo_attr": map_enigo_attrs.values(),
})

df = df_fill_na(df.join(df_map_enigo_attrs.select('enigo'), on="enigo", coalesce=True, how="full", maintain_order="left_right"))
df = df_insert_key_map(df, df_map_enigo_attrs, after="enigo", force=True)
df_check_key_map(df, df_map_enigo_attrs)
df

# %%
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
  try: return f"CGKeyCode::from({int(v)})"
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
df

# %%
save_csv(df, csv_filename)

# %%
