pub use ::hut_03;
pub use ::hut_04;
pub use hut_04 as hut;

#[cfg(all(feature = "windows", target_os = "windows"))]
pub use ::windows::Win32::UI::Input::KeyboardAndMouse::{self as windows, VIRTUAL_KEY};

#[cfg(all(feature = "enigo"))]
pub use ::enigo::{self, Key as EnigoKey};

#[cfg(all(feature = "macos", target_os = "macos"))]
pub use core_graphics::event as macos;
