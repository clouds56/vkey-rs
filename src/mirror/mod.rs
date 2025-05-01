#[cfg(feature = "mirror_winput_vk")]
pub mod winput;
#[cfg(feature = "mirror_enigo")]
pub mod enigo;
#[cfg(feature = "mirror_windows_vk")]
pub mod windows;
#[cfg(feature = "mirror_winit")]
pub mod winit;
#[cfg(feature = "mirror_macos")]
pub mod macos;
pub mod macos_ext;

pub mod native_code {
  pub mod windows {
    #[cfg(all(feature = "windows", target_os = "windows"))]
    pub use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY as NativeKeyCode;
    #[cfg(not(all(feature = "windows", target_os = "windows")))]
    #[cfg(feature = "mirror_windows_vk")]
    pub use crate::mirror::windows::VIRTUAL_KEY as NativeKeyCode;
  }
}
