use std::{borrow::Cow, path::Path};

use askama::Template;


struct Line<S> {
  pub hut: S,
  pub winput: S,
  pub vk: S,
  pub vk_value: S,
  pub hut_keyboard_value: S,
  pub enigo: S,
  pub enigo_attr: S,
  pub keysym: S,
  pub cg: S,
}

impl<T> From<Vec<T>> for Line<T> {
  fn from(v: Vec<T>) -> Self {
    let mut v = v.into_iter();
    Self {
      hut: v.next().unwrap(),
      winput: v.next().unwrap(),
      vk: v.next().unwrap(),
      vk_value: v.next().unwrap(),
      hut_keyboard_value: v.next().unwrap(),
      enigo: v.next().unwrap(),
      enigo_attr: v.next().unwrap(),
      keysym: v.next().unwrap(),
      cg: v.next().unwrap(),
    }
  }
}

mod filters {
  pub fn pad_right<T: std::fmt::Display>(s: T, _: &dyn askama::Values, len: usize) -> askama::Result<String> {
    let mut s = s.to_string();
    while s.len() < len {
      s.push(' ');
    }
    Ok(s)
  }
}

#[derive(Debug, Clone)]
pub struct Entry<'a, S: 'a> {
  pub k: S,
  pub v: S,
  pub attr_k: Option<S>,
  pub attr_v: Option<S>,
  marked: std::marker::PhantomData<&'a ()>,
}

#[derive(Debug, Clone, askama::Template)]
#[template(ext = "txt", source = r#"// This file is auto-generated. Do not edit manually.


