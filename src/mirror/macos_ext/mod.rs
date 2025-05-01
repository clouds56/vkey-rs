pub mod codes_ext;

pub use codes_ext as KeyCodeExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct CGKeyCode(pub u16);
impl CGKeyCode {
  pub const fn new(keycode: u16) -> Self {
    Self(keycode)
  }
}

impl From<u16> for CGKeyCode {
  fn from(keycode: u16) -> Self {
    Self::new(keycode)
  }
}
