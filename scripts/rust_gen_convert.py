# %%
from pathlib import Path
import polars as pl

workspace_dir = Path(__file__).parent.parent
csv_filename = workspace_dir / 'src/convert/convert.csv'
enigo__keycodes_rs = workspace_dir / 'scripts/cache' / 'raw.githubusercontent.com--enigo-rs--enigo--refs--heads--main--src--keycodes.rs'

# %%
col_map = {
  "hut":        ";hut::HUT",
  "winput":     "winput::Vk",
  "vk":         "windows::VIRTUAL_KEY",
  "vk_value":   "vkcode",
  "hut_value":  "hkcode",
  "enigo":      "enigo::Key",
  "keysym":     "xkeysym::Keysym",
  "cg":         "macos::KeyCode",
}
col_map_rev = {v: k for k, v in col_map.items()}
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
df = load_csv(csv_filename)
save_csv(df, csv_filename)
df

# %%
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
with open(enigo__keycodes_rs) as rsfile:
  content = rsfile.read()
data = parse_rs_match(content, blocking=('impl From<Key> for xkeysym::Keysym {', '}'))
key_map_enigo_keysym = {k.replace("Key::", "Enigo::"): v for k,v in data if "Key::" in k and "(" not in k}
data = parse_rs_match(content, blocking=('impl TryFrom<Key> for windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY {', '}'))
key_map_enigo_vk = {k.replace("Key::", "Enigo::"): v for k,v in data if "Key::" in k and "(" not in k}
