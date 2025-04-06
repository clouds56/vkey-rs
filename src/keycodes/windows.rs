#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/**
 * https://github.com/microsoft/windows-rs/blob/8d40a84d0428ca23ee43cc320742d92d78db04c1/crates/libs/sys/src/Windows/Win32/UI/Input/KeyboardAndMouse/mod.rs#L446-L858
 *
 *    MIT License
 *
 *    Copyright (c) Microsoft Corporation.
 *
 *    Permission is hereby granted, free of charge, to any person obtaining a copy
 *    of this software and associated documentation files (the "Software"), to deal
 *    in the Software without restriction, including without limitation the rights
 *    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *    copies of the Software, and to permit persons to whom the Software is
 *    furnished to do so, subject to the following conditions:
 *
 *    The above copyright notice and this permission notice shall be included in all
 *    copies or substantial portions of the Software.
 *
 *    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *    SOFTWARE
 */

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_KEY(pub u16);
pub const VK_0: VIRTUAL_KEY = VIRTUAL_KEY(48u16);
pub const VK_1: VIRTUAL_KEY = VIRTUAL_KEY(49u16);
pub const VK_2: VIRTUAL_KEY = VIRTUAL_KEY(50u16);
pub const VK_3: VIRTUAL_KEY = VIRTUAL_KEY(51u16);
pub const VK_4: VIRTUAL_KEY = VIRTUAL_KEY(52u16);
pub const VK_5: VIRTUAL_KEY = VIRTUAL_KEY(53u16);
pub const VK_6: VIRTUAL_KEY = VIRTUAL_KEY(54u16);
pub const VK_7: VIRTUAL_KEY = VIRTUAL_KEY(55u16);
pub const VK_8: VIRTUAL_KEY = VIRTUAL_KEY(56u16);
pub const VK_9: VIRTUAL_KEY = VIRTUAL_KEY(57u16);
pub const VK_A: VIRTUAL_KEY = VIRTUAL_KEY(65u16);
pub const VK_ABNT_C1: VIRTUAL_KEY = VIRTUAL_KEY(193u16);
pub const VK_ABNT_C2: VIRTUAL_KEY = VIRTUAL_KEY(194u16);
pub const VK_ACCEPT: VIRTUAL_KEY = VIRTUAL_KEY(30u16);
pub const VK_ADD: VIRTUAL_KEY = VIRTUAL_KEY(107u16);
pub const VK_APPS: VIRTUAL_KEY = VIRTUAL_KEY(93u16);
pub const VK_ATTN: VIRTUAL_KEY = VIRTUAL_KEY(246u16);
pub const VK_B: VIRTUAL_KEY = VIRTUAL_KEY(66u16);
pub const VK_BACK: VIRTUAL_KEY = VIRTUAL_KEY(8u16);
pub const VK_BROWSER_BACK: VIRTUAL_KEY = VIRTUAL_KEY(166u16);
pub const VK_BROWSER_FAVORITES: VIRTUAL_KEY = VIRTUAL_KEY(171u16);
pub const VK_BROWSER_FORWARD: VIRTUAL_KEY = VIRTUAL_KEY(167u16);
pub const VK_BROWSER_HOME: VIRTUAL_KEY = VIRTUAL_KEY(172u16);
pub const VK_BROWSER_REFRESH: VIRTUAL_KEY = VIRTUAL_KEY(168u16);
pub const VK_BROWSER_SEARCH: VIRTUAL_KEY = VIRTUAL_KEY(170u16);
pub const VK_BROWSER_STOP: VIRTUAL_KEY = VIRTUAL_KEY(169u16);
pub const VK_C: VIRTUAL_KEY = VIRTUAL_KEY(67u16);
pub const VK_CANCEL: VIRTUAL_KEY = VIRTUAL_KEY(3u16);
pub const VK_CAPITAL: VIRTUAL_KEY = VIRTUAL_KEY(20u16);
pub const VK_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(12u16);
pub const VK_CONTROL: VIRTUAL_KEY = VIRTUAL_KEY(17u16);
pub const VK_CONVERT: VIRTUAL_KEY = VIRTUAL_KEY(28u16);
pub const VK_CRSEL: VIRTUAL_KEY = VIRTUAL_KEY(247u16);
pub const VK_D: VIRTUAL_KEY = VIRTUAL_KEY(68u16);
pub const VK_DBE_ALPHANUMERIC: VIRTUAL_KEY = VIRTUAL_KEY(240u16);
pub const VK_DBE_CODEINPUT: VIRTUAL_KEY = VIRTUAL_KEY(250u16);
pub const VK_DBE_DBCSCHAR: VIRTUAL_KEY = VIRTUAL_KEY(244u16);
pub const VK_DBE_DETERMINESTRING: VIRTUAL_KEY = VIRTUAL_KEY(252u16);
pub const VK_DBE_ENTERDLGCONVERSIONMODE: VIRTUAL_KEY = VIRTUAL_KEY(253u16);
pub const VK_DBE_ENTERIMECONFIGMODE: VIRTUAL_KEY = VIRTUAL_KEY(248u16);
pub const VK_DBE_ENTERWORDREGISTERMODE: VIRTUAL_KEY = VIRTUAL_KEY(247u16);
pub const VK_DBE_FLUSHSTRING: VIRTUAL_KEY = VIRTUAL_KEY(249u16);
pub const VK_DBE_HIRAGANA: VIRTUAL_KEY = VIRTUAL_KEY(242u16);
pub const VK_DBE_KATAKANA: VIRTUAL_KEY = VIRTUAL_KEY(241u16);
pub const VK_DBE_NOCODEINPUT: VIRTUAL_KEY = VIRTUAL_KEY(251u16);
pub const VK_DBE_NOROMAN: VIRTUAL_KEY = VIRTUAL_KEY(246u16);
pub const VK_DBE_ROMAN: VIRTUAL_KEY = VIRTUAL_KEY(245u16);
pub const VK_DBE_SBCSCHAR: VIRTUAL_KEY = VIRTUAL_KEY(243u16);
pub const VK_DECIMAL: VIRTUAL_KEY = VIRTUAL_KEY(110u16);
pub const VK_DELETE: VIRTUAL_KEY = VIRTUAL_KEY(46u16);
pub const VK_DIVIDE: VIRTUAL_KEY = VIRTUAL_KEY(111u16);
pub const VK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(40u16);
pub const VK_E: VIRTUAL_KEY = VIRTUAL_KEY(69u16);
pub const VK_END: VIRTUAL_KEY = VIRTUAL_KEY(35u16);
pub const VK_EREOF: VIRTUAL_KEY = VIRTUAL_KEY(249u16);
pub const VK_ESCAPE: VIRTUAL_KEY = VIRTUAL_KEY(27u16);
pub const VK_EXECUTE: VIRTUAL_KEY = VIRTUAL_KEY(43u16);
pub const VK_EXSEL: VIRTUAL_KEY = VIRTUAL_KEY(248u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VK_F {
    pub Vk: u8,
    pub NLSFEProcType: u8,
    pub NLSFEProcCurrent: u8,
    pub NLSFEProcSwitch: u8,
    pub NLSFEProc: [VK_FPARAM; 8],
    pub NLSFEProcAlt: [VK_FPARAM; 8],
}
impl Default for VK_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VK_F: VIRTUAL_KEY = VIRTUAL_KEY(70u16);
pub const VK_F1: VIRTUAL_KEY = VIRTUAL_KEY(112u16);
pub const VK_F10: VIRTUAL_KEY = VIRTUAL_KEY(121u16);
pub const VK_F11: VIRTUAL_KEY = VIRTUAL_KEY(122u16);
pub const VK_F12: VIRTUAL_KEY = VIRTUAL_KEY(123u16);
pub const VK_F13: VIRTUAL_KEY = VIRTUAL_KEY(124u16);
pub const VK_F14: VIRTUAL_KEY = VIRTUAL_KEY(125u16);
pub const VK_F15: VIRTUAL_KEY = VIRTUAL_KEY(126u16);
pub const VK_F16: VIRTUAL_KEY = VIRTUAL_KEY(127u16);
pub const VK_F17: VIRTUAL_KEY = VIRTUAL_KEY(128u16);
pub const VK_F18: VIRTUAL_KEY = VIRTUAL_KEY(129u16);
pub const VK_F19: VIRTUAL_KEY = VIRTUAL_KEY(130u16);
pub const VK_F2: VIRTUAL_KEY = VIRTUAL_KEY(113u16);
pub const VK_F20: VIRTUAL_KEY = VIRTUAL_KEY(131u16);
pub const VK_F21: VIRTUAL_KEY = VIRTUAL_KEY(132u16);
pub const VK_F22: VIRTUAL_KEY = VIRTUAL_KEY(133u16);
pub const VK_F23: VIRTUAL_KEY = VIRTUAL_KEY(134u16);
pub const VK_F24: VIRTUAL_KEY = VIRTUAL_KEY(135u16);
pub const VK_F3: VIRTUAL_KEY = VIRTUAL_KEY(114u16);
pub const VK_F4: VIRTUAL_KEY = VIRTUAL_KEY(115u16);
pub const VK_F5: VIRTUAL_KEY = VIRTUAL_KEY(116u16);
pub const VK_F6: VIRTUAL_KEY = VIRTUAL_KEY(117u16);
pub const VK_F7: VIRTUAL_KEY = VIRTUAL_KEY(118u16);
pub const VK_F8: VIRTUAL_KEY = VIRTUAL_KEY(119u16);
pub const VK_F9: VIRTUAL_KEY = VIRTUAL_KEY(120u16);
pub const VK_FINAL: VIRTUAL_KEY = VIRTUAL_KEY(24u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VK_FPARAM {
    pub NLSFEProcIndex: u8,
    pub NLSFEProcParam: u32,
}
pub const VK_G: VIRTUAL_KEY = VIRTUAL_KEY(71u16);
pub const VK_GAMEPAD_A: VIRTUAL_KEY = VIRTUAL_KEY(195u16);
pub const VK_GAMEPAD_B: VIRTUAL_KEY = VIRTUAL_KEY(196u16);
pub const VK_GAMEPAD_DPAD_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(204u16);
pub const VK_GAMEPAD_DPAD_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(205u16);
pub const VK_GAMEPAD_DPAD_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(206u16);
pub const VK_GAMEPAD_DPAD_UP: VIRTUAL_KEY = VIRTUAL_KEY(203u16);
pub const VK_GAMEPAD_LEFT_SHOULDER: VIRTUAL_KEY = VIRTUAL_KEY(200u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: VIRTUAL_KEY = VIRTUAL_KEY(209u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(212u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(214u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(213u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: VIRTUAL_KEY = VIRTUAL_KEY(211u16);
pub const VK_GAMEPAD_LEFT_TRIGGER: VIRTUAL_KEY = VIRTUAL_KEY(201u16);
pub const VK_GAMEPAD_MENU: VIRTUAL_KEY = VIRTUAL_KEY(207u16);
pub const VK_GAMEPAD_RIGHT_SHOULDER: VIRTUAL_KEY = VIRTUAL_KEY(199u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: VIRTUAL_KEY = VIRTUAL_KEY(210u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(216u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(218u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(217u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: VIRTUAL_KEY = VIRTUAL_KEY(215u16);
pub const VK_GAMEPAD_RIGHT_TRIGGER: VIRTUAL_KEY = VIRTUAL_KEY(202u16);
pub const VK_GAMEPAD_VIEW: VIRTUAL_KEY = VIRTUAL_KEY(208u16);
pub const VK_GAMEPAD_X: VIRTUAL_KEY = VIRTUAL_KEY(197u16);
pub const VK_GAMEPAD_Y: VIRTUAL_KEY = VIRTUAL_KEY(198u16);
pub const VK_H: VIRTUAL_KEY = VIRTUAL_KEY(72u16);
pub const VK_HANGEUL: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_HANGUL: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_HANJA: VIRTUAL_KEY = VIRTUAL_KEY(25u16);
pub const VK_HELP: VIRTUAL_KEY = VIRTUAL_KEY(47u16);
pub const VK_HOME: VIRTUAL_KEY = VIRTUAL_KEY(36u16);
pub const VK_I: VIRTUAL_KEY = VIRTUAL_KEY(73u16);
pub const VK_ICO_00: VIRTUAL_KEY = VIRTUAL_KEY(228u16);
pub const VK_ICO_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(230u16);
pub const VK_ICO_HELP: VIRTUAL_KEY = VIRTUAL_KEY(227u16);
pub const VK_IME_OFF: VIRTUAL_KEY = VIRTUAL_KEY(26u16);
pub const VK_IME_ON: VIRTUAL_KEY = VIRTUAL_KEY(22u16);
pub const VK_INSERT: VIRTUAL_KEY = VIRTUAL_KEY(45u16);
pub const VK_J: VIRTUAL_KEY = VIRTUAL_KEY(74u16);
pub const VK_JUNJA: VIRTUAL_KEY = VIRTUAL_KEY(23u16);
pub const VK_K: VIRTUAL_KEY = VIRTUAL_KEY(75u16);
pub const VK_KANA: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_KANJI: VIRTUAL_KEY = VIRTUAL_KEY(25u16);
pub const VK_L: VIRTUAL_KEY = VIRTUAL_KEY(76u16);
pub const VK_LAUNCH_APP1: VIRTUAL_KEY = VIRTUAL_KEY(182u16);
pub const VK_LAUNCH_APP2: VIRTUAL_KEY = VIRTUAL_KEY(183u16);
pub const VK_LAUNCH_MAIL: VIRTUAL_KEY = VIRTUAL_KEY(180u16);
pub const VK_LAUNCH_MEDIA_SELECT: VIRTUAL_KEY = VIRTUAL_KEY(181u16);
pub const VK_LBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(1u16);
pub const VK_LCONTROL: VIRTUAL_KEY = VIRTUAL_KEY(162u16);
pub const VK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(37u16);
pub const VK_LMENU: VIRTUAL_KEY = VIRTUAL_KEY(164u16);
pub const VK_LSHIFT: VIRTUAL_KEY = VIRTUAL_KEY(160u16);
pub const VK_LWIN: VIRTUAL_KEY = VIRTUAL_KEY(91u16);
pub const VK_M: VIRTUAL_KEY = VIRTUAL_KEY(77u16);
pub const VK_MBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(4u16);
pub const VK_MEDIA_NEXT_TRACK: VIRTUAL_KEY = VIRTUAL_KEY(176u16);
pub const VK_MEDIA_PLAY_PAUSE: VIRTUAL_KEY = VIRTUAL_KEY(179u16);
pub const VK_MEDIA_PREV_TRACK: VIRTUAL_KEY = VIRTUAL_KEY(177u16);
pub const VK_MEDIA_STOP: VIRTUAL_KEY = VIRTUAL_KEY(178u16);
pub const VK_MENU: VIRTUAL_KEY = VIRTUAL_KEY(18u16);
pub const VK_MODECHANGE: VIRTUAL_KEY = VIRTUAL_KEY(31u16);
pub const VK_MULTIPLY: VIRTUAL_KEY = VIRTUAL_KEY(106u16);
pub const VK_N: VIRTUAL_KEY = VIRTUAL_KEY(78u16);
pub const VK_NAVIGATION_ACCEPT: VIRTUAL_KEY = VIRTUAL_KEY(142u16);
pub const VK_NAVIGATION_CANCEL: VIRTUAL_KEY = VIRTUAL_KEY(143u16);
pub const VK_NAVIGATION_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(139u16);
pub const VK_NAVIGATION_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(140u16);
pub const VK_NAVIGATION_MENU: VIRTUAL_KEY = VIRTUAL_KEY(137u16);
pub const VK_NAVIGATION_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(141u16);
pub const VK_NAVIGATION_UP: VIRTUAL_KEY = VIRTUAL_KEY(138u16);
pub const VK_NAVIGATION_VIEW: VIRTUAL_KEY = VIRTUAL_KEY(136u16);
pub const VK_NEXT: VIRTUAL_KEY = VIRTUAL_KEY(34u16);
pub const VK_NONAME: VIRTUAL_KEY = VIRTUAL_KEY(252u16);
pub const VK_NONCONVERT: VIRTUAL_KEY = VIRTUAL_KEY(29u16);
pub const VK_NUMLOCK: VIRTUAL_KEY = VIRTUAL_KEY(144u16);
pub const VK_NUMPAD0: VIRTUAL_KEY = VIRTUAL_KEY(96u16);
pub const VK_NUMPAD1: VIRTUAL_KEY = VIRTUAL_KEY(97u16);
pub const VK_NUMPAD2: VIRTUAL_KEY = VIRTUAL_KEY(98u16);
pub const VK_NUMPAD3: VIRTUAL_KEY = VIRTUAL_KEY(99u16);
pub const VK_NUMPAD4: VIRTUAL_KEY = VIRTUAL_KEY(100u16);
pub const VK_NUMPAD5: VIRTUAL_KEY = VIRTUAL_KEY(101u16);
pub const VK_NUMPAD6: VIRTUAL_KEY = VIRTUAL_KEY(102u16);
pub const VK_NUMPAD7: VIRTUAL_KEY = VIRTUAL_KEY(103u16);
pub const VK_NUMPAD8: VIRTUAL_KEY = VIRTUAL_KEY(104u16);
pub const VK_NUMPAD9: VIRTUAL_KEY = VIRTUAL_KEY(105u16);
pub const VK_O: VIRTUAL_KEY = VIRTUAL_KEY(79u16);
pub const VK_OEM_1: VIRTUAL_KEY = VIRTUAL_KEY(186u16);
pub const VK_OEM_102: VIRTUAL_KEY = VIRTUAL_KEY(226u16);
pub const VK_OEM_2: VIRTUAL_KEY = VIRTUAL_KEY(191u16);
pub const VK_OEM_3: VIRTUAL_KEY = VIRTUAL_KEY(192u16);
pub const VK_OEM_4: VIRTUAL_KEY = VIRTUAL_KEY(219u16);
pub const VK_OEM_5: VIRTUAL_KEY = VIRTUAL_KEY(220u16);
pub const VK_OEM_6: VIRTUAL_KEY = VIRTUAL_KEY(221u16);
pub const VK_OEM_7: VIRTUAL_KEY = VIRTUAL_KEY(222u16);
pub const VK_OEM_8: VIRTUAL_KEY = VIRTUAL_KEY(223u16);
pub const VK_OEM_ATTN: VIRTUAL_KEY = VIRTUAL_KEY(240u16);
pub const VK_OEM_AUTO: VIRTUAL_KEY = VIRTUAL_KEY(243u16);
pub const VK_OEM_AX: VIRTUAL_KEY = VIRTUAL_KEY(225u16);
pub const VK_OEM_BACKTAB: VIRTUAL_KEY = VIRTUAL_KEY(245u16);
pub const VK_OEM_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(254u16);
pub const VK_OEM_COMMA: VIRTUAL_KEY = VIRTUAL_KEY(188u16);
pub const VK_OEM_COPY: VIRTUAL_KEY = VIRTUAL_KEY(242u16);
pub const VK_OEM_CUSEL: VIRTUAL_KEY = VIRTUAL_KEY(239u16);
pub const VK_OEM_ENLW: VIRTUAL_KEY = VIRTUAL_KEY(244u16);
pub const VK_OEM_FINISH: VIRTUAL_KEY = VIRTUAL_KEY(241u16);
pub const VK_OEM_FJ_JISHO: VIRTUAL_KEY = VIRTUAL_KEY(146u16);
pub const VK_OEM_FJ_LOYA: VIRTUAL_KEY = VIRTUAL_KEY(149u16);
pub const VK_OEM_FJ_MASSHOU: VIRTUAL_KEY = VIRTUAL_KEY(147u16);
pub const VK_OEM_FJ_ROYA: VIRTUAL_KEY = VIRTUAL_KEY(150u16);
pub const VK_OEM_FJ_TOUROKU: VIRTUAL_KEY = VIRTUAL_KEY(148u16);
pub const VK_OEM_JUMP: VIRTUAL_KEY = VIRTUAL_KEY(234u16);
pub const VK_OEM_MINUS: VIRTUAL_KEY = VIRTUAL_KEY(189u16);
pub const VK_OEM_NEC_EQUAL: VIRTUAL_KEY = VIRTUAL_KEY(146u16);
pub const VK_OEM_PA1: VIRTUAL_KEY = VIRTUAL_KEY(235u16);
pub const VK_OEM_PA2: VIRTUAL_KEY = VIRTUAL_KEY(236u16);
pub const VK_OEM_PA3: VIRTUAL_KEY = VIRTUAL_KEY(237u16);
pub const VK_OEM_PERIOD: VIRTUAL_KEY = VIRTUAL_KEY(190u16);
pub const VK_OEM_PLUS: VIRTUAL_KEY = VIRTUAL_KEY(187u16);
pub const VK_OEM_RESET: VIRTUAL_KEY = VIRTUAL_KEY(233u16);
pub const VK_OEM_WSCTRL: VIRTUAL_KEY = VIRTUAL_KEY(238u16);
pub const VK_P: VIRTUAL_KEY = VIRTUAL_KEY(80u16);
pub const VK_PA1: VIRTUAL_KEY = VIRTUAL_KEY(253u16);
pub const VK_PACKET: VIRTUAL_KEY = VIRTUAL_KEY(231u16);
pub const VK_PAUSE: VIRTUAL_KEY = VIRTUAL_KEY(19u16);
pub const VK_PLAY: VIRTUAL_KEY = VIRTUAL_KEY(250u16);
pub const VK_PRINT: VIRTUAL_KEY = VIRTUAL_KEY(42u16);
pub const VK_PRIOR: VIRTUAL_KEY = VIRTUAL_KEY(33u16);
pub const VK_PROCESSKEY: VIRTUAL_KEY = VIRTUAL_KEY(229u16);
pub const VK_Q: VIRTUAL_KEY = VIRTUAL_KEY(81u16);
pub const VK_R: VIRTUAL_KEY = VIRTUAL_KEY(82u16);
pub const VK_RBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(2u16);
pub const VK_RCONTROL: VIRTUAL_KEY = VIRTUAL_KEY(163u16);
pub const VK_RETURN: VIRTUAL_KEY = VIRTUAL_KEY(13u16);
pub const VK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(39u16);
pub const VK_RMENU: VIRTUAL_KEY = VIRTUAL_KEY(165u16);
pub const VK_RSHIFT: VIRTUAL_KEY = VIRTUAL_KEY(161u16);
pub const VK_RWIN: VIRTUAL_KEY = VIRTUAL_KEY(92u16);
pub const VK_S: VIRTUAL_KEY = VIRTUAL_KEY(83u16);
pub const VK_SCROLL: VIRTUAL_KEY = VIRTUAL_KEY(145u16);
pub const VK_SELECT: VIRTUAL_KEY = VIRTUAL_KEY(41u16);
pub const VK_SEPARATOR: VIRTUAL_KEY = VIRTUAL_KEY(108u16);
pub const VK_SHIFT: VIRTUAL_KEY = VIRTUAL_KEY(16u16);
pub const VK_SLEEP: VIRTUAL_KEY = VIRTUAL_KEY(95u16);
pub const VK_SNAPSHOT: VIRTUAL_KEY = VIRTUAL_KEY(44u16);
pub const VK_SPACE: VIRTUAL_KEY = VIRTUAL_KEY(32u16);
pub const VK_SUBTRACT: VIRTUAL_KEY = VIRTUAL_KEY(109u16);
pub const VK_T: VIRTUAL_KEY = VIRTUAL_KEY(84u16);
pub const VK_TAB: VIRTUAL_KEY = VIRTUAL_KEY(9u16);
pub const VK_U: VIRTUAL_KEY = VIRTUAL_KEY(85u16);
pub const VK_UP: VIRTUAL_KEY = VIRTUAL_KEY(38u16);
pub const VK_V: VIRTUAL_KEY = VIRTUAL_KEY(86u16);
pub const VK_VOLUME_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(174u16);
pub const VK_VOLUME_MUTE: VIRTUAL_KEY = VIRTUAL_KEY(173u16);
pub const VK_VOLUME_UP: VIRTUAL_KEY = VIRTUAL_KEY(175u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VK_VSC {
    pub Vk: u8,
    pub Vsc: u8,
}
pub const VK_W: VIRTUAL_KEY = VIRTUAL_KEY(87u16);
pub const VK_X: VIRTUAL_KEY = VIRTUAL_KEY(88u16);
pub const VK_XBUTTON1: VIRTUAL_KEY = VIRTUAL_KEY(5u16);
pub const VK_XBUTTON2: VIRTUAL_KEY = VIRTUAL_KEY(6u16);
pub const VK_Y: VIRTUAL_KEY = VIRTUAL_KEY(89u16);
pub const VK_Z: VIRTUAL_KEY = VIRTUAL_KEY(90u16);
pub const VK_ZOOM: VIRTUAL_KEY = VIRTUAL_KEY(251u16);
pub const VK__none_: VIRTUAL_KEY = VIRTUAL_KEY(255u16);
