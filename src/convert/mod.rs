#![allow(deprecated)]

mod generated_winput_to_hut {
  #[cfg(all(feature = "mirror_winput_vk", feature = "hut_03"))]
  mod mirror_to_hut_03 {
    use crate::mirror::winput::Vk;
    use crate::deps::hut_03::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.Winput_to_HUT.rs");
  }

  #[cfg(all(feature = "mirror_winput_vk", feature = "hut_04"))]
  mod mirror_to_hut_04 {
    use crate::mirror::winput::Vk;
    use crate::deps::hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.Winput_to_HUT.rs");
  }
}

mod generated_winput_to_enigo {
  #[cfg(all(feature = "mirror_winput_vk", feature = "mirror_enigo"))]
  mod mirror_to_mirror {
    use crate::mirror::winput::Vk;
    use crate::mirror::enigo::Key as Enigo;
    include!("generated.Winput_to_Enigo.rs");
  }
  #[cfg(all(feature = "mirror_winput_vk", feature = "enigo", target_os = "windows"))]
  mod mirror_to_dep {
    use crate::mirror::winput::Vk;
    use crate::deps::enigo::Key as Enigo;
    include!("generated.Winput_to_Enigo.rs");
  }
}

mod generated_enigo_to_winput {
  #[cfg(all(feature = "mirror_enigo", feature = "mirror_winput_vk"))]
  mod mirror_to_mirror {
    use crate::mirror::enigo::Key as Enigo;
    use crate::mirror::winput::Vk;
    include!("generated.Enigo_to_Winput.rs");
  }
  #[cfg(all(feature = "enigo", feature = "mirror_winput_vk", target_os = "windows"))]
  mod dep_to_mirror {
    use crate::deps::enigo::Key as Enigo;
    use crate::mirror::winput::Vk;
    include!("generated.Enigo_to_Winput.rs");

  }
}

mod generated_enigo_to_vk {
  #[cfg(all(feature = "mirror_enigo", feature = "mirror_windows_vk"))]
  mod mirror_to_mirror {
    use crate::mirror::enigo::Key as Enigo;
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(feature = "enigo", feature = "mirror_windows_vk", target_os = "windows"))]
  mod dep_to_mirror {
    use crate::deps::enigo::Key as Enigo;
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(feature = "mirror_enigo", feature = "windows", target_os = "windows"))]
  mod mirror_to_dep {
    use crate::mirror::enigo::Key as Enigo;
    use crate::deps::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(feature = "enigo", feature = "windows", target_os = "windows"))]
  mod dep_to_dep {
    use crate::deps::enigo::Key as Enigo;
    use crate::deps::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
}

mod generated_winput_to_cg {
  #[cfg(any(feature = "macos", feature = "mirror_macos"))]
  mod mirror_to_any {
    use crate::mirror::winput::Vk;
    #[cfg(all(feature = "macos", target_os = "macos"))]
    use crate::deps::macos::KeyCode;
    #[cfg(not(all(feature = "macos", target_os = "macos")))]
    #[cfg(feature = "mirror_macos")]
    use crate::mirror::macos::KeyCode;
    use crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};
    include!("generated.Winput_to_CG.rs");
  }
}

pub trait Convert<F, T> {
  fn convert(from: F) -> Option<T>;
}

pub struct Converter;

pub trait ConvertExt: Sized {
  fn convert_key<T>(self) -> Option<T>
  where Converter: Convert<Self, T> {
    Converter::convert(self)
  }
}

#[cfg(feature = "mirror_winput_vk")]
impl ConvertExt for crate::mirror::winput::Vk {}
#[cfg(feature = "mirror_windows_vk")]
impl ConvertExt for crate::mirror::windows::VIRTUAL_KEY {}
#[cfg(feature = "mirror_enigo")]
impl ConvertExt for crate::mirror::enigo::Key {}
#[cfg(feature = "enigo")]
impl ConvertExt for enigo::Key {}
