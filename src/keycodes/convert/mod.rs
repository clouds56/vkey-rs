#![allow(deprecated)]

mod generated_winput_to_hut {
//! https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input
//!
//! VK or virtual keys have both mouse and keyboard, and is windows specific,
//! have both VK_SHIFT (0x10), VK_LSHIFT (0xA0) and VK_RSHIFT (0xA1).
//! see also https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
//!
//! Location or key position of AT101 is order (starting from 1) of keys on keyboard, regardless of language,
//! see also first picture at https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#scan-codes
//!
//! Usage or HID Usage Table is tuple of (page, id), the page of keyboard is 0x07,
//! within Keyboard Page,
//! see also https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages
//!      and https://usb.org/document-library/hid-usage-tables-16
//!
//! Scancode or Scan 1 Make is regardless of layout,
//! see also HidP_TranslateUsagesToI8042ScanCodes
//!
//! e.g.
//! |           | VK                    | AT101 | HID Usage     | Scan 1 |
//! | ` (tilde) | VK_OEM_3     0xC0 192 |     1 | 0x0007 0x0035 | 0x0029 |
//! | 1         | VK_1         0x31  49 |     2 | 0x0007 0x001E | 0x0002 |
//! | Numpad1   | VK_NUMPAD1   0x61  97 |    93 | 0x0007 0x0059 | 0x004F |
//! | A         | VK_A         0x41  65 |    31 | 0x0007 0x0004 | 0x001E |
//! | ENTER     | VK_RETURN    0x0D  13 |   108 | 0x0007 0x0058 | 0xE01C |
//! | LSHIFT    | VK_LSHIFT    0xA0 160 |    44 | 0x0007 0x00E1 | 0x002A |
//! | RSHIFT    | VK_RSHIFT    0xA1 161 |    57 | 0x0007 0x00E5 | 0x0036 |
//! | SLEEP     | VK_SLEEP     0x5F  95 |     - | 0x0001 0x0082 | 0xE05F |
//! | PLAY      | VK_PLAY      0xFA 250 |     - | 0x000C 0x00CD | 0xE022 |
//! | SHIFT     | VK_SHIFT     0x10  16 |     -
//! | LBUTTON   | VK_LBUTTON   0x01   1 |     -

  use crate::keycodes::vk::Vk;
  use hut::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
  include!("generated.Winput_to_HUT.rs");
}

mod generated_winput_to_enigo {
  #[cfg(all(feature = "mirror_winput_vk", feature = "mirror_enigo"))]
  mod mirror_to_mirror {
    use crate::keycodes::winput::Vk;
    use crate::keycodes::enigo::Key as Enigo;
    include!("generated.Winput_to_Enigo.rs");
  }
  #[cfg(all(feature = "mirror_winput_vk", feature = "enigo", target_os = "windows"))]
  mod mirror_to_dep {
    use crate::keycodes::winput::Vk;
    use ::enigo::Key as Enigo;
    include!("generated.Winput_to_Enigo.rs");
  }
}

mod generated_enigo_to_winput {
  #[cfg(all(feature = "mirror_enigo", feature = "mirror_winput_vk"))]
  mod mirror_to_mirror {
    use crate::keycodes::enigo::Key as Enigo;
    use crate::keycodes::winput::Vk;
    include!("generated.Enigo_to_Winput.rs");
  }
  #[cfg(all(feature = "enigo", feature = "mirror_winput_vk", target_os = "windows"))]
  mod dep_to_mirror {
    use ::enigo::Key as Enigo;
    use crate::keycodes::winput::Vk;
    include!("generated.Enigo_to_Winput.rs");

  }
}

mod generated_enigo_to_vk {
  #[cfg(all(feature = "mirror_enigo", feature = "mirror_windows_vk"))]
  mod mirror_to_mirror {
    use crate::keycodes::enigo::Key as Enigo;
    use crate::keycodes::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(feature = "enigo", feature = "mirror_windows_vk", target_os = "windows"))]
  mod dep_to_mirror {
    use ::enigo::Key as Enigo;
    use crate::keycodes::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(feature = "mirror_enigo", feature = "windows", target_os = "windows"))]
  mod mirror_to_dep {
    use crate::keycodes::enigo::Key as Enigo;
    use ::windows::Win32::UI::Input::KeyboardAndMouse::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
  #[cfg(all(feature = "enigo", feature = "windows", target_os = "windows"))]
  mod dep_to_dep {
    use ::enigo::Key as Enigo;
    use ::windows::Win32::UI::Input::KeyboardAndMouse::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.Enigo_to_WinVk.rs");
  }
}

mod generated_winput_to_cg {
  #[cfg(any(feature = "macos", feature = "mirror_macos"))]
  mod mirror_to_any {
    use crate::keycodes::winput::Vk;
    #[cfg(all(feature = "macos", target_os = "macos"))]
    use core_graphics::event::KeyCode;
    #[cfg(not(all(feature = "macos", target_os = "macos")))]
    #[cfg(feature = "mirror_macos")]
    use crate::keycodes::macos::KeyCode;
    use crate::keycodes::macos_ext::{CGKeyCode, KeyCodeExt};
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
impl ConvertExt for crate::keycodes::winput::Vk {}
#[cfg(feature = "mirror_windows_vk")]
impl ConvertExt for crate::keycodes::windows::VIRTUAL_KEY {}
#[cfg(feature = "mirror_enigo")]
impl ConvertExt for crate::keycodes::enigo::Key {}
#[cfg(feature = "enigo")]
impl ConvertExt for enigo::Key {}
