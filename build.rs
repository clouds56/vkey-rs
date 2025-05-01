use std::{borrow::Cow, path::Path};

use cfg_aliases::cfg_aliases;

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

const CONVERT_GENERAL: &str = r##"
pub fn {from}_to_{to}(value: {From}) -> Option<{To}> {
  let result = match value {
    #( $key => {prefix}$value{suffix}, )#
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<{From}, {To}> for crate::convert::Converter {
  fn convert(value: {From}) -> Option<{To}> {
    {from}_to_{to}(value)
  }
}
"##;

fn gen_template(template: &str, from: &str, to: &str, prefix: Option<&str>, suffix: Option<&str>) -> String {
  template.replace("{from}", &from.to_lowercase()).replace("{to}", &to.to_lowercase())
    .replace("{From}", from).replace("{To}", to)
    .replace("{prefix}", prefix.unwrap_or_default()).replace("{suffix}", suffix.unwrap_or_default())
}

fn gen_convert<S1: AsRef<str>, S2: AsRef<str>, I: IntoIterator<Item = (S1, S2)>>(template: &str, map: I) -> String {
  fn normalize(s: &str) -> Cow<str> {
    let s = s.trim_start();
    if s.contains('*') {
      s.rsplit('*').next().unwrap();
      return format!("{}{}", s.trim_end().trim_end_matches('*'), s.rsplit('*').next().unwrap()).into()
    }
    s.into()
  }
  let map = map.into_iter().collect::<Vec<_>>();
  // https://rustexp.lpil.uk/
  let generated_code = regex::Regex::new(r"\n([ \t]*)#\(([\s\S]*)\)#(\n?)").unwrap().replace_all(template, |caps: &regex::Captures| {
    let indent = &caps[1];
    let inner = caps[2].trim();
    let newline = &caps[3];
    let content = map.iter().map(|(key, value)| {
      inner.replace("$key", &normalize(key.as_ref())).replace("$value", &normalize(value.as_ref()))
    }).collect::<Vec<String>>().join(&format!("{newline}{indent}"));
    format!("{newline}{indent}{content}{newline}")
  });
  format!("// This file is auto-generated. Do not edit manually.\n\n{}", generated_code)
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
}

fn is_valid<S: AsRef<str>>(s: S) -> bool {
  let s = s.as_ref().trim();
  !s.is_empty() && !s.starts_with("n!") && !s.starts_with("na!") && !s.starts_with("todo!") && !s.starts_with("none!")
}
fn kv_is_valid<K: AsRef<str>, V: AsRef<str>>((k, v): &(K, V)) -> bool {
  is_valid(k) && is_valid(v) && !k.as_ref().trim().ends_with('*')
}

struct Gen(KeyType, KeyType);

impl Gen {
  pub fn build(self, csv: &[Line<&str>]) -> String {
    let from = self.0;
    let to = self.1;
    gen_convert(
      &gen_template(CONVERT_GENERAL, from.name(), to.name(), to.as_value_prefix(), to.as_value_suffix()),
      csv.iter().map(|i| (from.get_line(i), to.get_line(i))).filter(kv_is_valid)
    )
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

fn generate_rs() {
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
    let content = Gen(from, to).build(&csv);
    save_file(format!("{output_path}/{filename}"), content)
      .expect("Failed to write generated.rs");
  }
}

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATING_CONVERT");
  if std::env::var("CARGO_FEATURE_GENERATING_CONVERT").is_ok() {
    generate_rs();
  }

  cfg_aliases! {
    mirror_enigo: { feature = "mirror_enigo" },
    mirror_enigo_windows: { all(feature = "target_windows", feature = "mirror_enigo") },
    mirror_enigo_macos: { all(feature = "target_macos", feature = "mirror_enigo") },
    mirror_enigo_linux: { all(feature = "target_linux", feature = "mirror_enigo") },
    mirror_windows_vk: { feature = "mirror_windows_vk" },
    mirror_winput_vk: { feature = "mirror_winput_vk" },
    mirror_macos: { feature = "mirror_macos" },
    mirror_winit: { feature = "mirror_winit" },

    dep_enigo: { feature = "enigo" },
    dep_enigo_windows: { all(target_os = "windows", feature = "enigo") },
    dep_enigo_macos: { all(target_os = "macos", feature = "enigo") },
    dep_enigo_linux: { all(target_os = "linux", feature = "enigo") },
    dep_windows_vk: { all(target_os = "windows", feature = "windows") },
    dep_macos: { all(target_os = "macos", feature = "macos") },
    dep_linux: { all(target_os = "linux", feature = "linux") },
    dep_hut_03: { feature = "hut_03" },
    dep_hut_04: { feature = "hut_04" },
    dep_xkeysym: { feature = "xkeysym" },

    build_target_windows: { all(target_os = "windows") },
    build_target_macos: { all(target_os = "macos") },
    build_target_linux: { all(target_os = "linux") },

    for_windows: { all(feature = "target_windows") },
    for_macos: { all(feature = "target_macos") },
    for_linux: { all(feature = "target_linux") },
  }
}
