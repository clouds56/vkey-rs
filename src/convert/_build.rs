use std::{borrow::Cow, path::Path};

use askama::Template;


struct Line<S> {
  pub hut: S,
  pub hut_code: S,
  pub winput: S,
  pub vk: S,
  pub vk_code: S,
  pub win_make1code: S,
  pub enigo: S,
  pub enigo_attr: S,
  pub keysym: S,
  pub keysym_code: S,
  pub cg: S,
  pub cg_code: S,
}

impl<T> From<Vec<T>> for Line<T> {
  fn from(v: Vec<T>) -> Self {
    let mut v = v.into_iter();
    Self {
      hut: v.next().unwrap(),
      hut_code: v.next().unwrap(),
      winput: v.next().unwrap(),
      vk: v.next().unwrap(),
      vk_code: v.next().unwrap(),
      win_make1code: v.next().unwrap(),
      enigo: v.next().unwrap(),
      enigo_attr: v.next().unwrap(),
      keysym: v.next().unwrap(),
      keysym_code: v.next().unwrap(),
      cg: v.next().unwrap(),
      cg_code: v.next().unwrap(),
    }
  }
}

mod filters {
  use super::RustExpr;

  pub fn pad_right<T: std::fmt::Display>(s: T, _: &dyn askama::Values, len: usize) -> askama::Result<String> {
    let mut s = s.to_string();
    while s.len() < len {
      s.push(' ');
    }
    Ok(s)
  }

  pub fn hex<T: std::fmt::Display + std::fmt::UpperHex>(s: T, _: &dyn askama::Values) -> askama::Result<String> {
    Ok(format!("0x{:02X}", s))
  }

