// https://github.com/gymore-io/winput/blob/10ea80fde5b0436a6e6594e75a6fa5f00154db70/src/vk.rs
/*
Copyright 2020 Gymore

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

pub use super::vk::Vk;

macro_rules! from_vk_for_num {
  ($($t:ty)+) => {
    $(
      impl From<Vk> for $t {
        #[inline(always)]
        fn from(vk: Vk) -> Self {
          vk as Self
        }
      }
    )+
  };
}

from_vk_for_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);

impl Vk {
  /// Creates a Virtual-Key Code from the given `u8`.
  ///
  /// ## Safety
  ///
  /// This function is safe as long as the given number `n` is a valid Virtual-Key Code.
  /// Providing a invalid number is *undefined behaviour*.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use winput::Vk;
  ///
  /// // SAFETY: `0x0d` is a valid Virtual-Key Code.
  /// let vk = unsafe { Vk::from_u8(0x0d) };
  /// assert_eq!(vk, Vk::Enter);
  /// ```
  ///
  /// A safe way to use this function is to convert a Virtual-Key Code into a number.
  ///
  /// ```rust
  /// use winput::Vk;
  ///
  /// let n = Vk::Escape.into_u8();
  ///
  /// // SAFETY: `n` is a valid Virtual-Key Code.
  /// let vk = unsafe { Vk::from_u8_unchecked(n) };
  /// assert_eq!(vk, Vk::Escape);
  /// ```
  #[inline(always)]
  pub unsafe fn from_u8_unchecked(n: u8) -> Self {
    // SAFETY: The caller must ensure that the given `u8` represents a valid
    // Virtual-Key Code.
    unsafe {
      std::mem::transmute(n)
    }
  }

  #[cfg(feature = "num_enum")]
  #[inline(always)]
  pub fn from_u8(n: u8) -> Option<Self> {
    Self::try_from(n).ok()
  }

  /// Converts this Virtual-Key Code into a `u8`.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use winput::Vk;
  ///
  /// let value = Vk::Enter.into_u8();
  /// assert_eq!(value, 0x0d);
  /// ```
  #[inline(always)]
  pub fn into_u8(self) -> u8 {
    self.into()
  }
}
