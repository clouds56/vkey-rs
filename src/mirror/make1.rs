#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Make1Code(pub u32);
impl Make1Code {
  pub const fn new(keycode: u32) -> Self {
    Self(keycode)
  }

  pub const fn raw(self) -> u32 {
    self.0
  }
}

impl From<u32> for Make1Code {
  fn from(keycode: u32) -> Self {
    Self::new(keycode)
  }
}

impl Into<u32> for Make1Code {
  fn into(self) -> u32 {
    self.0
  }
}
