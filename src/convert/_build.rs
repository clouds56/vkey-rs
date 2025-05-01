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

#[derive(Debug, Clone, askama::Template)]
#[template(ext = "txt", source = r#"// This file is auto-generated. Do not edit manually.


pub fn {{from | lower}}_to_{{to | lower}}(value: {{from}}) -> Option<{{to}}> {
  let result = match value {
    {% for (k, v) in map -%}
      {{ k | pad_right(*k_len) }} => {{prefix}}{{ v | pad_right(*v_len) }}{{suffix}},
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
  pub map: Vec<(Cow<'a, str>, Cow<'a, str>)>,
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
      map: vec![],
      k_len: 0,
      v_len: 0,
    }
  }

  pub fn build(self, map: Vec<(Cow<'a, str>, Cow<'a, str>)>) -> String {
    Self {
      k_len: map.iter().map(|(k, _)| k.len()).max().unwrap_or(0),
      v_len: map.iter().map(|(_, v)| v.len()).max().unwrap_or(0),
      map,
      ..self
    }.render().unwrap()
  }
}

#[expect(unused)]
#[derive(Debug, Clone, Copy)]
enum KeyType {
  HUT, Winput, WinVk, VkValue, HutKeyboardValue, Enigo, KeySym, CG,
}

impl KeyType {
  pub fn name(self) -> &'static str {
    match self {
      KeyType::HUT => "Usage",
      KeyType::Winput => "Vk",
      KeyType::WinVk => "Vk",
      KeyType::VkValue => unimplemented!(),
      KeyType::HutKeyboardValue => unimplemented!(),
      KeyType::Enigo => "Enigo",
      KeyType::KeySym => "KeySym",
      KeyType::CG => "CGKeyCode",
    }
  }

  pub fn get_line<'a>(self, line: &'a Line<&'a str>) -> &'a str {
    match self {
      KeyType::HUT => line.hut,
      KeyType::Winput => line.winput,
      KeyType::WinVk => line.vk,
      KeyType::VkValue => line.vk_value,
      KeyType::HutKeyboardValue => line.hut_keyboard_value,
      KeyType::Enigo => line.enigo,
      KeyType::KeySym => line.keysym,
      KeyType::CG => line.cg,
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
}


#[derive(Debug, Clone, Copy)]
struct Gen(KeyType, KeyType);

impl Gen {
  pub fn build_kv<'a>(self, csv: &'a [Line<&'a str>]) -> Vec<(Cow<'a, str>, Cow<'a, str>)> {
    let from = self.0;
    let to = self.1;
    csv.iter().map(|i| (from.get_line(i), to.get_line(i))).filter(|t| self.kv_is_valid(t)).map(|(k, v)| (from.get_content_unchecked(k), to.get_content_unchecked(v))).collect()
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

  for tuple in [
    (KeyType::Enigo, KeyType::WinVk),
    (KeyType::Enigo, KeyType::Winput),
    (KeyType::Winput, KeyType::HUT),
    (KeyType::Winput, KeyType::Enigo),
    (KeyType::Winput, KeyType::CG),
  ] {
    let (from, to) = tuple;
    let filename = format!("generated.{from:?}_to_{to:?}.rs");
    let content = Gen(from, to).build_general(&csv);
    save_file(format!("{output_path}/{filename}"), content)
      .expect("Failed to write generated.rs");
  }
}
