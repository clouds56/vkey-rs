use std::borrow::Cow;

struct Line<S> {
  pub hut: S,
  pub winput: S,
  pub vk: S,
  pub vk_value: S,
  pub hut_keyboard_value: S,
  pub enigo: S,
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

impl crate::keycodes::convert::Convert<{From}, {To}> for crate::keycodes::convert::Converter {
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

fn main() {
  let csv_path = "src/keycodes/convert/convert.csv";
  let output_path = "src/keycodes/convert/";

  let csv_content = std::fs::read_to_string(csv_path).expect("Failed to read convert.csv");
  let csv = csv_content.lines().filter(|i| !i.trim().is_empty())
      .map(|i| i.split(',').collect::<Vec<_>>().into()).collect::<Vec<Line<_>>>();

  fn is_valid<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref().trim();
    !s.is_empty() && !s.starts_with("n!") && !s.starts_with("na!") && !s.starts_with("todo!") && !s.starts_with("none!")
  }
  fn kv_is_valid<K: AsRef<str>, V: AsRef<str>>((k, v): &(K, V)) -> bool {
    is_valid(k) && is_valid(v) && !k.as_ref().trim().ends_with('*')
  }

  std::fs::write(
    format!("{output_path}generated.winput_to_hut.rs"),
    gen_convert(
      &gen_template(CONVERT_GENERAL, "Vk", "Usage", None, Some(".usage()")),
      csv.iter().map(|i| (i.winput, i.hut)).filter(kv_is_valid)
    )
  ).expect("Failed to write generated.rs");


  std::fs::write(
    format!("{output_path}generated.winput_to_enigo.rs"),
    gen_convert(
      &gen_template(CONVERT_GENERAL, "Vk", "Enigo", None, None),
      csv.iter().map(|i| (i.winput, i.enigo)).filter(kv_is_valid)
    )
  ).expect("Failed to write generated.rs");

  std::fs::write(
    format!("{output_path}generated.enigo_to_winput.rs"),
    gen_convert(
      &gen_template(CONVERT_GENERAL, "Enigo", "Vk", None, None),
      csv.iter().map(|i| (i.enigo, i.winput)).filter(kv_is_valid)
    )
  ).expect("Failed to write generated.rs");

  std::fs::write(
    format!("{output_path}generated.enigo_to_vk.rs"),
    gen_convert(
      &gen_template(CONVERT_GENERAL, "Enigo", "Vk", Some("keys::"), None),
      csv.iter().map(|i| (i.enigo, i.vk)).filter(kv_is_valid)
    )
  ).expect("Failed to write generated.rs");

  std::fs::write(
    format!("{output_path}generated.winput_to_macos.rs"),
    gen_convert(
      &gen_template(CONVERT_GENERAL, "Vk", "CGKeyCode", None, Some(".into()")),
      csv.iter().map(|i| (i.winput, i.cg)).filter(kv_is_valid)
    )
  ).expect("Failed to write generated.rs");

  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed={csv_path}");
}
