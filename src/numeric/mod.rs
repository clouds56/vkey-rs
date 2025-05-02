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
