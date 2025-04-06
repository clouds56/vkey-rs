# %%
from pathlib import Path
from dataclasses import dataclass
import csv

workspace_dir = Path(__file__).parent.parent
csv_filename = workspace_dir / 'src/keycodes/convert/convert.csv'

@dataclass
class Row:
  hut:        str = " todo!(HUT)"
  winput:     str = " na!(Vk)"
  vk:         str = " todo!(VK_)"
  vk_value:   str = " n!()"
  hut_value:  str = " n!()"
  enigo:      str = " todo!(Enigo)"
  keysym:     str = " todo!(Keysym)"
  cg:         str = " todo!(CG)"
  __empty:    str = " empty!()"

def read_csv(filename = csv_filename):
  with open(filename) as csvfile:
    reader = csv.reader(csvfile)
    keycodes = [Row(*row) if len(row) == len(Row.__dataclass_fields__) else Row(*row[:-1]) for row in reader]
  return keycodes

def content_of_field(field: str):
  return field.strip().strip('*')

def normalize_field(field: str):
  field = field.strip()
  return field if field.startswith("*") else " " + field

def normalize_field_max_len(field: str, max_len: int):
  field = normalize_field(field)
  if max_len < 10:
    return " " + field + " " * (max_len - len(field) - 1)
  return field + " " * (max_len - len(field))

def format_csv(rows):
  has_new_field = any(row._Row__empty.strip() for row in rows)
  if has_new_field:
    for row in rows:
      if not row._Row__empty.strip():
        row._Row__empty = " empty!()"

  field_max_len = [
    max(len(normalize_field(getattr(row, field))) for row in rows) for field in Row.__dataclass_fields__.keys()
  ]
  field_max_len = [int(k * 1.1 + 2) if k > 10 else k+3 for k in field_max_len]
  for row in rows:
    for (field, max_len) in zip(Row.__dataclass_fields__.keys(), field_max_len):
      setattr(row, field, normalize_field_max_len(getattr(row, field), max_len))

def write_csv(rows, filename = csv_filename):
  filename = str(filename) + ".new"
  format_csv(rows)
  has_new_field = any(row._Row__empty.strip() for row in rows)
  with open(filename, mode='w', newline='\n') as csvfile:
    writer = csv.writer(csvfile)
    for row in rows:
      writer.writerow([getattr(row, field) for field in Row.__dataclass_fields__.keys()] + [""] * has_new_field)

def parse_rs_match(code: str, starts_with: str = ""):
  result = []
  for line in code.splitlines():
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

def fill_col(rows: list[Row], data: dict[str, str], key_col: str, value_col: str = "_Row__empty", default_value: str = "empty!()"):
  visited = set()
  for row in rows:
    key = content_of_field(getattr(row, key_col))
    old_value = content_of_field(getattr(row, value_col))
    value = data.get(key)
    if value:
      if old_value and not old_value.startswith("todo!"):
        assert content_of_field(value) == old_value, f"Conflict: {key} {old_value} {value}"
      setattr(row, value_col, value)
    elif not old_value:
      setattr(row, value_col, default_value)
    visited.add(key)
  new_keys = sorted(set(data.keys()) - visited)
  for key in new_keys:
    if '(' in key: continue
    new_row = Row()
    setattr(new_row, key_col, key)
    setattr(new_row, value_col, data[key])
    rows.append(new_row)
  return new_keys

# %%
keycodes = read_csv()
write_csv(keycodes)

# %%
keycodes = read_csv()
with open(workspace_dir / 'src/keycodes/enigo/windows.rs') as rsfile:
  data = parse_rs_match(rsfile.read(), "Key::")

data = {k.replace("Key::", "Enigo::"):v for k,v in data if "Key::" in k}
new_keys = fill_col(keycodes, data, "enigo", "vk")
write_csv(keycodes)
new_keys

# %%
keycodes = read_csv()
with open(workspace_dir / 'src/keycodes/enigo/linux.rs') as rsfile:
  data = parse_rs_match(rsfile.read(), "Key::")

data = {k.replace("Key::", "Enigo::"):v for k,v in data if "Key::" in k}
data |= {f"Enigo::Num{i}": f"Keysym::_{i}" for i in range(10)} | {f"Enigo::{chr(X)}": f"Keysym::{chr(X)}" for X in range(ord('A'), ord('Z')+1)}
new_keys = fill_col(keycodes, data, "enigo", "keysym")
write_csv(keycodes)
new_keys

# %%
# https://stackoverflow.com/questions/3202629/where-can-i-find-a-list-of-mac-virtual-key-codes/16125341#16125341
keycodes = read_csv()
with open(workspace_dir / 'src/keycodes/enigo/macos.rs') as rsfile:
  data = parse_rs_match(rsfile.read(), "Key::")

def normalize_v(v: str):
  try:     return f"KeyCode::from({int(v)})"
  except:  return v
data = {k.replace("Key::", "Enigo::"): normalize_v(v) for k,v in data if "Key::" in k}
data |= {f"Enigo::Num{i}": f"KeyCodeExt::kVK_ANSI_{i}" for i in range(10)} | {f"Enigo::{chr(X)}": f"KeyCodeExt::kVK_ANSI_{chr(X)}" for X in range(ord('A'), ord('Z')+1)}
new_keys = fill_col(keycodes, data, "enigo", "cg")
write_csv(keycodes)
new_keys

# %%
