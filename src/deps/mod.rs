#[cfg(dep_hut_03)]
pub use ::hut_03;
#[cfg(dep_hut_04)]
pub use ::hut_04;

#[cfg(dep_hut_04)]
pub use hut_04 as hut;
#[cfg(all(not(dep_hut_04), dep_hut_03))]
pub use hut_03 as hut;

#[cfg(dep_windows_vk)]
pub use ::windows::Win32::UI::Input::KeyboardAndMouse::{self as windows, VIRTUAL_KEY};

#[cfg(dep_enigo)]
pub use ::enigo::{self, Key as EnigoKey};

#[cfg(dep_macos)]
pub use core_graphics::event as macos;

#[cfg(dep_xkeysym)]
pub use xkeysym;
