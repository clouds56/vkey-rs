// This file is auto-generated. Do not edit manually.


pub fn cgkeycode_to_u16(value: &CGKeyCode) -> u16 {
  value.0
}

impl crate::numeric::AsCode<CGKeyCode> for crate::numeric::Coder {
  type Code = u16;
  fn as_code(value: &CGKeyCode) -> Self::Code {
    cgkeycode_to_u16(value)
  }
  #[allow(unreachable_patterns)]
  fn from_code(code: Self::Code) -> Option<CGKeyCode> {
    match code {
      0x4A => Some(KeyCode::MUTE                      .into_::<CGKeyCode>()),
      0x49 => Some(KeyCode::VOLUME_DOWN               .into_::<CGKeyCode>()),
      0x48 => Some(KeyCode::VOLUME_UP                 .into_::<CGKeyCode>()),
      0x1D => Some(KeyCodeExt::kVK_ANSI_0             .into_::<CGKeyCode>()),
      0x12 => Some(KeyCodeExt::kVK_ANSI_1             .into_::<CGKeyCode>()),
      0x13 => Some(KeyCodeExt::kVK_ANSI_2             .into_::<CGKeyCode>()),
      0x14 => Some(KeyCodeExt::kVK_ANSI_3             .into_::<CGKeyCode>()),
      0x15 => Some(KeyCodeExt::kVK_ANSI_4             .into_::<CGKeyCode>()),
      0x17 => Some(KeyCodeExt::kVK_ANSI_5             .into_::<CGKeyCode>()),
      0x16 => Some(KeyCodeExt::kVK_ANSI_6             .into_::<CGKeyCode>()),
      0x1A => Some(KeyCodeExt::kVK_ANSI_7             .into_::<CGKeyCode>()),
      0x1C => Some(KeyCodeExt::kVK_ANSI_8             .into_::<CGKeyCode>()),
      0x19 => Some(KeyCodeExt::kVK_ANSI_9             .into_::<CGKeyCode>()),
      0x00 => Some(KeyCodeExt::kVK_ANSI_A             .into_::<CGKeyCode>()),
      0x0B => Some(KeyCodeExt::kVK_ANSI_B             .into_::<CGKeyCode>()),
      0x08 => Some(KeyCodeExt::kVK_ANSI_C             .into_::<CGKeyCode>()),
      0x39 => Some(KeyCode::CAPS_LOCK                 .into_::<CGKeyCode>()),
      0x02 => Some(KeyCodeExt::kVK_ANSI_D             .into_::<CGKeyCode>()),
      0x33 => Some(KeyCode::DELETE                    .into_::<CGKeyCode>()),
      0x75 => Some(KeyCode::FORWARD_DELETE            .into_::<CGKeyCode>()),
      0x7D => Some(KeyCode::DOWN_ARROW                .into_::<CGKeyCode>()),
      0x0E => Some(KeyCodeExt::kVK_ANSI_E             .into_::<CGKeyCode>()),
      0x77 => Some(KeyCode::END                       .into_::<CGKeyCode>()),
      0x35 => Some(KeyCode::ESCAPE                    .into_::<CGKeyCode>()),
      0x03 => Some(KeyCodeExt::kVK_ANSI_F             .into_::<CGKeyCode>()),
      0x7A => Some(KeyCode::F1                        .into_::<CGKeyCode>()),
      0x6D => Some(KeyCode::F10                       .into_::<CGKeyCode>()),
      0x67 => Some(KeyCode::F11                       .into_::<CGKeyCode>()),
      0x6F => Some(KeyCode::F12                       .into_::<CGKeyCode>()),
      0x69 => Some(KeyCode::F13                       .into_::<CGKeyCode>()),
      0x6B => Some(KeyCode::F14                       .into_::<CGKeyCode>()),
      0x71 => Some(KeyCode::F15                       .into_::<CGKeyCode>()),
      0x6A => Some(KeyCode::F16                       .into_::<CGKeyCode>()),
      0x40 => Some(KeyCode::F17                       .into_::<CGKeyCode>()),
      0x4F => Some(KeyCode::F18                       .into_::<CGKeyCode>()),
      0x50 => Some(KeyCode::F19                       .into_::<CGKeyCode>()),
      0x78 => Some(KeyCode::F2                        .into_::<CGKeyCode>()),
      0x5A => Some(KeyCode::F20                       .into_::<CGKeyCode>()),
      0x63 => Some(KeyCode::F3                        .into_::<CGKeyCode>()),
      0x76 => Some(KeyCode::F4                        .into_::<CGKeyCode>()),
      0x60 => Some(KeyCode::F5                        .into_::<CGKeyCode>()),
      0x61 => Some(KeyCode::F6                        .into_::<CGKeyCode>()),
      0x62 => Some(KeyCode::F7                        .into_::<CGKeyCode>()),
      0x64 => Some(KeyCode::F8                        .into_::<CGKeyCode>()),
      0x65 => Some(KeyCode::F9                        .into_::<CGKeyCode>()),
      0x05 => Some(KeyCodeExt::kVK_ANSI_G             .into_::<CGKeyCode>()),
      0x04 => Some(KeyCodeExt::kVK_ANSI_H             .into_::<CGKeyCode>()),
      0x72 => Some(KeyCode::HELP                      .into_::<CGKeyCode>()),
      0x73 => Some(KeyCode::HOME                      .into_::<CGKeyCode>()),
      0x22 => Some(KeyCodeExt::kVK_ANSI_I             .into_::<CGKeyCode>()),
      0x26 => Some(KeyCodeExt::kVK_ANSI_J             .into_::<CGKeyCode>()),
      0x28 => Some(KeyCodeExt::kVK_ANSI_K             .into_::<CGKeyCode>()),
      0x25 => Some(KeyCodeExt::kVK_ANSI_L             .into_::<CGKeyCode>()),
      0x3A => Some(KeyCode::OPTION                    .into_::<CGKeyCode>()),
      0x7B => Some(KeyCode::LEFT_ARROW                .into_::<CGKeyCode>()),
      0x3B => Some(KeyCode::CONTROL                   .into_::<CGKeyCode>()),
      0x37 => Some(KeyCode::COMMAND                   .into_::<CGKeyCode>()),
      0x38 => Some(KeyCode::SHIFT                     .into_::<CGKeyCode>()),
      0x2E => Some(KeyCodeExt::kVK_ANSI_M             .into_::<CGKeyCode>()),
      0x2D => Some(KeyCodeExt::kVK_ANSI_N             .into_::<CGKeyCode>()),
      0x1F => Some(KeyCodeExt::kVK_ANSI_O             .into_::<CGKeyCode>()),
      0x23 => Some(KeyCodeExt::kVK_ANSI_P             .into_::<CGKeyCode>()),
      0x79 => Some(KeyCode::PAGE_DOWN                 .into_::<CGKeyCode>()),
      0x74 => Some(KeyCode::PAGE_UP                   .into_::<CGKeyCode>()),
      0x0C => Some(KeyCodeExt::kVK_ANSI_Q             .into_::<CGKeyCode>()),
      0x0F => Some(KeyCodeExt::kVK_ANSI_R             .into_::<CGKeyCode>()),
      0x24 => Some(KeyCode::RETURN                    .into_::<CGKeyCode>()),
      0x7C => Some(KeyCode::RIGHT_ARROW               .into_::<CGKeyCode>()),
      0x3E => Some(KeyCode::RIGHT_CONTROL             .into_::<CGKeyCode>()),
      0x3C => Some(KeyCode::RIGHT_SHIFT               .into_::<CGKeyCode>()),
      0x01 => Some(KeyCodeExt::kVK_ANSI_S             .into_::<CGKeyCode>()),
      0x31 => Some(KeyCode::SPACE                     .into_::<CGKeyCode>()),
      0x11 => Some(KeyCodeExt::kVK_ANSI_T             .into_::<CGKeyCode>()),
      0x30 => Some(KeyCode::TAB                       .into_::<CGKeyCode>()),
      0x20 => Some(KeyCodeExt::kVK_ANSI_U             .into_::<CGKeyCode>()),
      0x7E => Some(KeyCode::UP_ARROW                  .into_::<CGKeyCode>()),
      0x09 => Some(KeyCodeExt::kVK_ANSI_V             .into_::<CGKeyCode>()),
      0x0D => Some(KeyCodeExt::kVK_ANSI_W             .into_::<CGKeyCode>()),
      0x07 => Some(KeyCodeExt::kVK_ANSI_X             .into_::<CGKeyCode>()),
      0x10 => Some(KeyCodeExt::kVK_ANSI_Y             .into_::<CGKeyCode>()),
      0x06 => Some(KeyCodeExt::kVK_ANSI_Z             .into_::<CGKeyCode>()),
      0x52 => Some(KeyCodeExt::kVK_ANSI_Keypad0       .into_::<CGKeyCode>()),
      0x53 => Some(KeyCodeExt::kVK_ANSI_Keypad1       .into_::<CGKeyCode>()),
      0x54 => Some(KeyCodeExt::kVK_ANSI_Keypad2       .into_::<CGKeyCode>()),
      0x55 => Some(KeyCodeExt::kVK_ANSI_Keypad3       .into_::<CGKeyCode>()),
      0x56 => Some(KeyCodeExt::kVK_ANSI_Keypad4       .into_::<CGKeyCode>()),
      0x57 => Some(KeyCodeExt::kVK_ANSI_Keypad5       .into_::<CGKeyCode>()),
      0x58 => Some(KeyCodeExt::kVK_ANSI_Keypad6       .into_::<CGKeyCode>()),
      0x59 => Some(KeyCodeExt::kVK_ANSI_Keypad7       .into_::<CGKeyCode>()),
      0x5B => Some(KeyCodeExt::kVK_ANSI_Keypad8       .into_::<CGKeyCode>()),
      0x5C => Some(KeyCodeExt::kVK_ANSI_Keypad9       .into_::<CGKeyCode>()),
      0x4E => Some(KeyCodeExt::kVK_ANSI_KeypadMinus   .into_::<CGKeyCode>()),
      0x4B => Some(KeyCodeExt::kVK_ANSI_KeypadDivide  .into_::<CGKeyCode>()),
      0x41 => Some(KeyCodeExt::kVK_ANSI_KeypadDecimal .into_::<CGKeyCode>()),
      0x45 => Some(KeyCodeExt::kVK_ANSI_KeypadPlus    .into_::<CGKeyCode>()),
      0x43 => Some(KeyCodeExt::kVK_ANSI_KeypadMultiply.into_::<CGKeyCode>()),
      0x3F => Some(KeyCode::FUNCTION                  .into_::<CGKeyCode>()),
      0x83 => Some(CGKeyCode::from(131)               .into_::<CGKeyCode>()),
      0xA0 => Some(CGKeyCode::from(160)               .into_::<CGKeyCode>()),
      0x36 => Some(KeyCode::RIGHT_COMMAND             .into_::<CGKeyCode>()),
      0x3D => Some(KeyCode::RIGHT_OPTION              .into_::<CGKeyCode>()),
      _ => None,
    }
  }
  unsafe fn from_code_unchecked(code: Self::Code) -> CGKeyCode {
    CGKeyCode(code)
  }
}

