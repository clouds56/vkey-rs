#![allow(deprecated)]
/**
 * https://github.com/enigo-rs/enigo/blob/d109b41a78a0c903f5f0e749c130060a5a6b48fd/src/keycodes.rs
 *
 * MIT License
 *
 * Copyright (c) 2017 pythoneer
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// A key on the keyboard.
/// Use [`Key::Unicode`] to enter arbitrary Unicode chars.
/// If a key is missing, please open an issue in our repo and we will quickly
/// add it. In the mean time, you can simulate that key by using [`Key::Other`]
/// or the [`crate::Keyboard::raw`] function. Some of the keys are only
/// available on a specific platform. Use conditional compilation to use them.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    #[cfg(feature = "target_windows")]
    Num0,
    #[cfg(feature = "target_windows")]
    Num1,
    #[cfg(feature = "target_windows")]
    Num2,
    #[cfg(feature = "target_windows")]
    Num3,
    #[cfg(feature = "target_windows")]
    Num4,
    #[cfg(feature = "target_windows")]
    Num5,
    #[cfg(feature = "target_windows")]
    Num6,
    #[cfg(feature = "target_windows")]
    Num7,
    #[cfg(feature = "target_windows")]
    Num8,
    #[cfg(feature = "target_windows")]
    Num9,
    #[cfg(feature = "target_windows")]
    A,
    #[cfg(feature = "target_windows")]
    B,
    #[cfg(feature = "target_windows")]
    C,
    #[cfg(feature = "target_windows")]
    D,
    #[cfg(feature = "target_windows")]
    E,
    #[cfg(feature = "target_windows")]
    F,
    #[cfg(feature = "target_windows")]
    G,
    #[cfg(feature = "target_windows")]
    H,
    #[cfg(feature = "target_windows")]
    I,
    #[cfg(feature = "target_windows")]
    J,
    #[cfg(feature = "target_windows")]
    K,
    #[cfg(feature = "target_windows")]
    L,
    #[cfg(feature = "target_windows")]
    M,
    #[cfg(feature = "target_windows")]
    N,
    #[cfg(feature = "target_windows")]
    O,
    #[cfg(feature = "target_windows")]
    P,
    #[cfg(feature = "target_windows")]
    Q,
    #[cfg(feature = "target_windows")]
    R,
    #[cfg(feature = "target_windows")]
    S,
    #[cfg(feature = "target_windows")]
    T,
    #[cfg(feature = "target_windows")]
    U,
    #[cfg(feature = "target_windows")]
    V,
    #[cfg(feature = "target_windows")]
    W,
    #[cfg(feature = "target_windows")]
    X,
    #[cfg(feature = "target_windows")]
    Y,
    #[cfg(feature = "target_windows")]
    Z,
    #[cfg(feature = "target_windows")]
    AbntC1,
    #[cfg(feature = "target_windows")]
    AbntC2,
    #[cfg(feature = "target_windows")]
    Accept,
    Add,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    #[cfg(feature = "target_windows")]
    Apps,
    #[cfg(feature = "target_windows")]
    Attn,
    /// backspace key
    Backspace,
    #[cfg(feature = "target_linux")]
    Break,
    #[cfg(feature = "target_linux")]
    Begin,
    #[cfg(feature = "target_macos")]
    BrightnessDown,
    #[cfg(feature = "target_macos")]
    BrightnessUp,
    #[cfg(feature = "target_windows")]
    BrowserBack,
    #[cfg(feature = "target_windows")]
    BrowserFavorites,
    #[cfg(feature = "target_windows")]
    BrowserForward,
    #[cfg(feature = "target_windows")]
    BrowserHome,
    #[cfg(feature = "target_windows")]
    BrowserRefresh,
    #[cfg(feature = "target_windows")]
    BrowserSearch,
    #[cfg(feature = "target_windows")]
    BrowserStop,
    #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Cancel,
    /// caps lock key
    CapsLock,
    #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Clear,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// command key on macOS (super key on Linux, windows key on Windows)
    #[cfg_attr(feature = "serde", serde(alias = "cmd"))]
    Command,
    #[cfg(feature = "target_macos")]
    ContrastUp,
    #[cfg(feature = "target_macos")]
    ContrastDown,
    /// control key
    #[cfg_attr(feature = "serde", serde(alias = "ctrl"))]
    Control,
    #[cfg(feature = "target_windows")]
    Convert,
    #[cfg(feature = "target_windows")]
    Crsel,
    #[cfg(feature = "target_windows")]
    DBEAlphanumeric,
    #[cfg(feature = "target_windows")]
    DBECodeinput,
    #[cfg(feature = "target_windows")]
    DBEDetermineString,
    #[cfg(feature = "target_windows")]
    DBEEnterDLGConversionMode,
    #[cfg(feature = "target_windows")]
    DBEEnterIMEConfigMode,
    #[cfg(feature = "target_windows")]
    DBEEnterWordRegisterMode,
    #[cfg(feature = "target_windows")]
    DBEFlushString,
    #[cfg(feature = "target_windows")]
    DBEHiragana,
    #[cfg(feature = "target_windows")]
    DBEKatakana,
    #[cfg(feature = "target_windows")]
    DBENoCodepoint,
    #[cfg(feature = "target_windows")]
    DBENoRoman,
    #[cfg(feature = "target_windows")]
    DBERoman,
    #[cfg(feature = "target_windows")]
    DBESBCSChar,
    #[cfg(feature = "target_windows")]
    DBESChar,
    Decimal,
    /// delete key
    Delete,
    Divide,
    /// down arrow key
    DownArrow,
    #[cfg(feature = "target_macos")]
    Eject,
    /// end key
    End,
    #[cfg(feature = "target_windows")]
    Ereof,
    /// escape key (esc)
    Escape,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Execute,
    #[cfg(feature = "target_windows")]
    Exsel,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    /// F21 key
    F21,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    /// F22 key
    F22,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    /// F23 key
    F23,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    /// F24 key
    F24,
    #[cfg(feature = "target_linux")]
    F25,
    #[cfg(feature = "target_linux")]
    F26,
    #[cfg(feature = "target_linux")]
    F27,
    #[cfg(feature = "target_linux")]
    F28,
    #[cfg(feature = "target_linux")]
    F29,
    #[cfg(feature = "target_linux")]
    F30,
    #[cfg(feature = "target_linux")]
    F31,
    #[cfg(feature = "target_linux")]
    F32,
    #[cfg(feature = "target_linux")]
    F33,
    #[cfg(feature = "target_linux")]
    F34,
    #[cfg(feature = "target_linux")]
    F35,
    #[cfg(feature = "target_macos")]
    Function,
    #[cfg(feature = "target_windows")]
    Final,
    #[cfg(feature = "target_linux")]
    Find,
    #[cfg(feature = "target_windows")]
    GamepadA,
    #[cfg(feature = "target_windows")]
    GamepadB,
    #[cfg(feature = "target_windows")]
    GamepadDPadDown,
    #[cfg(feature = "target_windows")]
    GamepadDPadLeft,
    #[cfg(feature = "target_windows")]
    GamepadDPadRight,
    #[cfg(feature = "target_windows")]
    GamepadDPadUp,
    #[cfg(feature = "target_windows")]
    GamepadLeftShoulder,
    #[cfg(feature = "target_windows")]
    GamepadLeftThumbstickButton,
    #[cfg(feature = "target_windows")]
    GamepadLeftThumbstickDown,
    #[cfg(feature = "target_windows")]
    GamepadLeftThumbstickLeft,
    #[cfg(feature = "target_windows")]
    GamepadLeftThumbstickRight,
    #[cfg(feature = "target_windows")]
    GamepadLeftThumbstickUp,
    #[cfg(feature = "target_windows")]
    GamepadLeftTrigger,
    #[cfg(feature = "target_windows")]
    GamepadMenu,
    #[cfg(feature = "target_windows")]
    GamepadRightShoulder,
    #[cfg(feature = "target_windows")]
    GamepadRightThumbstickButton,
    #[cfg(feature = "target_windows")]
    GamepadRightThumbstickDown,
    #[cfg(feature = "target_windows")]
    GamepadRightThumbstickLeft,
    #[cfg(feature = "target_windows")]
    GamepadRightThumbstickRight,
    #[cfg(feature = "target_windows")]
    GamepadRightThumbstickUp,
    #[cfg(feature = "target_windows")]
    GamepadRightTrigger,
    #[cfg(feature = "target_windows")]
    GamepadView,
    #[cfg(feature = "target_windows")]
    GamepadX,
    #[cfg(feature = "target_windows")]
    GamepadY,
    #[cfg(feature = "target_windows")]
    Hangeul,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Hangul,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Hanja,
    Help,
    /// home key
    Home,
    #[cfg(feature = "target_windows")]
    Ico00,
    #[cfg(feature = "target_windows")]
    IcoClear,
    #[cfg(feature = "target_windows")]
    IcoHelp,
    #[cfg(feature = "target_macos")]
    IlluminationDown,
    #[cfg(feature = "target_macos")]
    IlluminationUp,
    #[cfg(feature = "target_macos")]
    IlluminationToggle,
    #[cfg(feature = "target_windows")]
    IMEOff,
    #[cfg(feature = "target_windows")]
    IMEOn,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Insert,
    #[cfg(feature = "target_windows")]
    Junja,
    #[cfg(feature = "target_windows")]
    Kana,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Kanji,
    #[cfg(feature = "target_windows")]
    LaunchApp1,
    #[cfg(feature = "target_windows")]
    LaunchApp2,
    #[cfg(feature = "target_windows")]
    LaunchMail,
    #[cfg(feature = "target_windows")]
    LaunchMediaSelect,
    #[cfg(feature = "target_macos")]
    /// Opens launchpad
    Launchpad,
    #[cfg(feature = "target_macos")]
    LaunchPanel,
    #[cfg(feature = "target_windows")]
    LButton,
    LControl,
    /// left arrow key
    LeftArrow,
    #[cfg(feature = "target_linux")]
    Linefeed,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    LMenu,
    LShift,
    #[cfg(feature = "target_windows")]
    LWin,
    #[cfg(feature = "target_windows")]
    MButton,
    #[cfg(feature = "target_macos")]
    MediaFast,
    MediaNextTrack,
    MediaPlayPause,
    MediaPrevTrack,
    #[cfg(feature = "target_macos")]
    MediaRewind,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    MediaStop,
    /// meta key (also known as "windows", "super", and "command")
    Meta,
    #[cfg(feature = "target_macos")]
    /// Opens mission control
    MissionControl,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    ModeChange,
    Multiply,
    #[cfg(feature = "target_windows")]
    NavigationAccept,
    #[cfg(feature = "target_windows")]
    NavigationCancel,
    #[cfg(feature = "target_windows")]
    NavigationDown,
    #[cfg(feature = "target_windows")]
    NavigationLeft,
    #[cfg(feature = "target_windows")]
    NavigationMenu,
    #[cfg(feature = "target_windows")]
    NavigationRight,
    #[cfg(feature = "target_windows")]
    NavigationUp,
    #[cfg(feature = "target_windows")]
    NavigationView,
    #[cfg(feature = "target_windows")]
    NoName,
    #[cfg(feature = "target_windows")]
    NonConvert,
    #[cfg(feature = "target_windows")]
    None,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    #[cfg(feature = "target_windows")]
    OEM1,
    #[cfg(feature = "target_windows")]
    OEM102,
    #[cfg(feature = "target_windows")]
    OEM2,
    #[cfg(feature = "target_windows")]
    OEM3,
    #[cfg(feature = "target_windows")]
    OEM4,
    #[cfg(feature = "target_windows")]
    OEM5,
    #[cfg(feature = "target_windows")]
    OEM6,
    #[cfg(feature = "target_windows")]
    OEM7,
    #[cfg(feature = "target_windows")]
    OEM8,
    #[cfg(feature = "target_windows")]
    OEMAttn,
    #[cfg(feature = "target_windows")]
    OEMAuto,
    #[cfg(feature = "target_windows")]
    OEMAx,
    #[cfg(feature = "target_windows")]
    OEMBacktab,
    #[cfg(feature = "target_windows")]
    OEMClear,
    #[cfg(feature = "target_windows")]
    OEMComma,
    #[cfg(feature = "target_windows")]
    OEMCopy,
    #[cfg(feature = "target_windows")]
    OEMCusel,
    #[cfg(feature = "target_windows")]
    OEMEnlw,
    #[cfg(feature = "target_windows")]
    OEMFinish,
    #[cfg(feature = "target_windows")]
    OEMFJJisho,
    #[cfg(feature = "target_windows")]
    OEMFJLoya,
    #[cfg(feature = "target_windows")]
    OEMFJMasshou,
    #[cfg(feature = "target_windows")]
    OEMFJRoya,
    #[cfg(feature = "target_windows")]
    OEMFJTouroku,
    #[cfg(feature = "target_windows")]
    OEMJump,
    #[cfg(feature = "target_windows")]
    OEMMinus,
    #[cfg(feature = "target_windows")]
    OEMNECEqual,
    #[cfg(feature = "target_windows")]
    OEMPA1,
    #[cfg(feature = "target_windows")]
    OEMPA2,
    #[cfg(feature = "target_windows")]
    OEMPA3,
    #[cfg(feature = "target_windows")]
    OEMPeriod,
    #[cfg(feature = "target_windows")]
    OEMPlus,
    #[cfg(feature = "target_windows")]
    OEMReset,
    #[cfg(feature = "target_windows")]
    OEMWsctrl,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    #[cfg(feature = "target_windows")]
    PA1,
    #[cfg(feature = "target_windows")]
    Packet,
    /// page down key
    PageDown,
    /// page up key
    PageUp,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Pause,
    #[cfg(feature = "target_windows")]
    Play,
    #[cfg(feature = "target_macos")]
    Power,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Print,
    /// Take a screenshot
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    #[doc(alias = "Print")]
    #[doc(alias = "Snapshot")]
    PrintScr,
    #[cfg(feature = "target_windows")]
    Processkey,
    #[cfg(feature = "target_windows")]
    RButton,
    #[cfg(feature = "target_macos")]
    RCommand,
    RControl,
    #[cfg(feature = "target_linux")]
    Redo,
    /// return key
    Return,
    /// right arrow key
    RightArrow,
    #[cfg(feature = "target_windows")]
    RMenu,
    #[cfg(feature = "target_macos")]
    ROption,
    RShift,
    #[cfg(feature = "target_windows")]
    RWin,
    #[cfg(feature = "target_windows")]
    Scroll,
    #[cfg(feature = "target_linux")]
    ScrollLock,
     #[cfg(any(feature = "target_windows", feature = "target_linux"))]
    Select,
    #[cfg(feature = "target_linux")]
    ScriptSwitch,
    #[cfg(feature = "target_windows")]
    Separator,
    /// shift key
    Shift,
    #[cfg(feature = "target_linux")]
    /// Lock shift key
    ShiftLock,
    #[cfg(feature = "target_windows")]
    Sleep,
    #[cfg(feature = "target_windows")]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Snapshot,
    /// space key
    Space,
    Subtract,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// super key on linux (command key on macOS, windows key on Windows)
    Super,
    #[cfg(feature = "target_linux")]
    SysReq,
    /// tab key (tabulator)
    Tab,
    #[cfg(feature = "target_linux")]
    Undo,
    /// up arrow key
    UpArrow,
    #[cfg(feature = "target_macos")]
    VidMirror,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    #[cfg(feature = "target_linux")]
    /// microphone mute toggle on linux
    MicMute,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// windows key on Windows (super key on Linux, command key on macOS)
    Windows,
    #[cfg(feature = "target_windows")]
    XButton1,
    #[cfg(feature = "target_windows")]
    XButton2,
    #[cfg(feature = "target_windows")]
    Zoom,
    /// Unicode character
    #[doc(alias = "Layout")]
    #[cfg_attr(feature = "serde", serde(alias = "uni"))]
    #[cfg_attr(feature = "serde", serde(alias = "Uni"))]
    #[cfg_attr(feature = "serde", serde(alias = "Char"))]
    #[cfg_attr(feature = "serde", serde(alias = "char"))]
    Unicode(char),
    /// Use this for keys that are not listed here that you know the
    /// value of. Let us know if you think the key should be listed so
    /// we can add it
    /// On Linux, this will result in a keysym,
    /// On Windows, this will result in a `Virtual_Key` and
    /// On macOS, this will yield a `KeyCode`
    Other(u32),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    #[cfg_attr(feature = "serde", serde(alias = "shift"))]
    Shift,
    #[cfg_attr(feature = "serde", serde(alias = "lock"))]
    Lock,
    #[cfg_attr(feature = "serde", serde(alias = "control"))]
    #[cfg_attr(feature = "serde", serde(alias = "crtl"))]
    Control,
    #[cfg_attr(feature = "serde", serde(alias = "mod1"))]
    #[cfg_attr(feature = "serde", serde(alias = "m1"))]
    Mod1,
    #[cfg_attr(feature = "serde", serde(alias = "mod2"))]
    #[cfg_attr(feature = "serde", serde(alias = "m2"))]
    Mod2,
    #[cfg_attr(feature = "serde", serde(alias = "mod3"))]
    #[cfg_attr(feature = "serde", serde(alias = "m3"))]
    Mod3,
    #[cfg_attr(feature = "serde", serde(alias = "mod4"))]
    #[cfg_attr(feature = "serde", serde(alias = "m4"))]
    Mod4,
    #[cfg_attr(feature = "serde", serde(alias = "mod5"))]
    #[cfg_attr(feature = "serde", serde(alias = "m5"))]
    Mod5,
}

/// Converts a Key to a modifier
impl TryFrom<Key> for Modifier {
    type Error = &'static str;

    fn try_from(key: Key) -> Result<Self, &'static str> {
        match key {
            Key::Shift | Key::LShift | Key::RShift => Ok(Self::Shift),
            Key::CapsLock => Ok(Self::Lock),
            Key::Control | Key::LControl | Key::RControl => Ok(Self::Control),
            Key::Alt | Key::Option => Ok(Self::Mod1),
            Key::Numlock => Ok(Self::Mod2),
            // The Mod3 modifier is usually unmapped
            // Key::Mod3 => Ok(Self::Mod3),
            Key::Command | Key::Super | Key::Windows | Key::Meta => Ok(Self::Mod4),
            Key::ModeChange => Ok(Self::Mod5),
            _ => Err("not a modifier key"),
        }
    }
}