pub fn {{from | lower}}_to_{{to | lower}}(value: {{from}}) -> Option<{{to}}> {
  let result = match value {
    {% for entry in entries -%}
    {% if let Some(attr) = entry.attr_k -%}
    {{ attr | indent(4) }}
    {% endif -%}
    {{ entry.k | pad_right(*k_len) }} => {{prefix}}{{ entry.v | pad_right(*v_len) }}{{suffix}},
    {% endfor -%}
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<{{from}}, {{to}}> for crate::convert::Converter {
  fn convert(value: {{from}}) -> Option<{{to}}> {
    {{from|lower}}_to_{{to|lower}}(value)
  }
}
"#)]
pub struct GeneralTemplate<'a> {
  pub from: &'a str,
  pub to: &'a str,
  pub prefix: &'a str,
  pub suffix: &'a str,
  pub entries: Vec<Entry<'a, Cow<'a, str>>>,
  pub k_len: usize,
  pub v_len: usize,
}

impl<'a> GeneralTemplate<'a> {
  pub fn create(from: &'a str, to: &'a str, prefix: Option<&'a str>, suffix: Option<&'a str>) -> Self {
    let prefix = prefix.unwrap_or("");
    let suffix = suffix.unwrap_or("");
    Self {
      from,
      to,
      prefix,
      suffix,
      entries: vec![],
      k_len: 0,
      v_len: 0,
    }
  }

  pub fn build(self, map: Vec<Entry<'a, Cow<'a, str>>>) -> String {
    Self {
      k_len: map.iter().map(|e| e.k.len()).max().unwrap_or(0),
      v_len: map.iter().map(|e| e.v.len()).max().unwrap_or(0),
      entries: map,
      ..self
    }.render().unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct ImportEntry {
  pub gate: &'static str,
  pub import: &'static str,
}

impl ImportEntry {
  pub fn new(gate: &'static str, import: &'static str) -> Self {
    Self { gate, import }
  }
}

#[derive(Debug, Clone, askama::Template)]
#[template(ext = "txt", source = r#"
{% for f in from_imports -%}
{% let i = loop.index -%}
{% for t in to_imports -%}
{% let j = loop.index %}
  #[cfg(all({{f.gate}}, {{t.gate}}))]
  mod {{from_source|lower}}_to_{{to_source|lower}}_{{i}}_{{j}} {
    {{f.import|indent(4)}}
    {{t.import|indent(4)}}
    include!("{{filename}}");
  }
{% endfor -%}
{% endfor -%}
"#)]
pub struct ImportTemplate {
  pub filename: String,
  pub from: &'static str,
  pub to: &'static str,
  pub from_source: &'static str,
  pub to_source: &'static str,
  pub from_imports: Vec<ImportEntry>,
  pub to_imports: Vec<ImportEntry>,
}

#[expect(unused)]
#[derive(Debug, Clone, Copy)]
enum KeyType {
  HUT, Winput, WinVk, VkValue, HutKeyboardValue, Enigo, KeySym, CG,
  EnigoDep, EnigoMirror,
}

impl KeyType {
  pub fn name(self) -> &'static str {
    match self {
      KeyType::HUT => "Usage",
      KeyType::Winput => "Vk",
      KeyType::WinVk => "Vk",
      KeyType::VkValue => unimplemented!(),
      KeyType::HutKeyboardValue => unimplemented!(),
      KeyType::Enigo | KeyType::EnigoDep | KeyType::EnigoMirror => "Enigo",
      KeyType::KeySym => "KeySym",
      KeyType::CG => "CGKeyCode",
    }
  }

  pub fn import_mirror(self) -> Vec<ImportEntry> {
    match self {
      KeyType::Winput => vec![
        ImportEntry::new("mirror_winput_vk", "use crate::mirror::winput::Vk;"),
      ],
      KeyType::WinVk => vec![
        ImportEntry::new("mirror_windows_vk", "use crate::mirror::windows::{self as keys, VIRTUAL_KEY as Vk};"),
      ],
      KeyType::EnigoMirror => vec![
        ImportEntry::new("mirror_enigo", "use crate::mirror::enigo::Key as Enigo;"),
      ],
      KeyType::KeySym => unimplemented!(),
      KeyType::CG => vec![
        ImportEntry::new("any(dep_macos, mirror_macos)", "#[cfg(dep_macos)]\nuse crate::deps::macos::KeyCode;\n#[cfg(not(dep_macos))]\n#[cfg(mirror_macos)]\nuse crate::mirror::macos::KeyCode;\nuse crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};"),
      ],
      _ => return vec![],
    }
  }

  pub fn import_dep(self) -> Vec<ImportEntry> {
    match self {
      KeyType::HUT => vec![
        ImportEntry::new("dep_hut_04", "use crate::deps::hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};"),
        ImportEntry::new("dep_hut_03", "use crate::deps::hut_03::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};"),
      ],
      KeyType::Winput => vec![],
      KeyType::WinVk => vec![
        ImportEntry::new("dep_windows_vk", "use crate::deps::windows::{self as keys, VIRTUAL_KEY as Vk};"),
      ],
      KeyType::EnigoDep => vec![ImportEntry::new("dep_enigo", "use crate::deps::enigo::Key as Enigo;")],
      KeyType::KeySym => unimplemented!(),
      _ => return vec![],
    }
  }

  pub fn get_line<'a>(self, line: &'a Line<&'a str>) -> &'a str {
    match self {
      KeyType::HUT => line.hut,
      KeyType::Winput => line.winput,
      KeyType::WinVk => line.vk,
      KeyType::VkValue => line.vk_value,
      KeyType::HutKeyboardValue => line.hut_keyboard_value,
      KeyType::Enigo | KeyType::EnigoDep | KeyType::EnigoMirror => line.enigo,
      KeyType::KeySym => line.keysym,
      KeyType::CG => line.cg,
    }
  }

  pub fn get_attr<'a>(self, line: &'a Line<&'a str>) -> Option<&'a str> {
    match self {
      KeyType::Enigo | KeyType::EnigoDep | KeyType::EnigoMirror => Some(line.enigo_attr),
      _ => None,
    }
  }

  pub fn as_value_prefix(self) -> Option<&'static str> {
    match self {
      KeyType::WinVk => Some("keys::"),
      _ => None,
    }
  }

  pub fn as_value_suffix(self) -> Option<&'static str> {
    match self {
      KeyType::HUT => Some(".usage()"),
      KeyType::CG => Some(".into()"),
      _ => None
    }
  }

  pub fn is_valid<S: AsRef<str>>(self, s: S) -> bool {
    let s = s.as_ref().trim();
    !s.is_empty() && !s.starts_with("n!") && !s.starts_with("na!") && !s.starts_with("todo!") && !s.starts_with("none!")
  }

  pub fn get_content_unchecked<'a>(self, s: &'a str) -> Cow<'a, str> {
    let s = s.trim_start();
    Cow::Borrowed(s.trim().trim_end_matches("*"))
  }

  pub fn get_attr_unchecked<'a>(self, s: &'a str) -> Option<Cow<'a, str>> {
    match self {
      KeyType::EnigoDep => build_attr_for_target_os(s),
      KeyType::EnigoMirror => build_attr_for_for_target(s),
      _ => None,
    }
  }
}


#[derive(Debug, Clone, Copy)]
struct Gen(KeyType, KeyType);

