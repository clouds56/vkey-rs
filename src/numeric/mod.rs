pub struct Coder;

#[path = "generated._index.rs"]
pub mod _impls;

pub trait AsCode<T> {
  type Code: Clone;

  fn as_code(value: &T) -> Self::Code;
  fn from_code(code: Self::Code) -> Option<T>;
  unsafe fn from_code_unchecked(code: Self::Code) -> T {
    Self::from_code(code).expect("Invalid code")
  }
}

#[allow(dead_code)]
unsafe fn convert_from_code_unchecked<T, C>(code: C) -> T
where
  Coder: AsCode<T, Code = C>,
{
  assert_eq!(
    std::mem::size_of::<C>(),
    std::mem::size_of::<T>(),
    "Size of code and value must be the same"
  );
  unsafe {
    std::ptr::read(&code as *const C as *const T)
  }
}

pub trait AsCodeExt: Sized {
  type Code: Clone;

  fn as_code(&self) -> Self::Code
  where Coder: AsCode<Self, Code = Self::Code> {
    Coder::as_code(self)
  }
  fn from_code(code: Self::Code) -> Option<Self>
  where Coder: AsCode<Self, Code = Self::Code> {
    Coder::from_code(code)
  }
  unsafe fn from_code_unchecked(code: Self::Code) -> Self
  where Coder: AsCode<Self, Code = Self::Code> {
    unsafe { Coder::from_code_unchecked(code) }
  }
}

#[cfg(any(mirror_macos, dep_macos))] // CG
impl AsCodeExt for crate::mirror::macos_ext::CGKeyCode { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(dep_hut_03)] // HUT
impl AsCodeExt for crate::deps::hut_03::Usage { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(dep_hut_04)] // HUT
impl AsCodeExt for crate::deps::hut_04::Usage { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(dep_xkeysym)] // Keysym
impl AsCodeExt for crate::deps::xkeysym::Keysym { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(mirror_winput_vk)] // Winput
impl AsCodeExt for crate::mirror::winput::Vk { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(dep_make1)] // WinScan
impl AsCodeExt for crate::mirror::make1::Make1Code { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(mirror_windows_vk)] // WinVk
impl AsCodeExt for crate::mirror::windows::VIRTUAL_KEY { type Code = <Coder as AsCode<Self>>::Code; }
#[cfg(dep_windows_vk)] // WinVk
impl AsCodeExt for crate::deps::windows::VIRTUAL_KEY { type Code = <Coder as AsCode<Self>>::Code; }
