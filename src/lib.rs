
//! https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input
//!
//! VK or virtual keys have both mouse and keyboard, and is windows specific,
//! have both VK_SHIFT (0x10), VK_LSHIFT (0xA0) and VK_RSHIFT (0xA1).
//! see also https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
//!
//! Location or key position of AT101 is order (starting from 1) of keys on keyboard, regardless of language,
//! see also first picture at https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#scan-codes
//!
//! Usage or HID Usage Table is tuple of (page, id), the page of keyboard is 0x07,
//! within Keyboard Page,
//! see also https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages
//!      and https://usb.org/document-library/hid-usage-tables-16
//!
//! Scancode or Scan 1 Make is regardless of layout,
//! see also HidP_TranslateUsagesToI8042ScanCodes
//!
//! e.g.
//! |           | VK                    | AT101 | HID Usage     | Scan 1 |
//! | ` (tilde) | VK_OEM_3     0xC0 192 |     1 | 0x0007 0x0035 | 0x0029 |
//! | 1         | VK_1         0x31  49 |     2 | 0x0007 0x001E | 0x0002 |
//! | Numpad1   | VK_NUMPAD1   0x61  97 |    93 | 0x0007 0x0059 | 0x004F |
//! | A         | VK_A         0x41  65 |    31 | 0x0007 0x0004 | 0x001E |
//! | ENTER     | VK_RETURN    0x0D  13 |   108 | 0x0007 0x0058 | 0xE01C |
//! | LSHIFT    | VK_LSHIFT    0xA0 160 |    44 | 0x0007 0x00E1 | 0x002A |
//! | RSHIFT    | VK_RSHIFT    0xA1 161 |    57 | 0x0007 0x00E5 | 0x0036 |
//! | SLEEP     | VK_SLEEP     0x5F  95 |     - | 0x0001 0x0082 | 0xE05F |
//! | PLAY      | VK_PLAY      0xFA 250 |     - | 0x000C 0x00CD | 0xE022 |
//! | SHIFT     | VK_SHIFT     0x10  16 |     -
//! | LBUTTON   | VK_LBUTTON   0x01   1 |     -

pub mod mirror;
pub mod deps;
pub mod convert;
pub mod error;

pub use convert::ConvertExt;