impl Gen {
  pub fn build_entry<'a>(self, line: &'a Line<&'a str>) -> Option<Entry<'a, Cow<'a, str>>> {
    let from = self.0;
    let to = self.1;
    let k = from.get_line(line);
    let v = to.get_line(line);
    if !self.kv_is_valid(&(k, v)) {
      return None;
    }
    let attr = from.get_attr(line);
    let attr2 = to.get_attr(line);
    Some(Entry {
      k: from.get_content_unchecked(k),
      v: to.get_content_unchecked(v),
      attr_k: attr.and_then(|i| from.get_attr_unchecked(i)),
      attr_v: attr2.and_then(|i| to.get_attr_unchecked(i)),
      marked: std::marker::PhantomData,
    })
  }

  pub fn build_kv<'a>(self, csv: &'a [Line<&'a str>]) -> Vec<Entry<'a, Cow<'a, str>>> {
    csv.iter().filter_map(|line| self.build_entry(line)).collect()
  }

  pub fn kv_is_valid<K: AsRef<str>, V: AsRef<str>>(self, (k, v): &(K, V)) -> bool {
    self.0.is_valid(k) && self.1.is_valid(v) && !k.as_ref().trim().ends_with('*')
  }

  pub fn build_general<'a>(self, csv: &'a [Line<&'a str>]) -> String {
    let from = self.0;
    let to = self.1;
    let map = self.build_kv(csv);
    GeneralTemplate::create(from.name(), to.name(), to.as_value_prefix(), to.as_value_suffix())
      .build(map)
  }

  pub fn build_imports(self, filename: &str) -> String {
    let (from, to) = (self.0, self.1);
    let content = vec![
      ImportTemplate {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "mirror",
        to_source: "mirror",
        from_imports: from.import_mirror(),
        to_imports: to.import_mirror(),
      }.render().unwrap(),
      ImportTemplate {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "mirror",
        to_source: "dep",
        from_imports: from.import_mirror(),
        to_imports: to.import_dep(),
      }.render().unwrap(),
      ImportTemplate {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "dep",
        to_source: "mirror",
        from_imports: from.import_dep(),
        to_imports: to.import_mirror(),
      }.render().unwrap(),
      ImportTemplate {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "dep",
        to_source: "dep",
        from_imports: from.import_dep(),
        to_imports: to.import_dep(),
      }.render().unwrap(),
    ].into_iter().filter(|i| !i.trim().is_empty()).map(|i| i.trim_end().trim_start_matches('\n').to_string()).collect::<Vec<_>>().join("\n");
    if content.trim().is_empty() {
      return String::new();
    }
    format!("mod generated_{}_to_{} {{\n  {}\n}}\n\n", format!("{:?}", from).to_lowercase(), format!("{:?}", to).to_lowercase(), content.trim())
  }
}

fn save_file<P: AsRef<Path>, S: AsRef<str>>(filename: P, content: S) -> std::io::Result<()> {
  let filename = filename.as_ref();
  let content = content.as_ref();
  let path = filename.to_path_buf();
  if content.is_empty() {
    return Err(std::io::Error::other("Content is empty"));
  }
  if path.exists() && !path.is_file() {
    return Err(std::io::Error::other(format!("{} is not a file", filename.display())));
  }
  if path.exists() && !content.is_empty() {
    let existing_content = std::fs::read_to_string(&path)?;
    if existing_content == content {
      return Ok(());
    }
  }
  std::fs::write(path, content)
}

fn build_attr_for_target_os<'a>(a: &'a str) -> Option<Cow<'a, str>> {
  let a = a.trim();
  if a.is_empty() {
    return None;
  }
  let a = a.trim_start_matches("#[").trim_end_matches("]");
  let aa = a.split('|').map(|i| format!("target_os = {:?}", i.trim())).collect::<Vec<_>>();
  if aa.len() == 1 {
    Some(Cow::Owned(format!("#[cfg({})]", aa[0])))
  } else {
    Some(Cow::Owned(format!("#[cfg(any({}))]", aa.join(", "))))
  }
}

fn build_attr_for_for_target<'a>(a: &'a str) -> Option<Cow<'a, str>> {
  let a = a.trim();
  if a.is_empty() {
    return None;
  }
  let a = a.trim_start_matches("#[").trim_end_matches("]");
  let aa = a.split('|').map(|i| format!("for_{}", i.trim())).collect::<Vec<_>>();
  if aa.len() == 1 {
    Some(Cow::Owned(format!("#[cfg({})]", aa[0])))
  } else {
    Some(Cow::Owned(format!("#[cfg(any({}))]", aa.join(", "))))
  }
}

pub fn main() {
  if std::env::var("DOCS_RS").is_ok() {
    return;
  }
  let csv_path = "src/convert/convert.csv";
  let output_path = "src/convert";

  println!("cargo:rerun-if-changed={csv_path}");

  let csv_content = std::fs::read_to_string(csv_path).expect("Failed to read convert.csv");
  let csv = csv_content.lines().filter(|i| !i.trim().is_empty() && !i.trim().starts_with(";"))
      .map(|i| i.split(',').collect::<Vec<_>>().into()).collect::<Vec<Line<_>>>();

  let mut index_mod = String::new();
  for tuple in [
    (KeyType::Winput, KeyType::HUT),
    (KeyType::Winput, KeyType::EnigoMirror),
    (KeyType::Winput, KeyType::EnigoDep),
    (KeyType::EnigoMirror, KeyType::Winput),
    (KeyType::EnigoDep, KeyType::Winput),
    (KeyType::EnigoMirror, KeyType::WinVk),
    (KeyType::EnigoDep, KeyType::WinVk),
    (KeyType::EnigoMirror, KeyType::CG),
    (KeyType::EnigoDep, KeyType::CG),
    (KeyType::Winput, KeyType::CG),
  ] {
    let (from, to) = tuple;
    let filename = format!("generated.{from:?}_to_{to:?}.rs");
    let content = Gen(from, to).build_general(&csv);
    save_file(format!("{output_path}/{filename}"), content)
      .expect("Failed to write generated.rs");

    let imports_str = Gen(from, to).build_imports(&filename);
    index_mod.push_str(&imports_str);
  }
  save_file(format!("{output_path}/generated._index.rs"), index_mod).expect("failed to write index.rs");
}