#[test]
#[allow(unused_parens)]
fn test_code() {
  assert!((&{KeyCode::MUTE                      .into_::<CGKeyCode>()}).0 == 0x4A);
  assert!((&{KeyCode::VOLUME_DOWN               .into_::<CGKeyCode>()}).0 == 0x49);
  assert!((&{KeyCode::VOLUME_UP                 .into_::<CGKeyCode>()}).0 == 0x48);
  assert!((&{KeyCodeExt::kVK_ANSI_0             .into_::<CGKeyCode>()}).0 == 0x1D);
  assert!((&{KeyCodeExt::kVK_ANSI_1             .into_::<CGKeyCode>()}).0 == 0x12);
  assert!((&{KeyCodeExt::kVK_ANSI_2             .into_::<CGKeyCode>()}).0 == 0x13);
  assert!((&{KeyCodeExt::kVK_ANSI_3             .into_::<CGKeyCode>()}).0 == 0x14);
  assert!((&{KeyCodeExt::kVK_ANSI_4             .into_::<CGKeyCode>()}).0 == 0x15);
  assert!((&{KeyCodeExt::kVK_ANSI_5             .into_::<CGKeyCode>()}).0 == 0x17);
  assert!((&{KeyCodeExt::kVK_ANSI_6             .into_::<CGKeyCode>()}).0 == 0x16);
  assert!((&{KeyCodeExt::kVK_ANSI_7             .into_::<CGKeyCode>()}).0 == 0x1A);
  assert!((&{KeyCodeExt::kVK_ANSI_8             .into_::<CGKeyCode>()}).0 == 0x1C);
  assert!((&{KeyCodeExt::kVK_ANSI_9             .into_::<CGKeyCode>()}).0 == 0x19);
  assert!((&{KeyCodeExt::kVK_ANSI_A             .into_::<CGKeyCode>()}).0 == 0x00);
  assert!((&{KeyCodeExt::kVK_ANSI_B             .into_::<CGKeyCode>()}).0 == 0x0B);
  assert!((&{KeyCodeExt::kVK_ANSI_C             .into_::<CGKeyCode>()}).0 == 0x08);
  assert!((&{KeyCode::CAPS_LOCK                 .into_::<CGKeyCode>()}).0 == 0x39);
  assert!((&{KeyCodeExt::kVK_ANSI_D             .into_::<CGKeyCode>()}).0 == 0x02);
  assert!((&{KeyCode::DELETE                    .into_::<CGKeyCode>()}).0 == 0x33);
  assert!((&{KeyCode::FORWARD_DELETE            .into_::<CGKeyCode>()}).0 == 0x75);
  assert!((&{KeyCode::DOWN_ARROW                .into_::<CGKeyCode>()}).0 == 0x7D);
  assert!((&{KeyCodeExt::kVK_ANSI_E             .into_::<CGKeyCode>()}).0 == 0x0E);
  assert!((&{KeyCode::END                       .into_::<CGKeyCode>()}).0 == 0x77);
  assert!((&{KeyCode::ESCAPE                    .into_::<CGKeyCode>()}).0 == 0x35);
  assert!((&{KeyCodeExt::kVK_ANSI_F             .into_::<CGKeyCode>()}).0 == 0x03);
  assert!((&{KeyCode::F1                        .into_::<CGKeyCode>()}).0 == 0x7A);
  assert!((&{KeyCode::F10                       .into_::<CGKeyCode>()}).0 == 0x6D);
  assert!((&{KeyCode::F11                       .into_::<CGKeyCode>()}).0 == 0x67);
  assert!((&{KeyCode::F12                       .into_::<CGKeyCode>()}).0 == 0x6F);
  assert!((&{KeyCode::F13                       .into_::<CGKeyCode>()}).0 == 0x69);
  assert!((&{KeyCode::F14                       .into_::<CGKeyCode>()}).0 == 0x6B);
  assert!((&{KeyCode::F15                       .into_::<CGKeyCode>()}).0 == 0x71);
  assert!((&{KeyCode::F16                       .into_::<CGKeyCode>()}).0 == 0x6A);
  assert!((&{KeyCode::F17                       .into_::<CGKeyCode>()}).0 == 0x40);
  assert!((&{KeyCode::F18                       .into_::<CGKeyCode>()}).0 == 0x4F);
  assert!((&{KeyCode::F19                       .into_::<CGKeyCode>()}).0 == 0x50);
  assert!((&{KeyCode::F2                        .into_::<CGKeyCode>()}).0 == 0x78);
  assert!((&{KeyCode::F20                       .into_::<CGKeyCode>()}).0 == 0x5A);
  assert!((&{KeyCode::F3                        .into_::<CGKeyCode>()}).0 == 0x63);
  assert!((&{KeyCode::F4                        .into_::<CGKeyCode>()}).0 == 0x76);
  assert!((&{KeyCode::F5                        .into_::<CGKeyCode>()}).0 == 0x60);
  assert!((&{KeyCode::F6                        .into_::<CGKeyCode>()}).0 == 0x61);
  assert!((&{KeyCode::F7                        .into_::<CGKeyCode>()}).0 == 0x62);
  assert!((&{KeyCode::F8                        .into_::<CGKeyCode>()}).0 == 0x64);
  assert!((&{KeyCode::F9                        .into_::<CGKeyCode>()}).0 == 0x65);
  assert!((&{KeyCodeExt::kVK_ANSI_G             .into_::<CGKeyCode>()}).0 == 0x05);
  assert!((&{KeyCodeExt::kVK_ANSI_H             .into_::<CGKeyCode>()}).0 == 0x04);
  assert!((&{KeyCode::HELP                      .into_::<CGKeyCode>()}).0 == 0x72);
  assert!((&{KeyCode::HOME                      .into_::<CGKeyCode>()}).0 == 0x73);
  assert!((&{KeyCodeExt::kVK_ANSI_I             .into_::<CGKeyCode>()}).0 == 0x22);
  assert!((&{KeyCodeExt::kVK_ANSI_J             .into_::<CGKeyCode>()}).0 == 0x26);
  assert!((&{KeyCodeExt::kVK_ANSI_K             .into_::<CGKeyCode>()}).0 == 0x28);
  assert!((&{KeyCodeExt::kVK_ANSI_L             .into_::<CGKeyCode>()}).0 == 0x25);
  assert!((&{KeyCode::OPTION                    .into_::<CGKeyCode>()}).0 == 0x3A);
  assert!((&{KeyCode::OPTION                    .into_::<CGKeyCode>()}).0 == 0x3A);
  assert!((&{KeyCode::LEFT_ARROW                .into_::<CGKeyCode>()}).0 == 0x7B);
  assert!((&{KeyCode::CONTROL                   .into_::<CGKeyCode>()}).0 == 0x3B);
  assert!((&{KeyCode::CONTROL                   .into_::<CGKeyCode>()}).0 == 0x3B);
  assert!((&{KeyCode::COMMAND                   .into_::<CGKeyCode>()}).0 == 0x37);
  assert!((&{KeyCode::COMMAND                   .into_::<CGKeyCode>()}).0 == 0x37);
  assert!((&{KeyCode::COMMAND                   .into_::<CGKeyCode>()}).0 == 0x37);
  assert!((&{KeyCode::COMMAND                   .into_::<CGKeyCode>()}).0 == 0x37);
  assert!((&{KeyCode::SHIFT                     .into_::<CGKeyCode>()}).0 == 0x38);
  assert!((&{KeyCode::SHIFT                     .into_::<CGKeyCode>()}).0 == 0x38);
  assert!((&{KeyCodeExt::kVK_ANSI_M             .into_::<CGKeyCode>()}).0 == 0x2E);
  assert!((&{KeyCodeExt::kVK_ANSI_N             .into_::<CGKeyCode>()}).0 == 0x2D);
  assert!((&{KeyCodeExt::kVK_ANSI_O             .into_::<CGKeyCode>()}).0 == 0x1F);
  assert!((&{KeyCodeExt::kVK_ANSI_P             .into_::<CGKeyCode>()}).0 == 0x23);
  assert!((&{KeyCode::PAGE_DOWN                 .into_::<CGKeyCode>()}).0 == 0x79);
  assert!((&{KeyCode::PAGE_UP                   .into_::<CGKeyCode>()}).0 == 0x74);
  assert!((&{KeyCodeExt::kVK_ANSI_Q             .into_::<CGKeyCode>()}).0 == 0x0C);
  assert!((&{KeyCodeExt::kVK_ANSI_R             .into_::<CGKeyCode>()}).0 == 0x0F);
  assert!((&{KeyCode::RETURN                    .into_::<CGKeyCode>()}).0 == 0x24);
  assert!((&{KeyCode::RIGHT_ARROW               .into_::<CGKeyCode>()}).0 == 0x7C);
  assert!((&{KeyCode::RIGHT_CONTROL             .into_::<CGKeyCode>()}).0 == 0x3E);
  assert!((&{KeyCode::RIGHT_SHIFT               .into_::<CGKeyCode>()}).0 == 0x3C);
  assert!((&{KeyCodeExt::kVK_ANSI_S             .into_::<CGKeyCode>()}).0 == 0x01);
  assert!((&{KeyCode::SPACE                     .into_::<CGKeyCode>()}).0 == 0x31);
  assert!((&{KeyCodeExt::kVK_ANSI_T             .into_::<CGKeyCode>()}).0 == 0x11);
  assert!((&{KeyCode::TAB                       .into_::<CGKeyCode>()}).0 == 0x30);
  assert!((&{KeyCodeExt::kVK_ANSI_U             .into_::<CGKeyCode>()}).0 == 0x20);
  assert!((&{KeyCode::UP_ARROW                  .into_::<CGKeyCode>()}).0 == 0x7E);
  assert!((&{KeyCodeExt::kVK_ANSI_V             .into_::<CGKeyCode>()}).0 == 0x09);
  assert!((&{KeyCodeExt::kVK_ANSI_W             .into_::<CGKeyCode>()}).0 == 0x0D);
  assert!((&{KeyCodeExt::kVK_ANSI_X             .into_::<CGKeyCode>()}).0 == 0x07);
  assert!((&{KeyCodeExt::kVK_ANSI_Y             .into_::<CGKeyCode>()}).0 == 0x10);
  assert!((&{KeyCodeExt::kVK_ANSI_Z             .into_::<CGKeyCode>()}).0 == 0x06);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad0       .into_::<CGKeyCode>()}).0 == 0x52);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad1       .into_::<CGKeyCode>()}).0 == 0x53);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad2       .into_::<CGKeyCode>()}).0 == 0x54);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad3       .into_::<CGKeyCode>()}).0 == 0x55);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad4       .into_::<CGKeyCode>()}).0 == 0x56);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad5       .into_::<CGKeyCode>()}).0 == 0x57);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad6       .into_::<CGKeyCode>()}).0 == 0x58);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad7       .into_::<CGKeyCode>()}).0 == 0x59);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad8       .into_::<CGKeyCode>()}).0 == 0x5B);
  assert!((&{KeyCodeExt::kVK_ANSI_Keypad9       .into_::<CGKeyCode>()}).0 == 0x5C);
  assert!((&{KeyCodeExt::kVK_ANSI_KeypadMinus   .into_::<CGKeyCode>()}).0 == 0x4E);
  assert!((&{KeyCodeExt::kVK_ANSI_KeypadDivide  .into_::<CGKeyCode>()}).0 == 0x4B);
  assert!((&{KeyCodeExt::kVK_ANSI_KeypadDecimal .into_::<CGKeyCode>()}).0 == 0x41);
  assert!((&{KeyCodeExt::kVK_ANSI_KeypadPlus    .into_::<CGKeyCode>()}).0 == 0x45);
  assert!((&{KeyCodeExt::kVK_ANSI_KeypadMultiply.into_::<CGKeyCode>()}).0 == 0x43);
  assert!((&{KeyCode::FUNCTION                  .into_::<CGKeyCode>()}).0 == 0x3F);
  assert!((&{CGKeyCode::from(131)               .into_::<CGKeyCode>()}).0 == 0x83);
  assert!((&{CGKeyCode::from(160)               .into_::<CGKeyCode>()}).0 == 0xA0);
  assert!((&{KeyCode::RIGHT_COMMAND             .into_::<CGKeyCode>()}).0 == 0x36);
  assert!((&{KeyCode::RIGHT_OPTION              .into_::<CGKeyCode>()}).0 == 0x3D);
}
