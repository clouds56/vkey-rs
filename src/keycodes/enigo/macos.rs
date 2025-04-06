use super::Key;



#[cfg(all(target_os = "macos", feature = "macos"))]
mod _impl {
    use core_foundation::{
        array::CFIndex,
        base::{OSStatus, UInt16, UInt32, UInt8},
        data::{CFDataGetBytePtr, CFDataRef},
        string::{CFStringRef, UniChar},
    };
    use core_graphics::event::CGKeyCode;

    #[repr(C)]
    struct __TISInputSource;
    type TISInputSourceRef = *const __TISInputSource;

    #[allow(non_upper_case_globals)]
    const kUCKeyTranslateNoDeadKeysBit: CFIndex = 0; // Previously was always u32. Change it back if there are bugs

    #[allow(improper_ctypes)]
    #[link(name = "Carbon", kind = "framework")]
    unsafe extern "C" {
        fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
        fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
        fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> TISInputSourceRef;

        #[allow(non_upper_case_globals)]
        static kTISPropertyUnicodeKeyLayoutData: CFStringRef;

        #[allow(non_snake_case)]
        fn TISGetInputSourceProperty(
            inputSource: TISInputSourceRef,
            propertyKey: CFStringRef,
        ) -> CFDataRef;

        #[allow(non_snake_case)]
        fn UCKeyTranslate(
            keyLayoutPtr: *const UInt8, //*const UCKeyboardLayout,
            virtualKeyCode: UInt16,
            keyAction: UInt16,
            modifierKeyState: UInt32,
            keyboardType: UInt32,
            keyTranslateOptions: CFIndex,
            deadKeyState: *mut UInt32,
            maxStringLength: CFIndex,
            actualStringLength: *mut CFIndex,
            unicodeString: *mut UniChar,
        ) -> OSStatus;

        fn LMGetKbdType() -> UInt8;
    }

    pub fn get_layoutdependent_keycode(string: &str) -> CGKeyCode {
        let mut pressed_keycode = 0;

        // loop through every keycode (0 - 127)
        for keycode in 0..128 {
            // no modifier
            if let Ok(key_string) = keycode_to_string(keycode, 0x100) {
                // debug!("{:?}", string);
                if string == key_string {
                    pressed_keycode = keycode;
                }
            }

            // shift modifier
            if let Ok(key_string) = keycode_to_string(keycode, 0x20102) {
                // debug!("{:?}", string);
                if string == key_string {
                    pressed_keycode = keycode;
                }
            }

            // alt modifier
            // if let Some(string) = keycode_to_string(keycode, 0x80120) {
            //     debug!("{:?}", string);
            // }
            // alt + shift modifier
            // if let Some(string) = keycode_to_string(keycode, 0xa0122) {
            //     debug!("{:?}", string);
            // }
        }

        pressed_keycode
    }

    fn keycode_to_string(keycode: u16, modifier: u32) -> Result<String, String> {
        let mut current_keyboard = unsafe { TISCopyCurrentKeyboardInputSource() };
        let mut layout_data =
            unsafe { TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData) };
        if layout_data.is_null() {
            // debug!("TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData) returned NULL");
            // TISGetInputSourceProperty returns null with some keyboard layout.
            // Using TISCopyCurrentKeyboardLayoutInputSource to fix NULL return.
            // See also: https://github.com/microsoft/node-native-keymap/blob/089d802efd387df4dce1f0e31898c66e28b3f67f/src/keyboard_mac.mm#L90
            current_keyboard = unsafe { TISCopyCurrentKeyboardLayoutInputSource() };
            layout_data = unsafe {
                TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData)
            };
            if layout_data.is_null() {
                // debug!("TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData) returned NULL again");
                current_keyboard = unsafe { TISCopyCurrentASCIICapableKeyboardLayoutInputSource() };
                layout_data = unsafe {
                    TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData)
                };
                debug_assert!(!layout_data.is_null());
                // debug!("Using layout of the TISCopyCurrentASCIICapableKeyboardLayoutInputSource");
            }
        }

        let keyboard_layout = unsafe { CFDataGetBytePtr(layout_data) };

        let mut keys_down: UInt32 = 0;
        let mut chars: [UniChar; 1] = [0];
        let mut real_length = 0;
        let status = unsafe {
            UCKeyTranslate(
                keyboard_layout,
                keycode,
                3, // kUCKeyActionDisplay = 3
                modifier,
                LMGetKbdType() as u32,
                kUCKeyTranslateNoDeadKeysBit,
                &mut keys_down,
                chars.len() as CFIndex,
                &mut real_length,
                chars.as_mut_ptr(),
            )
        };

        if status != 0 {
            // error!("UCKeyTranslate failed with status: {status}");
            return Err(format!("OSStatus error: {status}"));
        }

        let utf16_slice = &chars[..real_length as usize];
        String::from_utf16(utf16_slice).map_err(|e| {
            // error!("UTF-16 to String converstion failed: {e:?}");
            format!("FromUtf16Error: {e}")
        })
    }

}

