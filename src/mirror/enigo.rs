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
    #[cfg(for_windows)]
    Num0,
    #[cfg(for_windows)]
    Num1,
    #[cfg(for_windows)]
    Num2,
    #[cfg(for_windows)]
    Num3,
    #[cfg(for_windows)]
    Num4,
    #[cfg(for_windows)]
    Num5,
    #[cfg(for_windows)]
    Num6,
    #[cfg(for_windows)]
    Num7,
    #[cfg(for_windows)]
    Num8,
    #[cfg(for_windows)]
    Num9,
    #[cfg(for_windows)]
    A,
    #[cfg(for_windows)]
    B,
    #[cfg(for_windows)]
    C,
    #[cfg(for_windows)]
    D,
    #[cfg(for_windows)]
    E,
    #[cfg(for_windows)]
    F,
    #[cfg(for_windows)]
    G,
    #[cfg(for_windows)]
    H,
    #[cfg(for_windows)]
    I,
    #[cfg(for_windows)]
    J,
    #[cfg(for_windows)]
    K,
    #[cfg(for_windows)]
    L,
    #[cfg(for_windows)]
    M,
    #[cfg(for_windows)]
    N,
    #[cfg(for_windows)]
    O,
    #[cfg(for_windows)]
    P,
    #[cfg(for_windows)]
    Q,
    #[cfg(for_windows)]
    R,
    #[cfg(for_windows)]
    S,
    #[cfg(for_windows)]
    T,
    #[cfg(for_windows)]
    U,
    #[cfg(for_windows)]
    V,
    #[cfg(for_windows)]
    W,
    #[cfg(for_windows)]
    X,
    #[cfg(for_windows)]
    Y,
    #[cfg(for_windows)]
    Z,
    #[cfg(for_windows)]
    AbntC1,
    #[cfg(for_windows)]
    AbntC2,
    #[cfg(for_windows)]
    Accept,
    Add,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    #[cfg(for_windows)]
    Apps,
    #[cfg(for_windows)]
    Attn,
    /// backspace key
    Backspace,
    #[cfg(for_linux)]
    Break,
    #[cfg(for_linux)]
    Begin,
    #[cfg(for_macos)]
    BrightnessDown,
    #[cfg(for_macos)]
    BrightnessUp,
    #[cfg(for_windows)]
    BrowserBack,
    #[cfg(for_windows)]
    BrowserFavorites,
    #[cfg(for_windows)]
    BrowserForward,
    #[cfg(for_windows)]
    BrowserHome,
    #[cfg(for_windows)]
    BrowserRefresh,
    #[cfg(for_windows)]
    BrowserSearch,
    #[cfg(for_windows)]
    BrowserStop,
    #[cfg(any(for_windows, for_linux))]
    Cancel,
    /// caps lock key
    CapsLock,
    #[cfg(any(for_windows, for_linux))]
    Clear,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// command key on macOS (super key on Linux, windows key on Windows)
    #[cfg_attr(feature = "serde", serde(alias = "cmd"))]
    Command,
    #[cfg(for_macos)]
    ContrastUp,
    #[cfg(for_macos)]
    ContrastDown,
    /// control key
    #[cfg_attr(feature = "serde", serde(alias = "ctrl"))]
    Control,
    #[cfg(for_windows)]
    Convert,
    #[cfg(for_windows)]
    Crsel,
    #[cfg(for_windows)]
    DBEAlphanumeric,
    #[cfg(for_windows)]
    DBECodeinput,
    #[cfg(for_windows)]
    DBEDetermineString,
    #[cfg(for_windows)]
    DBEEnterDLGConversionMode,
    #[cfg(for_windows)]
    DBEEnterIMEConfigMode,
    #[cfg(for_windows)]
    DBEEnterWordRegisterMode,
    #[cfg(for_windows)]
    DBEFlushString,
    #[cfg(for_windows)]
    DBEHiragana,
    #[cfg(for_windows)]
    DBEKatakana,
    #[cfg(for_windows)]
    DBENoCodepoint,
    #[cfg(for_windows)]
    DBENoRoman,
    #[cfg(for_windows)]
    DBERoman,
    #[cfg(for_windows)]
    DBESBCSChar,
    #[cfg(for_windows)]
    DBESChar,
    Decimal,
    /// delete key
    Delete,
    Divide,
    /// down arrow key
    DownArrow,
    #[cfg(for_macos)]
    Eject,
    /// end key
    End,
    #[cfg(for_windows)]
    Ereof,
    /// escape key (esc)
    Escape,
     #[cfg(any(for_windows, for_linux))]
    Execute,
    #[cfg(for_windows)]
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
     #[cfg(any(for_windows, for_linux))]
    /// F21 key
    F21,
     #[cfg(any(for_windows, for_linux))]
    /// F22 key
    F22,
     #[cfg(any(for_windows, for_linux))]
    /// F23 key
    F23,
     #[cfg(any(for_windows, for_linux))]
    /// F24 key
    F24,
    #[cfg(for_linux)]
    F25,
    #[cfg(for_linux)]
    F26,
    #[cfg(for_linux)]
    F27,
    #[cfg(for_linux)]
    F28,
    #[cfg(for_linux)]
    F29,
    #[cfg(for_linux)]
    F30,
    #[cfg(for_linux)]
    F31,
    #[cfg(for_linux)]
    F32,
    #[cfg(for_linux)]
    F33,
    #[cfg(for_linux)]
    F34,
    #[cfg(for_linux)]
    F35,
    #[cfg(for_macos)]
    Function,
    #[cfg(for_windows)]
    Final,
    #[cfg(for_linux)]
    Find,
    #[cfg(for_windows)]
    GamepadA,
    #[cfg(for_windows)]
    GamepadB,
    #[cfg(for_windows)]
    GamepadDPadDown,
    #[cfg(for_windows)]
    GamepadDPadLeft,
    #[cfg(for_windows)]
    GamepadDPadRight,
    #[cfg(for_windows)]
    GamepadDPadUp,
    #[cfg(for_windows)]
    GamepadLeftShoulder,
    #[cfg(for_windows)]
    GamepadLeftThumbstickButton,
    #[cfg(for_windows)]
    GamepadLeftThumbstickDown,
    #[cfg(for_windows)]
    GamepadLeftThumbstickLeft,
    #[cfg(for_windows)]
    GamepadLeftThumbstickRight,
    #[cfg(for_windows)]
    GamepadLeftThumbstickUp,
    #[cfg(for_windows)]
    GamepadLeftTrigger,
    #[cfg(for_windows)]
    GamepadMenu,
    #[cfg(for_windows)]
    GamepadRightShoulder,
    #[cfg(for_windows)]
    GamepadRightThumbstickButton,
    #[cfg(for_windows)]
    GamepadRightThumbstickDown,
    #[cfg(for_windows)]
    GamepadRightThumbstickLeft,
    #[cfg(for_windows)]
    GamepadRightThumbstickRight,
    #[cfg(for_windows)]
    GamepadRightThumbstickUp,
    #[cfg(for_windows)]
    GamepadRightTrigger,
    #[cfg(for_windows)]
    GamepadView,
    #[cfg(for_windows)]
    GamepadX,
    #[cfg(for_windows)]
    GamepadY,
    #[cfg(for_windows)]
    Hangeul,
     #[cfg(any(for_windows, for_linux))]
    Hangul,
     #[cfg(any(for_windows, for_linux))]
    Hanja,
    Help,
    /// home key
    Home,
    #[cfg(for_windows)]
    Ico00,
    #[cfg(for_windows)]
    IcoClear,
    #[cfg(for_windows)]
    IcoHelp,
    #[cfg(for_macos)]
    IlluminationDown,
    #[cfg(for_macos)]
    IlluminationUp,
    #[cfg(for_macos)]
    IlluminationToggle,
    #[cfg(for_windows)]
    IMEOff,
    #[cfg(for_windows)]
    IMEOn,
     #[cfg(any(for_windows, for_linux))]
    Insert,
    #[cfg(for_windows)]
    Junja,
    #[cfg(for_windows)]
    Kana,
     #[cfg(any(for_windows, for_linux))]
    Kanji,
    #[cfg(for_windows)]
    LaunchApp1,
    #[cfg(for_windows)]
    LaunchApp2,
    #[cfg(for_windows)]
    LaunchMail,
    #[cfg(for_windows)]
    LaunchMediaSelect,
    #[cfg(for_macos)]
    /// Opens launchpad
    Launchpad,
    #[cfg(for_macos)]
    LaunchPanel,
    #[cfg(for_windows)]
    LButton,
    LControl,
    /// left arrow key
    LeftArrow,
    #[cfg(for_linux)]
    Linefeed,
     #[cfg(any(for_windows, for_linux))]
    LMenu,
    LShift,
    #[cfg(for_windows)]
    LWin,
    #[cfg(for_windows)]
    MButton,
    #[cfg(for_macos)]
    MediaFast,
    MediaNextTrack,
    MediaPlayPause,
    MediaPrevTrack,
    #[cfg(for_macos)]
    MediaRewind,
     #[cfg(any(for_windows, for_linux))]
    MediaStop,
    /// meta key (also known as "windows", "super", and "command")
    Meta,
    #[cfg(for_macos)]
    /// Opens mission control
    MissionControl,
     #[cfg(any(for_windows, for_linux))]
    ModeChange,
    Multiply,
    #[cfg(for_windows)]
    NavigationAccept,
    #[cfg(for_windows)]
    NavigationCancel,
    #[cfg(for_windows)]
    NavigationDown,
    #[cfg(for_windows)]
    NavigationLeft,
    #[cfg(for_windows)]
    NavigationMenu,
    #[cfg(for_windows)]
    NavigationRight,
    #[cfg(for_windows)]
    NavigationUp,
    #[cfg(for_windows)]
    NavigationView,
    #[cfg(for_windows)]
    NoName,
    #[cfg(for_windows)]
    NonConvert,
    #[cfg(for_windows)]
    None,
     #[cfg(any(for_windows, for_linux))]
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
    #[cfg(for_windows)]
    OEM1,
    #[cfg(for_windows)]
    OEM102,
    #[cfg(for_windows)]
    OEM2,
    #[cfg(for_windows)]
    OEM3,
    #[cfg(for_windows)]
    OEM4,
    #[cfg(for_windows)]
    OEM5,
    #[cfg(for_windows)]
    OEM6,
    #[cfg(for_windows)]
    OEM7,
    #[cfg(for_windows)]
    OEM8,
    #[cfg(for_windows)]
    OEMAttn,
    #[cfg(for_windows)]
    OEMAuto,
    #[cfg(for_windows)]
    OEMAx,
    #[cfg(for_windows)]
    OEMBacktab,
    #[cfg(for_windows)]
    OEMClear,
    #[cfg(for_windows)]
    OEMComma,
    #[cfg(for_windows)]
    OEMCopy,
    #[cfg(for_windows)]
    OEMCusel,
    #[cfg(for_windows)]
    OEMEnlw,
    #[cfg(for_windows)]
    OEMFinish,
    #[cfg(for_windows)]
    OEMFJJisho,
    #[cfg(for_windows)]
    OEMFJLoya,
    #[cfg(for_windows)]
    OEMFJMasshou,
    #[cfg(for_windows)]
    OEMFJRoya,
    #[cfg(for_windows)]
    OEMFJTouroku,
    #[cfg(for_windows)]
    OEMJump,
    #[cfg(for_windows)]
    OEMMinus,
    #[cfg(for_windows)]
    OEMNECEqual,
    #[cfg(for_windows)]
    OEMPA1,
    #[cfg(for_windows)]
    OEMPA2,
    #[cfg(for_windows)]
    OEMPA3,
    #[cfg(for_windows)]
    OEMPeriod,
    #[cfg(for_windows)]
    OEMPlus,
    #[cfg(for_windows)]
    OEMReset,
    #[cfg(for_windows)]
    OEMWsctrl,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    #[cfg(for_windows)]
    PA1,
    #[cfg(for_windows)]
    Packet,
    /// page down key
    PageDown,
    /// page up key
    PageUp,
     #[cfg(any(for_windows, for_linux))]
    Pause,
    #[cfg(for_windows)]
    Play,
    #[cfg(for_macos)]
    Power,
     #[cfg(any(for_windows, for_linux))]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Print,
    /// Take a screenshot
     #[cfg(any(for_windows, for_linux))]
    #[doc(alias = "Print")]
    #[doc(alias = "Snapshot")]
    PrintScr,
    #[cfg(for_windows)]
    Processkey,
    #[cfg(for_windows)]
    RButton,
    #[cfg(for_macos)]
    RCommand,
    RControl,
    #[cfg(for_linux)]
    Redo,
    /// return key
    Return,
    /// right arrow key
    RightArrow,
    #[cfg(for_windows)]
    RMenu,
    #[cfg(for_macos)]
    ROption,
    RShift,
    #[cfg(for_windows)]
    RWin,
    #[cfg(for_windows)]
    Scroll,
    #[cfg(for_linux)]
    ScrollLock,
     #[cfg(any(for_windows, for_linux))]
    Select,
    #[cfg(for_linux)]
    ScriptSwitch,
    #[cfg(for_windows)]
    Separator,
    /// shift key
    Shift,
    #[cfg(for_linux)]
    /// Lock shift key
    ShiftLock,
    #[cfg(for_windows)]
    Sleep,
    #[cfg(for_windows)]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Snapshot,
    /// space key
    Space,
    Subtract,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// super key on linux (command key on macOS, windows key on Windows)
    Super,
    #[cfg(for_linux)]
    SysReq,
    /// tab key (tabulator)
    Tab,
    #[cfg(for_linux)]
    Undo,
    /// up arrow key
    UpArrow,
    #[cfg(for_macos)]
    VidMirror,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    #[cfg(for_linux)]
    /// microphone mute toggle on linux
    MicMute,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// windows key on Windows (super key on Linux, command key on macOS)
    Windows,
    #[cfg(for_windows)]
    XButton1,
    #[cfg(for_windows)]
    XButton2,
    #[cfg(for_windows)]
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
            #[cfg(any(for_windows, for_linux))]
            Key::Numlock => Ok(Self::Mod2),
            // The Mod3 modifier is usually unmapped
            // Key::Mod3 => Ok(Self::Mod3),
            Key::Command | Key::Super | Key::Windows | Key::Meta => Ok(Self::Mod4),
            #[cfg(any(for_windows, for_linux))]
            Key::ModeChange => Ok(Self::Mod5),
            _ => Err("not a modifier key"),
        }
    }
}
