use cfg_aliases::cfg_aliases;

#[cfg(feature = "generating_convert")]
#[path = "src/convert/_build.rs"]
pub mod generate_rs;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_GENERATING_CONVERT");

  #[cfg(feature = "generating_convert")]
  generate_rs::main();

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
    dep_make1: { feature = "make1" },
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
