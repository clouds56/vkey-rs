#![allow(deprecated)]

#[path = "generated._index.rs"]
pub mod _impls;

pub trait Convert<F, T> {
  fn convert(from: F) -> Option<T>;
}

pub struct Converter;

#[allow(dead_code)]
pub(crate) trait Into_ {
  fn into_<T>(self) -> T where Self: Into<T> {
    Into::into(self)
  }
}

impl<T> Into_ for T {}

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
#[cfg(dep_windows_vk)]
impl ConvertExt for crate::deps::windows::VIRTUAL_KEY {}
#[cfg(mirror_enigo)]
impl ConvertExt for crate::mirror::enigo::Key {}
#[cfg(dep_enigo)]
impl ConvertExt for crate::deps::enigo::Key {}
#[cfg(dep_hut_03)]
impl ConvertExt for crate::deps::hut_03::Usage {}
#[cfg(dep_hut_04)]
impl ConvertExt for crate::deps::hut_04::Usage {}
#[cfg(any(mirror_macos, dep_macos))]
impl ConvertExt for crate::mirror::macos_ext::CGKeyCode {}
#[cfg(dep_xkeysym)]
impl ConvertExt for crate::deps::xkeysym::Keysym {}
