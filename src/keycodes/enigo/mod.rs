pub mod key;

#[cfg(feature = "target_windows")]
pub mod windows {
  use super::key::Key;

  #[cfg(all(feature = "windows", target_os = "windows"))]
  mod _windows {
    use windows::Win32::UI::Input::KeyboardAndMouse::{self, VIRTUAL_KEY};
    include!("windows.rs");
  }
  #[cfg(feature = "mirror_windows_vk")]
  mod _mirror_windows_vk {
    use crate::keycodes::windows::{self as KeyboardAndMouse, VIRTUAL_KEY};
    include!("windows.rs");
  }
}

#[cfg(feature = "target_linux")]
pub mod linux;

#[cfg(feature = "target_macos")]
pub mod macos {
  use super::key::Key;
  #[cfg(all(feature = "macos", target_os = "macos"))]
  mod _macos {
    use core_graphics::event::{CGKeyCode, KeyCode};
    include!("macos.rs");
  }

  #[cfg(feature = "mirror_windows_vk")]
  mod _mirror_macos {
    use crate::keycodes::{macos::KeyCode, macos_ext::CGKeyCode};
    include!("macos.rs");
  }
}

pub use key::*;