#[allow(deprecated)]
/// Converts a `Key` to a `CGKeyCode`
impl TryFrom<Key> for CGKeyCode {
    type Error = ();

    #[allow(clippy::too_many_lines)]
    fn try_from(key: Key) -> Result<Self, Self::Error> {
        // A list of names is available at:
        // https://docs.rs/core-graphics/latest/core_graphics/event/struct.KeyCode.html
        // https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.13.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h
        let key = match key {
            Key::Alt | Key::Option => KeyCode::OPTION,
            Key::Backspace => KeyCode::DELETE,
            Key::CapsLock => KeyCode::CAPS_LOCK,
            Key::Control | Key::LControl => KeyCode::CONTROL,
            Key::Delete => KeyCode::FORWARD_DELETE,
            Key::DownArrow => KeyCode::DOWN_ARROW,
            Key::End => KeyCode::END,
            Key::Escape => KeyCode::ESCAPE,
            Key::F1 => KeyCode::F1,
            Key::F2 => KeyCode::F2,
            Key::F3 => KeyCode::F3,
            Key::F4 => KeyCode::F4,
            Key::F5 => KeyCode::F5,
            Key::F6 => KeyCode::F6,
            Key::F7 => KeyCode::F7,
            Key::F8 => KeyCode::F8,
            Key::F9 => KeyCode::F9,
            Key::F10 => KeyCode::F10,
            Key::F11 => KeyCode::F11,
            Key::F12 => KeyCode::F12,
            Key::F13 => KeyCode::F13,
            Key::F14 => KeyCode::F14,
            Key::F15 => KeyCode::F15,
            Key::F16 => KeyCode::F16,
            Key::F17 => KeyCode::F17,
            Key::F18 => KeyCode::F18,
            Key::F19 => KeyCode::F19,
            Key::F20 => KeyCode::F20,
            Key::Function => KeyCode::FUNCTION,
            Key::Help => KeyCode::HELP,
            Key::Home => KeyCode::HOME,
            Key::Launchpad => 131,
            Key::LeftArrow => KeyCode::LEFT_ARROW,
            Key::MissionControl => 160,
            Key::PageDown => KeyCode::PAGE_DOWN,
            Key::PageUp => KeyCode::PAGE_UP,
            Key::RCommand => KeyCode::RIGHT_COMMAND,
            Key::RControl => KeyCode::RIGHT_CONTROL,
            Key::Return => KeyCode::RETURN,
            Key::RightArrow => KeyCode::RIGHT_ARROW,
            Key::RShift => KeyCode::RIGHT_SHIFT,
            Key::ROption => KeyCode::RIGHT_OPTION,
            Key::Shift | Key::LShift => KeyCode::SHIFT,
            Key::Space => KeyCode::SPACE,
            Key::Tab => KeyCode::TAB,
            Key::UpArrow => KeyCode::UP_ARROW,
            Key::VolumeDown => KeyCode::VOLUME_DOWN,
            Key::VolumeUp => KeyCode::VOLUME_UP,
            Key::VolumeMute => KeyCode::MUTE,
            Key::Unicode(c) => {
                #[cfg(not(all(target_os = "macos", feature = "macos")))]
                {
                    let _ = c; return Err(());
                }
                #[cfg(all(target_os = "macos", feature = "macos"))]
                _impl::get_layoutdependent_keycode(&c.to_string())
            },
            Key::Other(v) => {
                let Ok(v) = u16::try_from(v) else {
                    return Err(());
                };
                v
            }
            Key::Super | Key::Command | Key::Windows | Key::Meta => KeyCode::COMMAND,
            Key::BrightnessDown
            | Key::BrightnessUp
            | Key::ContrastUp
            | Key::ContrastDown
            | Key::Eject
            | Key::IlluminationDown
            | Key::IlluminationUp
            | Key::IlluminationToggle
            | Key::LaunchPanel
            | Key::MediaFast
            | Key::MediaNextTrack
            | Key::MediaPlayPause
            | Key::MediaPrevTrack
            | Key::MediaRewind
            | Key::Power
            | Key::VidMirror => return Err(()),
            #[cfg(any(feature = "target_macos", feature = "target_windows"))]
            _ => return Err(()),
        };
        Ok(key.into())
    }
}
