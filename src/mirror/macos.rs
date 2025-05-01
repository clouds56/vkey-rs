pub type CGKeyCode = u16;

/* https://github.com/servo/core-foundation-rs/blob/0e783c63558ad282ccdea39097892132a33c4aec/core-graphics/src/event.rs

Copyright (c) 2012-2013 Mozilla Foundation

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

*/

/// Constants for the virtual key codes
///
/// These constants are the virtual keycodes defined originally in
/// Inside Mac Volume V, pg. V-191. They identify physical keys on a
/// keyboard. The struct contains the values of the `ANSIKeyCode`,
/// `KeyCode`, `ISOKeyCode` and `JISKeyCode` of the original Carbon headers.
///
/// Those constants with "ANSI" in the name are labeled
/// according to the key position on an ANSI-standard US keyboard.
/// For example, `ANSI_A` indicates the virtual keycode for the key
/// with the letter 'A' in the US keyboard layout. Other keyboard
/// layouts may have the 'A' key label on a different physical key;
/// in this case, pressing 'A' will generate a different virtual
/// keycode. Constants with the 'JIS_' or 'ISO_' prefix behave
/// analogously. Keys without a prefix are independent of the
/// keyboard layout.
///
/// [Ref](https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.13.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h#L197-L327)
#[repr(C)]
pub struct KeyCode;
impl KeyCode {
    pub const ANSI_A: CGKeyCode = 0x00;
    pub const ANSI_S: CGKeyCode = 0x01;
    pub const ANSI_D: CGKeyCode = 0x02;
    pub const ANSI_F: CGKeyCode = 0x03;
    pub const ANSI_H: CGKeyCode = 0x04;
    pub const ANSI_G: CGKeyCode = 0x05;
    pub const ANSI_Z: CGKeyCode = 0x06;
    pub const ANSI_X: CGKeyCode = 0x07;
    pub const ANSI_C: CGKeyCode = 0x08;
    pub const ANSI_V: CGKeyCode = 0x09;
    pub const ANSI_B: CGKeyCode = 0x0B;
    pub const ANSI_Q: CGKeyCode = 0x0C;
    pub const ANSI_W: CGKeyCode = 0x0D;
    pub const ANSI_E: CGKeyCode = 0x0E;
    pub const ANSI_R: CGKeyCode = 0x0F;
    pub const ANSI_Y: CGKeyCode = 0x10;
    pub const ANSI_T: CGKeyCode = 0x11;
    pub const ANSI_1: CGKeyCode = 0x12;
    pub const ANSI_2: CGKeyCode = 0x13;
    pub const ANSI_3: CGKeyCode = 0x14;
    pub const ANSI_4: CGKeyCode = 0x15;
    pub const ANSI_6: CGKeyCode = 0x16;
    pub const ANSI_5: CGKeyCode = 0x17;
    pub const ANSI_EQUAL: CGKeyCode = 0x18;
    pub const ANSI_9: CGKeyCode = 0x19;
    pub const ANSI_7: CGKeyCode = 0x1A;
    pub const ANSI_MINUS: CGKeyCode = 0x1B;
    pub const ANSI_8: CGKeyCode = 0x1C;
    pub const ANSI_0: CGKeyCode = 0x1D;
    pub const ANSI_RIGHT_BRACKET: CGKeyCode = 0x1E;
    pub const ANSI_O: CGKeyCode = 0x1F;
    pub const ANSI_U: CGKeyCode = 0x20;
    pub const ANSI_LEFT_BRACKET: CGKeyCode = 0x21;
    pub const ANSI_I: CGKeyCode = 0x22;
    pub const ANSI_P: CGKeyCode = 0x23;
    pub const ANSI_L: CGKeyCode = 0x25;
    pub const ANSI_J: CGKeyCode = 0x26;
    pub const ANSI_QUOTE: CGKeyCode = 0x27;
    pub const ANSI_K: CGKeyCode = 0x28;
    pub const ANSI_SEMICOLON: CGKeyCode = 0x29;
    pub const ANSI_BACKSLASH: CGKeyCode = 0x2A;
    pub const ANSI_COMMA: CGKeyCode = 0x2B;
    pub const ANSI_SLASH: CGKeyCode = 0x2C;
    pub const ANSI_N: CGKeyCode = 0x2D;
    pub const ANSI_M: CGKeyCode = 0x2E;
    pub const ANSI_PERIOD: CGKeyCode = 0x2F;
    pub const ANSI_GRAVE: CGKeyCode = 0x32;
    pub const ANSI_KEYPAD_DECIMAL: CGKeyCode = 0x41;
    pub const ANSI_KEYPAD_MULTIPLY: CGKeyCode = 0x43;
    pub const ANSI_KEYPAD_PLUS: CGKeyCode = 0x45;
    pub const ANSI_KEYPAD_CLEAR: CGKeyCode = 0x47;
    pub const ANSI_KEYPAD_DIVIDE: CGKeyCode = 0x4B;
    pub const ANSI_KEYPAD_ENTER: CGKeyCode = 0x4C;
    pub const ANSI_KEYPAD_MINUS: CGKeyCode = 0x4E;
    pub const ANSI_KEYPAD_EQUAL: CGKeyCode = 0x51;
    pub const ANSI_KEYPAD_0: CGKeyCode = 0x52;
    pub const ANSI_KEYPAD_1: CGKeyCode = 0x53;
    pub const ANSI_KEYPAD_2: CGKeyCode = 0x54;
    pub const ANSI_KEYPAD_3: CGKeyCode = 0x55;
    pub const ANSI_KEYPAD_4: CGKeyCode = 0x56;
    pub const ANSI_KEYPAD_5: CGKeyCode = 0x57;
    pub const ANSI_KEYPAD_6: CGKeyCode = 0x58;
    pub const ANSI_KEYPAD_7: CGKeyCode = 0x59;
    pub const ANSI_KEYPAD_8: CGKeyCode = 0x5B;
    pub const ANSI_KEYPAD_9: CGKeyCode = 0x5C;
    pub const RETURN: CGKeyCode = 0x24;
    pub const TAB: CGKeyCode = 0x30;
    pub const SPACE: CGKeyCode = 0x31;
    pub const DELETE: CGKeyCode = 0x33;
    pub const ESCAPE: CGKeyCode = 0x35;
    pub const COMMAND: CGKeyCode = 0x37;
    pub const SHIFT: CGKeyCode = 0x38;
    pub const CAPS_LOCK: CGKeyCode = 0x39;
    pub const OPTION: CGKeyCode = 0x3A;
    pub const CONTROL: CGKeyCode = 0x3B;
    pub const RIGHT_COMMAND: CGKeyCode = 0x36;
    pub const RIGHT_SHIFT: CGKeyCode = 0x3C;
    pub const RIGHT_OPTION: CGKeyCode = 0x3D;
    pub const RIGHT_CONTROL: CGKeyCode = 0x3E;
    pub const FUNCTION: CGKeyCode = 0x3F;
    pub const F17: CGKeyCode = 0x40;
    pub const VOLUME_UP: CGKeyCode = 0x48;
    pub const VOLUME_DOWN: CGKeyCode = 0x49;
    pub const MUTE: CGKeyCode = 0x4A;
    pub const F18: CGKeyCode = 0x4F;
    pub const F19: CGKeyCode = 0x50;
    pub const F20: CGKeyCode = 0x5A;
    pub const F5: CGKeyCode = 0x60;
    pub const F6: CGKeyCode = 0x61;
    pub const F7: CGKeyCode = 0x62;
    pub const F3: CGKeyCode = 0x63;
    pub const F8: CGKeyCode = 0x64;
    pub const F9: CGKeyCode = 0x65;
    pub const F11: CGKeyCode = 0x67;
    pub const F13: CGKeyCode = 0x69;
    pub const F16: CGKeyCode = 0x6A;
    pub const F14: CGKeyCode = 0x6B;
    pub const F10: CGKeyCode = 0x6D;
    pub const F12: CGKeyCode = 0x6F;
    pub const F15: CGKeyCode = 0x71;
    pub const HELP: CGKeyCode = 0x72;
    pub const HOME: CGKeyCode = 0x73;
    pub const PAGE_UP: CGKeyCode = 0x74;
    pub const FORWARD_DELETE: CGKeyCode = 0x75;
    pub const F4: CGKeyCode = 0x76;
    pub const END: CGKeyCode = 0x77;
    pub const F2: CGKeyCode = 0x78;
    pub const PAGE_DOWN: CGKeyCode = 0x79;
    pub const F1: CGKeyCode = 0x7A;
    pub const LEFT_ARROW: CGKeyCode = 0x7B;
    pub const RIGHT_ARROW: CGKeyCode = 0x7C;
    pub const DOWN_ARROW: CGKeyCode = 0x7D;
    pub const UP_ARROW: CGKeyCode = 0x7E;
    pub const ISO_SECTION: CGKeyCode = 0x0A;
    pub const JIS_YEN: CGKeyCode = 0x5D;
    pub const JIS_UNDERSCORE: CGKeyCode = 0x5E;
    pub const JIS_KEYPAD_COMMA: CGKeyCode = 0x5F;
    pub const JIS_EISU: CGKeyCode = 0x66;
    pub const JIS_KANA: CGKeyCode = 0x68;
}
