// This file is auto-generated. Do not edit manually.


pub fn cgkeycode_to_u16(value: &CGKeyCode) -> u16 {
  #[allow(unused_parens)]
  const {
    assert!((&{CGKeyCode( KeyCode::MUTE                       )}).0 == 0x4A);
    assert!((&{CGKeyCode( KeyCode::VOLUME_DOWN                )}).0 == 0x49);
    assert!((&{CGKeyCode( KeyCode::VOLUME_UP                  )}).0 == 0x48);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_0              )}).0 == 0x1D);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_1              )}).0 == 0x12);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_2              )}).0 == 0x13);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_3              )}).0 == 0x14);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_4              )}).0 == 0x15);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_5              )}).0 == 0x17);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_6              )}).0 == 0x16);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_7              )}).0 == 0x1A);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_8              )}).0 == 0x1C);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_9              )}).0 == 0x19);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_A              )}).0 == 0x00);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_B              )}).0 == 0x0B);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_C              )}).0 == 0x08);
    assert!((&{CGKeyCode( KeyCode::CAPS_LOCK                  )}).0 == 0x39);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_D              )}).0 == 0x02);
    assert!((&{CGKeyCode( KeyCode::DELETE                     )}).0 == 0x33);
    assert!((&{CGKeyCode( KeyCode::FORWARD_DELETE             )}).0 == 0x75);
    assert!((&{CGKeyCode( KeyCode::DOWN_ARROW                 )}).0 == 0x7D);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_E              )}).0 == 0x0E);
    assert!((&{CGKeyCode( KeyCode::END                        )}).0 == 0x77);
    assert!((&{CGKeyCode( KeyCode::ESCAPE                     )}).0 == 0x35);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_F              )}).0 == 0x03);
    assert!((&{CGKeyCode( KeyCode::F1                         )}).0 == 0x7A);
    assert!((&{CGKeyCode( KeyCode::F10                        )}).0 == 0x6D);
    assert!((&{CGKeyCode( KeyCode::F11                        )}).0 == 0x67);
    assert!((&{CGKeyCode( KeyCode::F12                        )}).0 == 0x6F);
    assert!((&{CGKeyCode( KeyCode::F13                        )}).0 == 0x69);
    assert!((&{CGKeyCode( KeyCode::F14                        )}).0 == 0x6B);
    assert!((&{CGKeyCode( KeyCode::F15                        )}).0 == 0x71);
    assert!((&{CGKeyCode( KeyCode::F16                        )}).0 == 0x6A);
    assert!((&{CGKeyCode( KeyCode::F17                        )}).0 == 0x40);
    assert!((&{CGKeyCode( KeyCode::F18                        )}).0 == 0x4F);
    assert!((&{CGKeyCode( KeyCode::F19                        )}).0 == 0x50);
    assert!((&{CGKeyCode( KeyCode::F2                         )}).0 == 0x78);
    assert!((&{CGKeyCode( KeyCode::F20                        )}).0 == 0x5A);
    assert!((&{CGKeyCode( KeyCode::F3                         )}).0 == 0x63);
    assert!((&{CGKeyCode( KeyCode::F4                         )}).0 == 0x76);
    assert!((&{CGKeyCode( KeyCode::F5                         )}).0 == 0x60);
    assert!((&{CGKeyCode( KeyCode::F6                         )}).0 == 0x61);
    assert!((&{CGKeyCode( KeyCode::F7                         )}).0 == 0x62);
    assert!((&{CGKeyCode( KeyCode::F8                         )}).0 == 0x64);
    assert!((&{CGKeyCode( KeyCode::F9                         )}).0 == 0x65);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_G              )}).0 == 0x05);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_H              )}).0 == 0x04);
    assert!((&{CGKeyCode( KeyCode::HELP                       )}).0 == 0x72);
    assert!((&{CGKeyCode( KeyCode::HOME                       )}).0 == 0x73);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_I              )}).0 == 0x22);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_J              )}).0 == 0x26);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_K              )}).0 == 0x28);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_L              )}).0 == 0x25);
    assert!((&{CGKeyCode( KeyCode::OPTION                     )}).0 == 0x3A);
    assert!((&{CGKeyCode( KeyCode::LEFT_ARROW                 )}).0 == 0x7B);
    assert!((&{CGKeyCode( KeyCode::CONTROL                    )}).0 == 0x3B);
    assert!((&{CGKeyCode( KeyCode::CONTROL                    )}).0 == 0x3B);
    assert!((&{CGKeyCode( KeyCode::COMMAND                    )}).0 == 0x37);
    assert!((&{CGKeyCode( KeyCode::SHIFT                      )}).0 == 0x38);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_M              )}).0 == 0x2E);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_N              )}).0 == 0x2D);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_O              )}).0 == 0x1F);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_P              )}).0 == 0x23);
    assert!((&{CGKeyCode( KeyCode::PAGE_DOWN                  )}).0 == 0x79);
    assert!((&{CGKeyCode( KeyCode::PAGE_UP                    )}).0 == 0x74);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Q              )}).0 == 0x0C);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_R              )}).0 == 0x0F);
    assert!((&{CGKeyCode( KeyCode::RETURN                     )}).0 == 0x24);
    assert!((&{CGKeyCode( KeyCode::RIGHT_OPTION               )}).0 == 0x3D);
    assert!((&{CGKeyCode( KeyCode::RIGHT_ARROW                )}).0 == 0x7C);
    assert!((&{CGKeyCode( KeyCode::RIGHT_CONTROL              )}).0 == 0x3E);
    assert!((&{CGKeyCode( KeyCode::RIGHT_COMMAND              )}).0 == 0x36);
    assert!((&{CGKeyCode( KeyCode::RIGHT_SHIFT                )}).0 == 0x3C);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_S              )}).0 == 0x01);
    assert!((&{CGKeyCode( KeyCode::SPACE                      )}).0 == 0x31);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_T              )}).0 == 0x11);
    assert!((&{CGKeyCode( KeyCode::TAB                        )}).0 == 0x30);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_U              )}).0 == 0x20);
    assert!((&{CGKeyCode( KeyCode::UP_ARROW                   )}).0 == 0x7E);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_V              )}).0 == 0x09);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_W              )}).0 == 0x0D);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_X              )}).0 == 0x07);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Y              )}).0 == 0x10);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Z              )}).0 == 0x06);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad0        )}).0 == 0x52);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad1        )}).0 == 0x53);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad2        )}).0 == 0x54);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad3        )}).0 == 0x55);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad4        )}).0 == 0x56);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad5        )}).0 == 0x57);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad6        )}).0 == 0x58);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad7        )}).0 == 0x59);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad8        )}).0 == 0x5B);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad9        )}).0 == 0x5C);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadMinus    )}).0 == 0x4E);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadDivide   )}).0 == 0x4B);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadDecimal  )}).0 == 0x41);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadPlus     )}).0 == 0x45);
    assert!((&{CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadMultiply )}).0 == 0x43);
    assert!((&{CGKeyCode( KeyCode::FUNCTION                   )}).0 == 0x3F);
    assert!((&{CGKeyCode( CGKeyCode(131).0                    )}).0 == 0x83);
    assert!((&{CGKeyCode( CGKeyCode(160).0                    )}).0 == 0xA0);
  }
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
      0x4A => Some(CGKeyCode( KeyCode::MUTE                       )),
      0x49 => Some(CGKeyCode( KeyCode::VOLUME_DOWN                )),
      0x48 => Some(CGKeyCode( KeyCode::VOLUME_UP                  )),
      0x1D => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_0              )),
      0x12 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_1              )),
      0x13 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_2              )),
      0x14 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_3              )),
      0x15 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_4              )),
      0x17 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_5              )),
      0x16 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_6              )),
      0x1A => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_7              )),
      0x1C => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_8              )),
      0x19 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_9              )),
      0x00 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_A              )),
      0x0B => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_B              )),
      0x08 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_C              )),
      0x39 => Some(CGKeyCode( KeyCode::CAPS_LOCK                  )),
      0x02 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_D              )),
      0x33 => Some(CGKeyCode( KeyCode::DELETE                     )),
      0x75 => Some(CGKeyCode( KeyCode::FORWARD_DELETE             )),
      0x7D => Some(CGKeyCode( KeyCode::DOWN_ARROW                 )),
      0x0E => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_E              )),
      0x77 => Some(CGKeyCode( KeyCode::END                        )),
      0x35 => Some(CGKeyCode( KeyCode::ESCAPE                     )),
      0x03 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_F              )),
      0x7A => Some(CGKeyCode( KeyCode::F1                         )),
      0x6D => Some(CGKeyCode( KeyCode::F10                        )),
      0x67 => Some(CGKeyCode( KeyCode::F11                        )),
      0x6F => Some(CGKeyCode( KeyCode::F12                        )),
      0x69 => Some(CGKeyCode( KeyCode::F13                        )),
      0x6B => Some(CGKeyCode( KeyCode::F14                        )),
      0x71 => Some(CGKeyCode( KeyCode::F15                        )),
      0x6A => Some(CGKeyCode( KeyCode::F16                        )),
      0x40 => Some(CGKeyCode( KeyCode::F17                        )),
      0x4F => Some(CGKeyCode( KeyCode::F18                        )),
      0x50 => Some(CGKeyCode( KeyCode::F19                        )),
      0x78 => Some(CGKeyCode( KeyCode::F2                         )),
      0x5A => Some(CGKeyCode( KeyCode::F20                        )),
      0x63 => Some(CGKeyCode( KeyCode::F3                         )),
      0x76 => Some(CGKeyCode( KeyCode::F4                         )),
      0x60 => Some(CGKeyCode( KeyCode::F5                         )),
      0x61 => Some(CGKeyCode( KeyCode::F6                         )),
      0x62 => Some(CGKeyCode( KeyCode::F7                         )),
      0x64 => Some(CGKeyCode( KeyCode::F8                         )),
      0x65 => Some(CGKeyCode( KeyCode::F9                         )),
      0x05 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_G              )),
      0x04 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_H              )),
      0x72 => Some(CGKeyCode( KeyCode::HELP                       )),
      0x73 => Some(CGKeyCode( KeyCode::HOME                       )),
      0x22 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_I              )),
      0x26 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_J              )),
      0x28 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_K              )),
      0x25 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_L              )),
      0x3A => Some(CGKeyCode( KeyCode::OPTION                     )),
      0x7B => Some(CGKeyCode( KeyCode::LEFT_ARROW                 )),
      0x3B => Some(CGKeyCode( KeyCode::CONTROL                    )),
      0x37 => Some(CGKeyCode( KeyCode::COMMAND                    )),
      0x38 => Some(CGKeyCode( KeyCode::SHIFT                      )),
      0x2E => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_M              )),
      0x2D => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_N              )),
      0x1F => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_O              )),
      0x23 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_P              )),
      0x79 => Some(CGKeyCode( KeyCode::PAGE_DOWN                  )),
      0x74 => Some(CGKeyCode( KeyCode::PAGE_UP                    )),
      0x0C => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Q              )),
      0x0F => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_R              )),
      0x24 => Some(CGKeyCode( KeyCode::RETURN                     )),
      0x3D => Some(CGKeyCode( KeyCode::RIGHT_OPTION               )),
      0x7C => Some(CGKeyCode( KeyCode::RIGHT_ARROW                )),
      0x3E => Some(CGKeyCode( KeyCode::RIGHT_CONTROL              )),
      0x36 => Some(CGKeyCode( KeyCode::RIGHT_COMMAND              )),
      0x3C => Some(CGKeyCode( KeyCode::RIGHT_SHIFT                )),
      0x01 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_S              )),
      0x31 => Some(CGKeyCode( KeyCode::SPACE                      )),
      0x11 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_T              )),
      0x30 => Some(CGKeyCode( KeyCode::TAB                        )),
      0x20 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_U              )),
      0x7E => Some(CGKeyCode( KeyCode::UP_ARROW                   )),
      0x09 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_V              )),
      0x0D => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_W              )),
      0x07 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_X              )),
      0x10 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Y              )),
      0x06 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Z              )),
      0x52 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad0        )),
      0x53 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad1        )),
      0x54 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad2        )),
      0x55 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad3        )),
      0x56 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad4        )),
      0x57 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad5        )),
      0x58 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad6        )),
      0x59 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad7        )),
      0x5B => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad8        )),
      0x5C => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad9        )),
      0x4E => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadMinus    )),
      0x4B => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadDivide   )),
      0x41 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadDecimal  )),
      0x45 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadPlus     )),
      0x43 => Some(CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadMultiply )),
      0x3F => Some(CGKeyCode( KeyCode::FUNCTION                   )),
      0x83 => Some(CGKeyCode( CGKeyCode(131).0                    )),
      0xA0 => Some(CGKeyCode( CGKeyCode(160).0                    )),
      _ => None,
    }
  }
  unsafe fn from_code_unchecked(code: Self::Code) -> CGKeyCode {
    CGKeyCode(code)
  }
}

