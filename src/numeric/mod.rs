pub struct Coder;

#[path = "generated._index.rs"]
pub mod _impls;

pub trait AsCode<T> {
  type Code: Clone;

  fn as_code(value: &T) -> Self::Code;
  fn from_code(code: Self::Code) -> Option<T>;
  fn from_code_unchecked(code: Self::Code) -> T;
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
  fn from_code_unchecked(code: Self::Code) -> Self
  where Coder: AsCode<Self, Code = Self::Code> {
    Coder::from_code_unchecked(code)
  }
}
