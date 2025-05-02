#![allow(deprecated)]

include!("generated._index.rs");

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
