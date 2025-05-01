#![allow(deprecated)]

mod generated_winput_to_hut {
  #[cfg(all(mirror_winput_vk, dep_hut_03))]
  mod mirror_to_hut_03 {
    use crate::mirror::winput::Vk;
    use crate::deps::hut_03::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.Winput_to_HUT.rs");
  }

  #[cfg(all(mirror_winput_vk, dep_hut_04))]
  mod mirror_to_hut_04 {
    use crate::mirror::winput::Vk;
    use crate::deps::hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.Winput_to_HUT.rs");
  }
}

mod generated_winput_to_enigo {
  #[cfg(all(mirror_winput_vk, mirror_enigo_windows))]
  mod mirror_to_mirror {
    use crate::mirror::winput::Vk;
    use crate::mirror::enigo::Key as Enigo;
    include!("generated.Winput_to_Enigo.rs");
  }
  #[cfg(all(mirror_winput_vk, dep_enigo_windows))]
  mod mirror_to_dep {
    use crate::mirror::winput::Vk;
    use crate::deps::enigo::Key as Enigo;
    include!("generated.Winput_to_Enigo.rs");
  }
}

mod generated_enigo_to_winput {
  #[cfg(all(mirror_enigo_windows, mirror_winput_vk))]
  mod mirror_to_mirror {
    use crate::mirror::enigo::Key as Enigo;
    use crate::mirror::winput::Vk;
    include!("generated.Enigo_to_Winput.rs");
  }
  #[cfg(all(dep_enigo_windows, mirror_winput_vk))]
  mod dep_to_mirror {
    use crate::deps::enigo::Key as Enigo;
    use crate::mirror::winput::Vk;
    include!("generated.Enigo_to_Winput.rs");

  }
}

mod generated_enigo_to_vk {
  #[cfg(all(mirror_enigo_windows, mirror_windows_vk))]
  mod mirror_to_mirror {
    use crate::mirror::enigo::Key as Enigo;
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(dep_enigo_windows, mirror_windows_vk))]
  mod dep_to_mirror {
    use crate::deps::enigo::Key as Enigo;
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(mirror_enigo_windows, dep_windows_vk))]
  mod mirror_to_dep {
    use crate::mirror::enigo::Key as Enigo;
    use crate::deps::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(dep_enigo_windows, dep_windows_vk))]
  mod dep_to_dep {
    use crate::deps::enigo::Key as Enigo;
    use crate::deps::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
}

mod generated_winput_to_cg {
  #[cfg(any(dep_macos, mirror_macos))]
  mod mirror_to_any {
    use crate::mirror::winput::Vk;
    #[cfg(dep_macos)]
    use crate::deps::macos::KeyCode;
    #[cfg(not(dep_macos))]
    #[cfg(mirror_macos)]
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

#[cfg(mirror_winput_vk)]
impl ConvertExt for crate::mirror::winput::Vk {}
#[cfg(mirror_windows_vk)]
impl ConvertExt for crate::mirror::windows::VIRTUAL_KEY {}
#[cfg(mirror_enigo)]
impl ConvertExt for crate::mirror::enigo::Key {}
#[cfg(dep_enigo)]
impl ConvertExt for enigo::Key {}