  pub fn apply_expr<T: std::fmt::Display>(s: T, _: &dyn askama::Values, expr: &RustExpr) -> askama::Result<String> {
    Ok(expr.apply(&s.to_string()))
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
    {% if let Some(attr) = entry.attr_v -%}
    {{ attr | indent(4) }}
    {% endif -%}
    {{ entry.k | pad_right(*k_len) | apply_expr(key_expr) }} => {{ entry.v | pad_right(*v_len) | apply_expr(value_expr) }},
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
pub struct ConvertImplTemplate<'a> {
  pub from: &'a str,
  pub to: &'a str,
  pub key_expr: RustExpr,
  pub value_expr: RustExpr,
  pub entries: Vec<Entry<'a, Cow<'a, str>>>,
  pub k_len: usize,
  pub v_len: usize,
}

impl<'a> ConvertImplTemplate<'a> {
  pub fn create(from: &'a str, to: &'a str, key_expr: Option<RustExpr>, value_expr: Option<RustExpr>) -> Self {
    let value_expr = value_expr.unwrap_or_else(RustExpr::default);
    let key_expr = key_expr.unwrap_or_else(RustExpr::default);
    Self {
      from,
      to,
      key_expr,
      value_expr,
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
pub struct ConvertImportEntry {
  pub gate: &'static str,
  pub import: &'static str,
}

impl ConvertImportEntry {
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
pub struct ConvertImport2Template {
  pub filename: String,
  pub from: &'static str,
  pub to: &'static str,
  pub from_source: &'static str,
  pub to_source: &'static str,
  pub from_imports: Vec<ConvertImportEntry>,
  pub to_imports: Vec<ConvertImportEntry>,
}

#[derive(Debug, Clone, askama::Template)]
#[template(ext = "txt", source = r#"
{% for f in from_imports -%}
{% let i = loop.index %}
  #[cfg({{f.gate}})]
  mod {{from_source|lower}}_{{i}} {
    {{f.import|indent(4)}}
    include!("{{filename}}");
  }
{% endfor -%}
"#)]
pub struct ConvertImport1Template {
  pub filename: String,
  pub from: &'static str,
  pub from_source: &'static str,
  pub from_imports: Vec<ConvertImportEntry>,
}

#[derive(Debug, Clone, askama::Template)]
#[template(ext = "txt", source = r#"// This file is auto-generated. Do not edit manually.


pub fn {{ty|lower}}_to_{{code.ty|lower}}(value: &{{ty}}) -> {{code.ty}} {
  {%- if code.can_const_check %}
  #[allow(unused_parens)]
  const {
    {%- for entry in entries %}
    assert!({{ ("(&{" ~ entry.0 | pad_right(*k_len) | apply_expr(value_expr) ~ "})") | apply_expr(code.unwrap_expr) }} == {{entry.1 | hex}});
    {%- endfor %}
  }
  {%- endif %}
  {{ "value" | apply_expr(code.unwrap_expr) }}
}

impl crate::numeric::AsCode<{{ty}}> for crate::numeric::Coder {
  type Code = {{code.ty}};
  fn as_code(value: &{{ty}}) -> Self::Code {
    {{ty|lower}}_to_{{code.ty|lower}}(value)
  }
  #[allow(unreachable_patterns)]
  fn from_code(code: Self::Code) -> Option<{{ty}}> {
    match code {
      {%- for entry in entries|unique %}
      {{entry.1 | hex}} => Some({{ entry.0 | pad_right(*k_len) | apply_expr(value_expr) }}),
      {%- endfor %}
      _ => None,
    }
  }
  {%- if let Some(wrap_expr) = code.wrap_expr %}
  unsafe fn from_code_unchecked(code: Self::Code) -> {{ty}} {
    {{ "code" | apply_expr(wrap_expr) }}
  }
  {%- endif %}
}

{% if !code.can_const_check -%}
#[test]
#[allow(unused_parens)]
fn test_code() {
  {%- for entry in entries %}
  assert!({{ ("(&{" ~ entry.0 | pad_right(*k_len) | apply_expr(value_expr) ~ "})") | apply_expr(code.unwrap_expr) }} == {{entry.1 | hex}});
  {%- endfor %}
}
{% endif -%}
"#)]
pub struct AsCodeImplTemplate<'a> {
  pub ty: &'a str,
  pub code: KeyCodeInfo,
  pub value_expr: RustExpr,
  pub k_len: usize,
  pub entries: Vec<(Cow<'a, str>, u64)>,
}
impl<'a> AsCodeImplTemplate<'a> {
  pub fn create(from: &'a str, to: KeyCodeInfo, value_expr: Option<RustExpr>) -> Self {
    let value_expr = value_expr.unwrap_or_else(RustExpr::default);
    Self {
      ty: from,
      code: to,
      value_expr,
      entries: vec![],
      k_len: 0,
    }
  }

  pub fn build(self, map: Vec<(Cow<'a, str>, u64)>) -> String {
    Self {
      k_len: map.iter().map(|e| e.0.len()).max().unwrap_or(0),
      entries: map,
      ..self
    }.render().unwrap()
  }
}

#[expect(unused)]
#[derive(Debug, Clone, Copy)]
enum KeyType {
  HUT, Winput, WinVk, WinScan, Enigo, Keysym, CG,
  EnigoDep, EnigoMirror,
}

impl KeyType {
  pub fn name(self) -> &'static str {
    const {
      assert!(true);
    }
    match self {
      KeyType::HUT => "Usage",
      KeyType::Winput => "Vk",
      KeyType::WinVk => "VIRTUAL_KEY",
      KeyType::WinScan => "Make1Code",
      KeyType::Enigo | KeyType::EnigoDep | KeyType::EnigoMirror => "Enigo",
      KeyType::Keysym => "Keysym",
      KeyType::CG => "CGKeyCode",
    }
  }

  pub fn import_mirror(self) -> Vec<ConvertImportEntry> {
    match self {
      KeyType::Winput => vec![
        ConvertImportEntry::new("mirror_winput_vk", "use crate::mirror::winput::Vk;"),
      ],
      KeyType::WinVk => vec![
        ConvertImportEntry::new("mirror_windows_vk", "use crate::mirror::windows::{self as keys, VIRTUAL_KEY};"),
      ],
      KeyType::EnigoMirror => vec![
        ConvertImportEntry::new("mirror_enigo", "use crate::mirror::enigo::Key as Enigo;"),
      ],
      KeyType::Keysym => vec![],
      KeyType::CG => vec![
        ConvertImportEntry::new("any(dep_macos, mirror_macos)", "#[cfg(dep_macos)]\nuse crate::deps::macos::KeyCode;\n#[cfg(not(dep_macos))]\n#[cfg(mirror_macos)]\nuse crate::mirror::macos::KeyCode;\nuse crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};"),
      ],
      _ => return vec![],
    }
  }

  pub fn import_dep(self) -> Vec<ConvertImportEntry> {
    match self {
      KeyType::HUT => vec![
        ConvertImportEntry::new("dep_hut_04", "use crate::deps::hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};"),
        ConvertImportEntry::new("dep_hut_03", "use crate::deps::hut_03::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};"),
      ],
      KeyType::Winput => vec![],
      KeyType::WinVk => vec![
        ConvertImportEntry::new("dep_windows_vk", "use crate::deps::windows::{self as keys, VIRTUAL_KEY};"),
      ],
      KeyType::WinScan => vec![
        ConvertImportEntry::new("dep_make1", "use crate::mirror::make1::Make1Code;"),
      ],
      KeyType::EnigoDep => vec![ConvertImportEntry::new("dep_enigo", "use crate::deps::enigo::Key as Enigo;")],
      KeyType::Keysym => vec![
        ConvertImportEntry::new("dep_xkeysym", "use crate::deps::xkeysym::Keysym;"),
      ],
      _ => return vec![],
    }
  }

  pub fn get_line<'a>(self, line: &'a Line<&'a str>) -> &'a str {
    match self {
      KeyType::HUT => line.hut,
      KeyType::Winput => line.winput,
      KeyType::WinVk => line.vk,
      KeyType::WinScan => line.win_make1code,
      KeyType::Enigo | KeyType::EnigoDep | KeyType::EnigoMirror => line.enigo,
      KeyType::Keysym => line.keysym,
      KeyType::CG => line.cg,
    }
  }

  pub fn get_attr<'a>(self, line: &'a Line<&'a str>) -> Option<&'a str> {
    match self {
      KeyType::Enigo | KeyType::EnigoDep | KeyType::EnigoMirror => Some(line.enigo_attr),
      _ => None,
    }
  }

  pub fn get_code<'a>(self, line: &'a Line<&'a str>) -> Option<&'a str> {
    match self {
      KeyType::HUT => line.hut_code,
      KeyType::Winput => line.vk_code,
      KeyType::WinVk => line.vk_code,
      KeyType::WinScan => line.win_make1code,
      KeyType::Keysym => line.keysym_code,
      KeyType::CG => line.cg_code,
      _ => return None,
    }.into()
  }

  pub fn as_key_expr(self) -> Option<RustExpr> {
    match (self, self.as_value_expr()) {
      (KeyType::HUT, Some(v)) | (KeyType::WinVk, Some(v)) => Some(RustExpr::new(format!("k if k == {}", v.0))),
      (_, v) => v,
    }
  }

  pub fn as_value_expr(self) -> Option<RustExpr> {
    match self {
      KeyType::HUT => Some(RustExpr::new("{}.usage()")),
      KeyType::WinVk => Some(RustExpr::new("keys::{}")),
      KeyType::CG => Some(RustExpr::new("CGKeyCode( {} )")),
      KeyType::WinScan => Some(RustExpr::new("Make1Code( {} )")),
      _ => None,
    }
  }

  pub fn as_code_info(self) -> Option<KeyCodeInfo> {
    self.try_into().ok()
  }

  pub fn is_valid<S: AsRef<str>>(self, s: S) -> bool {
    let s = s.as_ref().trim();
    !s.is_empty() && !s.starts_with("n!") && !s.starts_with("na!") && !s.starts_with("todo!") && !s.starts_with("none!")
  }

  pub fn parse_content_unchecked<'a>(self, s: &'a str) -> Cow<'a, str> {
    let s = s.trim_start();
    Cow::Borrowed(s.trim().trim_end_matches("*"))
  }

  pub fn parse_code_unchecked<'a>(self, s: &'a str) -> u64 {
    let s = s.trim_start();
    let s = s.trim().trim_end_matches("*");
    if s.is_empty() {
      return 0;
    }
    if s.starts_with("0x") {
      u64::from_str_radix(&s[2..], 16).unwrap()
    } else {
      s.parse::<u64>().unwrap()
    }
  }

  pub fn get_attr_unchecked<'a>(self, s: &'a str) -> Option<Cow<'a, str>> {
    match self {
      KeyType::EnigoDep => build_attr_for_target_os(s),
      KeyType::EnigoMirror => build_attr_for_for_target(s),
      _ => None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct RustExpr(pub String);
impl RustExpr {
  pub fn new<S: ToString>(s: S) -> Self {
    Self(s.to_string())
  }
  pub fn default() -> Self {
    Self::new("{}")
  }
  pub fn apply(&self, s: &str) -> String {
    // let (prefix, suffix) = split_expr(self.0);
    // format!("{}{}{}", prefix, s, suffix)
    self.0.replace("{}", s)
  }
}

#[derive(Debug, Clone)]
pub struct KeyCodeInfo {
  pub ty: &'static str,
  pub can_const_check: bool,
  pub wrap_expr: Option<RustExpr>,
  pub unwrap_expr: RustExpr,
}

impl KeyCodeInfo {
  /// `from == ""` means None
  pub fn new(ty: &'static str, wrap: &'static str, unwrap: &'static str) -> Self {
    assert_ne!(unwrap, "");
    Self {
      ty,
      can_const_check: true,
      wrap_expr: if wrap.is_empty() { None } else { Some(RustExpr::new(wrap)) },
      unwrap_expr: RustExpr::new(unwrap),
    }
  }
  pub fn set_can_const_check(mut self, can_const_check: bool) -> Self {
    self.can_const_check = can_const_check;
    self
  }
}

impl TryFrom<KeyType> for KeyCodeInfo {
  type Error = ();

  fn try_from(ty: KeyType) -> Result<Self, Self::Error> {
    match ty {
      KeyType::HUT => Ok(KeyCodeInfo::new("u32", "", "{}.usage_value()").set_can_const_check(false)),
      KeyType::Winput => Ok(KeyCodeInfo::new("u8", "unsafe { std::mem::transmute({}) }", "*{} as u8")),
      KeyType::WinVk => Ok(KeyCodeInfo::new("u16", "VIRTUAL_KEY({})", "{}.0")),
      KeyType::WinScan => Ok(KeyCodeInfo::new("u32", "Make1Code({})", "{}.0")),
      KeyType::Keysym => Ok(KeyCodeInfo::new("u32", "Keysym::new({})", "{}.raw()")),
      KeyType::CG => Ok(KeyCodeInfo::new("u16", "CGKeyCode({})", "{}.0")),
      _ => Err(()),
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
      k: from.parse_content_unchecked(k),
      v: to.parse_content_unchecked(v),
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

  pub fn build_convert_impl<'a>(self, csv: &'a [Line<&'a str>]) -> String {
    let from = self.0;
    let to = self.1;
    let map = self.build_kv(csv);
    ConvertImplTemplate::create(from.name(), to.name(), from.as_key_expr(), to.as_value_expr())
      .build(map)
  }

  pub fn build_imports1(self, filename: &str) -> String {
    let from = self.0;
    let content = vec![
      ConvertImport1Template {
        filename: filename.to_string(),
        from: from.name(),
        from_source: "mirror",
        from_imports: from.import_mirror(),
      }.render().unwrap(),
      ConvertImport1Template {
        filename: filename.to_string(),
        from: from.name(),
        from_source: "dep",
        from_imports: from.import_dep(),
      }.render().unwrap(),
    ].into_iter().filter(|i| !i.trim().is_empty()).map(|i| i.trim_end().trim_start_matches('\n').to_string()).collect::<Vec<_>>().join("\n");
    if content.trim().is_empty() {
      return String::new();
    }
    format!("#[allow(unused_imports)]\nmod generated_{} {{\n  {}\n}}\n\n", format!("{:?}", from).to_lowercase(), content.trim())
  }

  pub fn build_imports2(self, filename: &str) -> String {
    let (from, to) = (self.0, self.1);
    let content = vec![
      ConvertImport2Template {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "mirror",
        to_source: "mirror",
        from_imports: from.import_mirror(),
        to_imports: to.import_mirror(),
      }.render().unwrap(),
      ConvertImport2Template {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "mirror",
        to_source: "dep",
        from_imports: from.import_mirror(),
        to_imports: to.import_dep(),
      }.render().unwrap(),
      ConvertImport2Template {
        filename: filename.to_string(),
        from: from.name(),
        to: to.name(),
        from_source: "dep",
        to_source: "mirror",
        from_imports: from.import_dep(),
        to_imports: to.import_mirror(),
      }.render().unwrap(),
      ConvertImport2Template {
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

  pub fn build_as_code(self, csv: &[Line<&str>]) -> String {
    let from = self.0;
    let to = self.1;
    let Some(code_info) = to.as_code_info() else {
      return String::new();
    };
    let map = csv.iter().filter_map(|line| {
      let k = from.get_line(line);
      let v = to.get_code(line).unwrap();
      if !self.kv_is_valid(&(k, v)) {
        return None;
      }
      Some((from.parse_content_unchecked(k), to.parse_code_unchecked(v)))
    }).collect::<Vec<_>>();
    AsCodeImplTemplate::create(from.name(), code_info, from.as_value_expr())
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

const INDEX_MOD_HEADER: &str = r#"
// This file is auto-generated. Do not edit manually.
#![allow(unused_imports)]

"#;

pub fn main() {
  if std::env::var("DOCS_RS").is_ok() {
    return;
  }
  let csv_path = "src/convert/convert.csv";
  let output_path = "src/convert";
  let output_path2 = "src/numeric";

  println!("cargo:rerun-if-changed={csv_path}");

  let csv_content = std::fs::read_to_string(csv_path).expect("Failed to read convert.csv");
  let csv = csv_content.lines().filter(|i| !i.trim().is_empty() && !i.trim().starts_with(";"))
      .map(|i| i.split(',').collect::<Vec<_>>().into()).collect::<Vec<Line<_>>>();

  let mut index_mod = INDEX_MOD_HEADER.trim_start().to_string();
  for (from, to) in [
    (KeyType::HUT, KeyType::Winput),

    (KeyType::Winput, KeyType::HUT),
    (KeyType::Winput, KeyType::EnigoMirror),
    (KeyType::Winput, KeyType::EnigoDep),

    (KeyType::Winput, KeyType::CG),
    (KeyType::Winput, KeyType::Keysym),

    (KeyType::Winput, KeyType::WinScan),
    (KeyType::WinVk, KeyType::WinScan),

    (KeyType::EnigoMirror, KeyType::Winput),
    (KeyType::EnigoDep, KeyType::Winput),
    (KeyType::EnigoMirror, KeyType::WinVk),
    (KeyType::EnigoDep, KeyType::WinVk),
    (KeyType::EnigoMirror, KeyType::Keysym),
    (KeyType::EnigoDep, KeyType::Keysym),
    (KeyType::EnigoMirror, KeyType::CG),
    (KeyType::EnigoDep, KeyType::CG),
  ] {
    let filename = format!("generated.{from:?}_to_{to:?}.rs");
    let content = Gen(from, to).build_convert_impl(&csv);
    save_file(format!("{output_path}/{filename}"), content)
      .expect("Failed to write generated.rs");

    let imports_str = Gen(from, to).build_imports2(&filename);
    index_mod.push_str(&imports_str);
  }
  save_file(format!("{output_path}/generated._index.rs"), index_mod).expect("failed to write index.rs");

  let mut index_mod = INDEX_MOD_HEADER.trim_start().to_string();
  for ty in [KeyType::HUT, KeyType::Winput, KeyType::WinVk, KeyType::WinScan, KeyType::Keysym, KeyType::CG] {
    let filename = format!("generated.{ty:?}.rs");
    let content = Gen(ty, ty).build_as_code(&csv);
    save_file(format!("{output_path2}/{filename}"), content)
      .expect("Failed to write generated.rs");

    let imports_str = Gen(ty, ty).build_imports1(&filename);
    index_mod.push_str(&imports_str);
  }
  save_file(format!("{output_path2}/generated._index.rs"), index_mod).expect("failed to write index.rs");
}
