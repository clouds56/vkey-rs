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

use super::Key;

#[allow(deprecated)]
/// Converts a Key to a Virtual Key
impl TryFrom<Key> for VIRTUAL_KEY {
    type Error = &'static str;

    #[allow(clippy::too_many_lines)]
    fn try_from(key: Key) -> Result<Self, Self::Error> {
        use KeyboardAndMouse::{
            VIRTUAL_KEY, VK__none_, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9,
            VK_A, VK_ABNT_C1, VK_ABNT_C2, VK_ACCEPT, VK_ADD, VK_APPS, VK_ATTN, VK_B, VK_BACK,
            VK_BROWSER_BACK, VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME,
            VK_BROWSER_REFRESH, VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_C, VK_CANCEL, VK_CAPITAL,
            VK_CLEAR, VK_CONTROL, VK_CONVERT, VK_CRSEL, VK_D, VK_DBE_ALPHANUMERIC,
            VK_DBE_CODEINPUT, VK_DBE_DBCSCHAR, VK_DBE_DETERMINESTRING,
            VK_DBE_ENTERDLGCONVERSIONMODE, VK_DBE_ENTERIMECONFIGMODE, VK_DBE_ENTERWORDREGISTERMODE,
            VK_DBE_FLUSHSTRING, VK_DBE_HIRAGANA, VK_DBE_KATAKANA, VK_DBE_NOCODEINPUT,
            VK_DBE_NOROMAN, VK_DBE_ROMAN, VK_DBE_SBCSCHAR, VK_DECIMAL, VK_DELETE, VK_DIVIDE,
            VK_DOWN, VK_E, VK_END, VK_EREOF, VK_ESCAPE, VK_EXECUTE, VK_EXSEL, VK_F, VK_F1, VK_F2,
            VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12, VK_F13,
            VK_F14, VK_F15, VK_F16, VK_F17, VK_F18, VK_F19, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24,
            VK_FINAL, VK_G, VK_GAMEPAD_A, VK_GAMEPAD_B, VK_GAMEPAD_DPAD_DOWN, VK_GAMEPAD_DPAD_LEFT,
            VK_GAMEPAD_DPAD_RIGHT, VK_GAMEPAD_DPAD_UP, VK_GAMEPAD_LEFT_SHOULDER,
            VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON, VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
            VK_GAMEPAD_LEFT_THUMBSTICK_LEFT, VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
            VK_GAMEPAD_LEFT_THUMBSTICK_UP, VK_GAMEPAD_LEFT_TRIGGER, VK_GAMEPAD_MENU,
            VK_GAMEPAD_RIGHT_SHOULDER, VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
            VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN, VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
            VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT, VK_GAMEPAD_RIGHT_THUMBSTICK_UP,
            VK_GAMEPAD_RIGHT_TRIGGER, VK_GAMEPAD_VIEW, VK_GAMEPAD_X, VK_GAMEPAD_Y, VK_H,
            VK_HANGEUL, VK_HANGUL, VK_HANJA, VK_HELP, VK_HOME, VK_I, VK_ICO_00, VK_ICO_CLEAR,
            VK_ICO_HELP, VK_IME_OFF, VK_IME_ON, VK_INSERT, VK_J, VK_JUNJA, VK_K, VK_KANA, VK_KANJI,
            VK_L, VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT,
            VK_LBUTTON, VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON,
            VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_MENU,
            VK_MODECHANGE, VK_MULTIPLY, VK_N, VK_NAVIGATION_ACCEPT, VK_NAVIGATION_CANCEL,
            VK_NAVIGATION_DOWN, VK_NAVIGATION_LEFT, VK_NAVIGATION_MENU, VK_NAVIGATION_RIGHT,
            VK_NAVIGATION_UP, VK_NAVIGATION_VIEW, VK_NEXT, VK_NONAME, VK_NONCONVERT, VK_NUMLOCK,
            VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6,
            VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4,
            VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_102, VK_OEM_ATTN, VK_OEM_AUTO,
            VK_OEM_AX, VK_OEM_BACKTAB, VK_OEM_CLEAR, VK_OEM_COMMA, VK_OEM_COPY, VK_OEM_CUSEL,
            VK_OEM_ENLW, VK_OEM_FINISH, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA, VK_OEM_FJ_MASSHOU,
            VK_OEM_FJ_ROYA, VK_OEM_FJ_TOUROKU, VK_OEM_JUMP, VK_OEM_MINUS, VK_OEM_NEC_EQUAL,
            VK_OEM_PA1, VK_OEM_PA2, VK_OEM_PA3, VK_OEM_PERIOD, VK_OEM_PLUS, VK_OEM_RESET,
            VK_OEM_WSCTRL, VK_P, VK_PA1, VK_PACKET, VK_PAUSE, VK_PLAY, VK_PRINT, VK_PRIOR,
            VK_PROCESSKEY, VK_Q, VK_R, VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU,
            VK_RSHIFT, VK_RWIN, VK_S, VK_SCROLL, VK_SELECT, VK_SEPARATOR, VK_SHIFT, VK_SLEEP,
            VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V, VK_VOLUME_DOWN,
            VK_VOLUME_MUTE, VK_VOLUME_UP, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2, VK_Y, VK_Z,
            VK_ZOOM,
        };

        let vk = match key {
            Key::Num0 => VK_0,
            Key::Num1 => VK_1,
            Key::Num2 => VK_2,
            Key::Num3 => VK_3,
            Key::Num4 => VK_4,
            Key::Num5 => VK_5,
            Key::Num6 => VK_6,
            Key::Num7 => VK_7,
            Key::Num8 => VK_8,
            Key::Num9 => VK_9,
            Key::A => VK_A,
            Key::B => VK_B,
            Key::C => VK_C,
            Key::D => VK_D,
            Key::E => VK_E,
            Key::F => VK_F,
            Key::G => VK_G,
            Key::H => VK_H,
            Key::I => VK_I,
            Key::J => VK_J,
            Key::K => VK_K,
            Key::L => VK_L,
            Key::M => VK_M,
            Key::N => VK_N,
            Key::O => VK_O,
            Key::P => VK_P,
            Key::Q => VK_Q,
            Key::R => VK_R,
            Key::S => VK_S,
            Key::T => VK_T,
            Key::U => VK_U,
            Key::V => VK_V,
            Key::W => VK_W,
            Key::X => VK_X,
            Key::Y => VK_Y,
            Key::Z => VK_Z,
            Key::AbntC1 => VK_ABNT_C1,
            Key::AbntC2 => VK_ABNT_C2,
            Key::Accept => VK_ACCEPT,
            Key::Add => VK_ADD,
            Key::Alt | Key::Option => VK_MENU,
            Key::Apps => VK_APPS,
            Key::Attn => VK_ATTN,
            Key::Backspace => VK_BACK,
            Key::BrowserBack => VK_BROWSER_BACK,
            Key::BrowserFavorites => VK_BROWSER_FAVORITES,
            Key::BrowserForward => VK_BROWSER_FORWARD,
            Key::BrowserHome => VK_BROWSER_HOME,
            Key::BrowserRefresh => VK_BROWSER_REFRESH,
            Key::BrowserSearch => VK_BROWSER_SEARCH,
            Key::BrowserStop => VK_BROWSER_STOP,
            Key::Cancel => VK_CANCEL,
            Key::CapsLock => VK_CAPITAL,
            Key::Clear => VK_CLEAR,
            Key::Control => VK_CONTROL,
            Key::Convert => VK_CONVERT,
            Key::Crsel => VK_CRSEL,
            Key::DBEAlphanumeric => VK_DBE_ALPHANUMERIC,
            Key::DBECodeinput => VK_DBE_CODEINPUT,
            Key::DBEDetermineString => VK_DBE_DETERMINESTRING,
            Key::DBEEnterDLGConversionMode => VK_DBE_ENTERDLGCONVERSIONMODE,
            Key::DBEEnterIMEConfigMode => VK_DBE_ENTERIMECONFIGMODE,
            Key::DBEEnterWordRegisterMode => VK_DBE_ENTERWORDREGISTERMODE,
            Key::DBEFlushString => VK_DBE_FLUSHSTRING,
            Key::DBEHiragana => VK_DBE_HIRAGANA,
            Key::DBEKatakana => VK_DBE_KATAKANA,
            Key::DBENoCodepoint => VK_DBE_NOCODEINPUT,
            Key::DBENoRoman => VK_DBE_NOROMAN,
            Key::DBERoman => VK_DBE_ROMAN,
            Key::DBESBCSChar => VK_DBE_SBCSCHAR,
            Key::DBESChar => VK_DBE_DBCSCHAR,
            Key::Decimal => VK_DECIMAL,
            Key::Delete => VK_DELETE,
            Key::Divide => VK_DIVIDE,
            Key::DownArrow => VK_DOWN,
            Key::End => VK_END,
            Key::Ereof => VK_EREOF,
            Key::Escape => VK_ESCAPE,
            Key::Execute => VK_EXECUTE,
            Key::Exsel => VK_EXSEL,
            Key::F1 => VK_F1,
            Key::F2 => VK_F2,
            Key::F3 => VK_F3,
            Key::F4 => VK_F4,
            Key::F5 => VK_F5,
            Key::F6 => VK_F6,
            Key::F7 => VK_F7,
            Key::F8 => VK_F8,
            Key::F9 => VK_F9,
            Key::F10 => VK_F10,
            Key::F11 => VK_F11,
            Key::F12 => VK_F12,
            Key::F13 => VK_F13,
            Key::F14 => VK_F14,
            Key::F15 => VK_F15,
            Key::F16 => VK_F16,
            Key::F17 => VK_F17,
            Key::F18 => VK_F18,
            Key::F19 => VK_F19,
            Key::F20 => VK_F20,
            Key::F21 => VK_F21,
            Key::F22 => VK_F22,
            Key::F23 => VK_F23,
            Key::F24 => VK_F24,
            Key::Final => VK_FINAL,
            Key::GamepadA => VK_GAMEPAD_A,
            Key::GamepadB => VK_GAMEPAD_B,
            Key::GamepadDPadDown => VK_GAMEPAD_DPAD_DOWN,
            Key::GamepadDPadLeft => VK_GAMEPAD_DPAD_LEFT,
            Key::GamepadDPadRight => VK_GAMEPAD_DPAD_RIGHT,
            Key::GamepadDPadUp => VK_GAMEPAD_DPAD_UP,
            Key::GamepadLeftShoulder => VK_GAMEPAD_LEFT_SHOULDER,
            Key::GamepadLeftThumbstickButton => VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON,
            Key::GamepadLeftThumbstickDown => VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
            Key::GamepadLeftThumbstickLeft => VK_GAMEPAD_LEFT_THUMBSTICK_LEFT,
            Key::GamepadLeftThumbstickRight => VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
            Key::GamepadLeftThumbstickUp => VK_GAMEPAD_LEFT_THUMBSTICK_UP,
            Key::GamepadLeftTrigger => VK_GAMEPAD_LEFT_TRIGGER,
            Key::GamepadMenu => VK_GAMEPAD_MENU,
            Key::GamepadRightShoulder => VK_GAMEPAD_RIGHT_SHOULDER,
            Key::GamepadRightThumbstickButton => VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
            Key::GamepadRightThumbstickDown => VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN,
            Key::GamepadRightThumbstickLeft => VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
            Key::GamepadRightThumbstickRight => VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT,
            Key::GamepadRightThumbstickUp => VK_GAMEPAD_RIGHT_THUMBSTICK_UP,
            Key::GamepadRightTrigger => VK_GAMEPAD_RIGHT_TRIGGER,
            Key::GamepadView => VK_GAMEPAD_VIEW,
            Key::GamepadX => VK_GAMEPAD_X,
            Key::GamepadY => VK_GAMEPAD_Y,
            Key::Hangeul => VK_HANGEUL,
            Key::Hangul => VK_HANGUL,
            Key::Hanja => VK_HANJA,
            Key::Help => VK_HELP,
            Key::Home => VK_HOME,
            Key::Ico00 => VK_ICO_00,
            Key::IcoClear => VK_ICO_CLEAR,
            Key::IcoHelp => VK_ICO_HELP,
            Key::IMEOff => VK_IME_OFF,
            Key::IMEOn => VK_IME_ON,
            Key::Insert => VK_INSERT,
            Key::Junja => VK_JUNJA,
            Key::Kana => VK_KANA,
            Key::Kanji => VK_KANJI,
            Key::LaunchApp1 => VK_LAUNCH_APP1,
            Key::LaunchApp2 => VK_LAUNCH_APP2,
            Key::LaunchMail => VK_LAUNCH_MAIL,
            Key::LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT,
            Key::LButton => VK_LBUTTON,
            Key::LControl => VK_LCONTROL,
            Key::LeftArrow => VK_LEFT,
            Key::LMenu => VK_LMENU,
            Key::LShift => VK_LSHIFT,
            Key::MButton => VK_MBUTTON,
            Key::MediaNextTrack => VK_MEDIA_NEXT_TRACK,
            Key::MediaPlayPause => VK_MEDIA_PLAY_PAUSE,
            Key::MediaPrevTrack => VK_MEDIA_PREV_TRACK,
            Key::MediaStop => VK_MEDIA_STOP,
            Key::ModeChange => VK_MODECHANGE,
            Key::Multiply => VK_MULTIPLY,
            Key::NavigationAccept => VK_NAVIGATION_ACCEPT,
            Key::NavigationCancel => VK_NAVIGATION_CANCEL,
            Key::NavigationDown => VK_NAVIGATION_DOWN,
            Key::NavigationLeft => VK_NAVIGATION_LEFT,
            Key::NavigationMenu => VK_NAVIGATION_MENU,
            Key::NavigationRight => VK_NAVIGATION_RIGHT,
            Key::NavigationUp => VK_NAVIGATION_UP,
            Key::NavigationView => VK_NAVIGATION_VIEW,
            Key::NoName => VK_NONAME,
            Key::NonConvert => VK_NONCONVERT,
            Key::None => VK__none_,
            Key::Numlock => VK_NUMLOCK,
            Key::Numpad0 => VK_NUMPAD0,
            Key::Numpad1 => VK_NUMPAD1,
            Key::Numpad2 => VK_NUMPAD2,
            Key::Numpad3 => VK_NUMPAD3,
            Key::Numpad4 => VK_NUMPAD4,
            Key::Numpad5 => VK_NUMPAD5,
            Key::Numpad6 => VK_NUMPAD6,
            Key::Numpad7 => VK_NUMPAD7,
            Key::Numpad8 => VK_NUMPAD8,
            Key::Numpad9 => VK_NUMPAD9,
            Key::OEM1 => VK_OEM_1,
            Key::OEM102 => VK_OEM_102,
            Key::OEM2 => VK_OEM_2,
            Key::OEM3 => VK_OEM_3,
            Key::OEM4 => VK_OEM_4,
            Key::OEM5 => VK_OEM_5,
            Key::OEM6 => VK_OEM_6,
            Key::OEM7 => VK_OEM_7,
            Key::OEM8 => VK_OEM_8,
            Key::OEMAttn => VK_OEM_ATTN,
            Key::OEMAuto => VK_OEM_AUTO,
            Key::OEMAx => VK_OEM_AX,
            Key::OEMBacktab => VK_OEM_BACKTAB,
            Key::OEMClear => VK_OEM_CLEAR,
            Key::OEMComma => VK_OEM_COMMA,
            Key::OEMCopy => VK_OEM_COPY,
            Key::OEMCusel => VK_OEM_CUSEL,
            Key::OEMEnlw => VK_OEM_ENLW,
            Key::OEMFinish => VK_OEM_FINISH,
            Key::OEMFJJisho => VK_OEM_FJ_JISHO,
            Key::OEMFJLoya => VK_OEM_FJ_LOYA,
            Key::OEMFJMasshou => VK_OEM_FJ_MASSHOU,
            Key::OEMFJRoya => VK_OEM_FJ_ROYA,
            Key::OEMFJTouroku => VK_OEM_FJ_TOUROKU,
            Key::OEMJump => VK_OEM_JUMP,
            Key::OEMMinus => VK_OEM_MINUS,
            Key::OEMNECEqual => VK_OEM_NEC_EQUAL,
            Key::OEMPA1 => VK_OEM_PA1,
            Key::OEMPA2 => VK_OEM_PA2,
            Key::OEMPA3 => VK_OEM_PA3,
            Key::OEMPeriod => VK_OEM_PERIOD,
            Key::OEMPlus => VK_OEM_PLUS,
            Key::OEMReset => VK_OEM_RESET,
            Key::OEMWsctrl => VK_OEM_WSCTRL,
            Key::PA1 => VK_PA1,
            Key::Packet => VK_PACKET,
            Key::PageDown => VK_NEXT,
            Key::PageUp => VK_PRIOR,
            Key::Pause => VK_PAUSE,
            Key::Play => VK_PLAY,
            Key::Print => VK_PRINT,
            Key::PrintScr | Key::Snapshot => VK_SNAPSHOT,
            Key::Processkey => VK_PROCESSKEY,
            Key::RButton => VK_RBUTTON,
            Key::RControl => VK_RCONTROL,
            Key::Return => VK_RETURN,
            Key::RightArrow => VK_RIGHT,
            Key::RMenu => VK_RMENU,
            Key::RShift => VK_RSHIFT,
            Key::RWin => VK_RWIN,
            Key::Scroll => VK_SCROLL,
            Key::Select => VK_SELECT,
            Key::Separator => VK_SEPARATOR,
            Key::Shift => VK_SHIFT,
            Key::Sleep => VK_SLEEP,
            Key::Space => VK_SPACE,
            Key::Subtract => VK_SUBTRACT,
            Key::Tab => VK_TAB,
            Key::UpArrow => VK_UP,
            Key::VolumeDown => VK_VOLUME_DOWN,
            Key::VolumeMute => VK_VOLUME_MUTE,
            Key::VolumeUp => VK_VOLUME_UP,
            Key::XButton1 => VK_XBUTTON1,
            Key::XButton2 => VK_XBUTTON2,
            Key::Zoom => VK_ZOOM,
            Key::Unicode(c) => 'unicode_handling: {
                // Handle special characters separately
                match c {
                    '\n' => break 'unicode_handling VK_RETURN,

                    '\r' => { // TODO: What is the correct key to type here?
                        // break 'unicode_handling VK_,
                    }
                    '\t' => break 'unicode_handling VK_TAB,
                    '\0' => {
                        return Err("Invalid mapping");
                    }
                    _ => (),
                }

                #[cfg(all(feature = "windows", target_os = "windows"))]
                {
                    let layout = unsafe {
                        windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout(0)
                    };

                    let mut buffer = [0; 2];
                    let utf16_surrogates = c.encode_utf16(&mut buffer);
                    if utf16_surrogates.len() != 1 {
                        return Err("Character can't be mapped to only one virtual key");
                    }
                    // Translate a character to the corresponding virtual-key code and shift state.
                    // If the function succeeds, the low-order byte of the return value contains the
                    // virtual-key code and the high-order byte contains the shift state, which can
                    // be a combination of the following flag bits. If the function finds no key
                    // that translates to the passed character code, both the low-order and
                    // high-order bytes contain -1
                    let vk = unsafe {
                        windows::Win32::UI::Input::KeyboardAndMouse::VkKeyScanExW(
                            utf16_surrogates[0],
                            layout,
                        )
                    };
                    // TODO: Check if the condition should be <=
                    if vk < 0 {
                        return Err("Character can't be mapped to virtual key");
                    }
                    break 'unicode_handling VIRTUAL_KEY(vk as u16)
                }
                #[allow(unreachable_code)]
                return Err("Character can't be mapped to virtual key");
            }
            Key::Other(v) => {
                let Ok(v) = u16::try_from(v) else {
                    return Err("virtual keycodes on Windows have to fit into u16");
                };
                VIRTUAL_KEY(v)
            }
            Key::Super | Key::Command | Key::Windows | Key::Meta | Key::LWin => VK_LWIN,
            #[cfg(any(feature = "target_macos", feature = "target_linux"))]
            _ => return Err("Invalid mapping"),
        };

        Ok(vk)
    }
}
