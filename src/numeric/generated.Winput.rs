// This file is auto-generated. Do not edit manually.


pub fn vk_to_u8(value: &Vk) -> u8 {
  #[allow(unused_parens)]
  const {
    assert!(*(&{Vk::MouseLeft       }) as u8 == 0x01);
    assert!(*(&{Vk::MouseRight      }) as u8 == 0x02);
    assert!(*(&{Vk::MouseMiddle     }) as u8 == 0x04);
    assert!(*(&{Vk::MouseX1         }) as u8 == 0x05);
    assert!(*(&{Vk::MouseX2         }) as u8 == 0x06);
    assert!(*(&{Vk::BrowserBack     }) as u8 == 0xA6);
    assert!(*(&{Vk::BrowserFavorites}) as u8 == 0xAB);
    assert!(*(&{Vk::BrowserForward  }) as u8 == 0xA7);
    assert!(*(&{Vk::BrowserHome     }) as u8 == 0xAC);
    assert!(*(&{Vk::BrowserRefresh  }) as u8 == 0xA8);
    assert!(*(&{Vk::BrowserSearch   }) as u8 == 0xAA);
    assert!(*(&{Vk::BrowserStop     }) as u8 == 0xA9);
    assert!(*(&{Vk::StartMail       }) as u8 == 0xB4);
    assert!(*(&{Vk::SelectMedia     }) as u8 == 0xB5);
    assert!(*(&{Vk::VolumeMute      }) as u8 == 0xAD);
    assert!(*(&{Vk::MediaPlayPause  }) as u8 == 0xB3);
    assert!(*(&{Vk::NextTrack       }) as u8 == 0xB0);
    assert!(*(&{Vk::PrevTrack       }) as u8 == 0xB1);
    assert!(*(&{Vk::MediaStop       }) as u8 == 0xB2);
    assert!(*(&{Vk::VolumeDown      }) as u8 == 0xAE);
    assert!(*(&{Vk::VolumeUp        }) as u8 == 0xAF);
    assert!(*(&{Vk::Sleep           }) as u8 == 0x5F);
    assert!(*(&{Vk::_0              }) as u8 == 0x30);
    assert!(*(&{Vk::_1              }) as u8 == 0x31);
    assert!(*(&{Vk::_2              }) as u8 == 0x32);
    assert!(*(&{Vk::_3              }) as u8 == 0x33);
    assert!(*(&{Vk::_4              }) as u8 == 0x34);
    assert!(*(&{Vk::_5              }) as u8 == 0x35);
    assert!(*(&{Vk::_6              }) as u8 == 0x36);
    assert!(*(&{Vk::_7              }) as u8 == 0x37);
    assert!(*(&{Vk::_8              }) as u8 == 0x38);
    assert!(*(&{Vk::_9              }) as u8 == 0x39);
    assert!(*(&{Vk::A               }) as u8 == 0x41);
    assert!(*(&{Vk::Apps            }) as u8 == 0x5D);
    assert!(*(&{Vk::B               }) as u8 == 0x42);
    assert!(*(&{Vk::Oem5            }) as u8 == 0xDC);
    assert!(*(&{Vk::C               }) as u8 == 0x43);
    assert!(*(&{Vk::Cancel          }) as u8 == 0x03);
    assert!(*(&{Vk::CapsLock        }) as u8 == 0x14);
    assert!(*(&{Vk::Clear           }) as u8 == 0x0C);
    assert!(*(&{Vk::Comma           }) as u8 == 0xBC);
    assert!(*(&{Vk::D               }) as u8 == 0x44);
    assert!(*(&{Vk::Minus           }) as u8 == 0xBD);
    assert!(*(&{Vk::Backspace       }) as u8 == 0x08);
    assert!(*(&{Vk::Delete          }) as u8 == 0x2E);
    assert!(*(&{Vk::DownArrow       }) as u8 == 0x28);
    assert!(*(&{Vk::E               }) as u8 == 0x45);
    assert!(*(&{Vk::End             }) as u8 == 0x23);
    assert!(*(&{Vk::Plus            }) as u8 == 0xBB);
    assert!(*(&{Vk::Escape          }) as u8 == 0x1B);
    assert!(*(&{Vk::Execute         }) as u8 == 0x2B);
    assert!(*(&{Vk::F               }) as u8 == 0x46);
    assert!(*(&{Vk::F1              }) as u8 == 0x70);
    assert!(*(&{Vk::F10             }) as u8 == 0x79);
    assert!(*(&{Vk::F11             }) as u8 == 0x7A);
    assert!(*(&{Vk::F12             }) as u8 == 0x7B);
    assert!(*(&{Vk::F13             }) as u8 == 0x7C);
    assert!(*(&{Vk::F14             }) as u8 == 0x7D);
    assert!(*(&{Vk::F15             }) as u8 == 0x7E);
    assert!(*(&{Vk::F16             }) as u8 == 0x7F);
    assert!(*(&{Vk::F17             }) as u8 == 0x80);
    assert!(*(&{Vk::F18             }) as u8 == 0x81);
    assert!(*(&{Vk::F19             }) as u8 == 0x82);
    assert!(*(&{Vk::F2              }) as u8 == 0x71);
    assert!(*(&{Vk::F20             }) as u8 == 0x83);
    assert!(*(&{Vk::F21             }) as u8 == 0x84);
    assert!(*(&{Vk::F22             }) as u8 == 0x85);
    assert!(*(&{Vk::F23             }) as u8 == 0x86);
    assert!(*(&{Vk::F24             }) as u8 == 0x87);
    assert!(*(&{Vk::F3              }) as u8 == 0x72);
    assert!(*(&{Vk::F4              }) as u8 == 0x73);
    assert!(*(&{Vk::F5              }) as u8 == 0x74);
    assert!(*(&{Vk::F6              }) as u8 == 0x75);
    assert!(*(&{Vk::F7              }) as u8 == 0x76);
    assert!(*(&{Vk::F8              }) as u8 == 0x77);
    assert!(*(&{Vk::F9              }) as u8 == 0x78);
    assert!(*(&{Vk::Oem2            }) as u8 == 0xBF);
    assert!(*(&{Vk::G               }) as u8 == 0x47);
    assert!(*(&{Vk::Oem3            }) as u8 == 0xC0);
    assert!(*(&{Vk::H               }) as u8 == 0x48);
    assert!(*(&{Vk::Help            }) as u8 == 0x2F);
    assert!(*(&{Vk::Home            }) as u8 == 0x24);
    assert!(*(&{Vk::I               }) as u8 == 0x49);
    assert!(*(&{Vk::Insert          }) as u8 == 0x2D);
    assert!(*(&{Vk::J               }) as u8 == 0x4A);
    assert!(*(&{Vk::K               }) as u8 == 0x4B);
    assert!(*(&{Vk::L               }) as u8 == 0x4C);
    assert!(*(&{Vk::LeftMenu        }) as u8 == 0xA4);
    assert!(*(&{Vk::Alt             }) as u8 == 0x12);
    assert!(*(&{Vk::Oem7            }) as u8 == 0xDE);
    assert!(*(&{Vk::LeftArrow       }) as u8 == 0x25);
    assert!(*(&{Vk::Oem4            }) as u8 == 0xDB);
    assert!(*(&{Vk::Control         }) as u8 == 0x11);
    assert!(*(&{Vk::LeftControl     }) as u8 == 0xA2);
    assert!(*(&{Vk::LeftWin         }) as u8 == 0x5B);
    assert!(*(&{Vk::LeftShift       }) as u8 == 0xA0);
    assert!(*(&{Vk::Shift           }) as u8 == 0x10);
    assert!(*(&{Vk::M               }) as u8 == 0x4D);
    assert!(*(&{Vk::N               }) as u8 == 0x4E);
    assert!(*(&{Vk::Oem102          }) as u8 == 0xE2);
    assert!(*(&{Vk::O               }) as u8 == 0x4F);
    assert!(*(&{Vk::P               }) as u8 == 0x50);
    assert!(*(&{Vk::PageDown        }) as u8 == 0x22);
    assert!(*(&{Vk::PageUp          }) as u8 == 0x21);
    assert!(*(&{Vk::Pause           }) as u8 == 0x13);
    assert!(*(&{Vk::Period          }) as u8 == 0xBE);
    assert!(*(&{Vk::PrintScreen     }) as u8 == 0x2C);
    assert!(*(&{Vk::Q               }) as u8 == 0x51);
    assert!(*(&{Vk::R               }) as u8 == 0x52);
    assert!(*(&{Vk::Enter           }) as u8 == 0x0D);
    assert!(*(&{Vk::RightMenu       }) as u8 == 0xA5);
    assert!(*(&{Vk::RightArrow      }) as u8 == 0x27);
    assert!(*(&{Vk::Oem6            }) as u8 == 0xDD);
    assert!(*(&{Vk::RightControl    }) as u8 == 0xA3);
    assert!(*(&{Vk::RightWin        }) as u8 == 0x5C);
    assert!(*(&{Vk::RightShift      }) as u8 == 0xA1);
    assert!(*(&{Vk::S               }) as u8 == 0x53);
    assert!(*(&{Vk::Scroll          }) as u8 == 0x91);
    assert!(*(&{Vk::Select          }) as u8 == 0x29);
    assert!(*(&{Vk::Oem1            }) as u8 == 0xBA);
    assert!(*(&{Vk::Space           }) as u8 == 0x20);
    assert!(*(&{Vk::T               }) as u8 == 0x54);
    assert!(*(&{Vk::Tab             }) as u8 == 0x09);
    assert!(*(&{Vk::U               }) as u8 == 0x55);
    assert!(*(&{Vk::UpArrow         }) as u8 == 0x26);
    assert!(*(&{Vk::V               }) as u8 == 0x56);
    assert!(*(&{Vk::W               }) as u8 == 0x57);
    assert!(*(&{Vk::X               }) as u8 == 0x58);
    assert!(*(&{Vk::Y               }) as u8 == 0x59);
    assert!(*(&{Vk::Z               }) as u8 == 0x5A);
    assert!(*(&{Vk::Numpad0         }) as u8 == 0x60);
    assert!(*(&{Vk::Numpad1         }) as u8 == 0x61);
    assert!(*(&{Vk::Numpad2         }) as u8 == 0x62);
    assert!(*(&{Vk::Numpad3         }) as u8 == 0x63);
    assert!(*(&{Vk::Numpad4         }) as u8 == 0x64);
    assert!(*(&{Vk::Numpad5         }) as u8 == 0x65);
    assert!(*(&{Vk::Numpad6         }) as u8 == 0x66);
    assert!(*(&{Vk::Numpad7         }) as u8 == 0x67);
    assert!(*(&{Vk::Numpad8         }) as u8 == 0x68);
    assert!(*(&{Vk::Numpad9         }) as u8 == 0x69);
    assert!(*(&{Vk::Subtract        }) as u8 == 0x6D);
    assert!(*(&{Vk::Divide          }) as u8 == 0x6F);
    assert!(*(&{Vk::Numlock         }) as u8 == 0x90);
    assert!(*(&{Vk::Decimal         }) as u8 == 0x6E);
    assert!(*(&{Vk::Add             }) as u8 == 0x6B);
    assert!(*(&{Vk::Multiply        }) as u8 == 0x6A);
    assert!(*(&{Vk::Accept          }) as u8 == 0x1E);
    assert!(*(&{Vk::Convert         }) as u8 == 0x1C);
    assert!(*(&{Vk::Final           }) as u8 == 0x18);
    assert!(*(&{Vk::ImeOff          }) as u8 == 0x1A);
    assert!(*(&{Vk::ImeOn           }) as u8 == 0x16);
    assert!(*(&{Vk::Junja           }) as u8 == 0x17);
    assert!(*(&{Vk::Kana            }) as u8 == 0x15);
    assert!(*(&{Vk::Kanji           }) as u8 == 0x19);
    assert!(*(&{Vk::ModeChange      }) as u8 == 0x1F);
    assert!(*(&{Vk::NonConvert      }) as u8 == 0x1D);
    assert!(*(&{Vk::Print           }) as u8 == 0x2A);
    assert!(*(&{Vk::Separator       }) as u8 == 0x6C);
    assert!(*(&{Vk::StartApp1       }) as u8 == 0xB6);
    assert!(*(&{Vk::StartApp2       }) as u8 == 0xB7);
    }
  *value as u8
}

impl crate::numeric::AsCode<Vk> for crate::numeric::Coder {
  type Code = u8;
  fn as_code(value: &Vk) -> Self::Code {
    vk_to_u8(value)
  }
  fn from_code(code: Self::Code) -> Option<Vk> {
    match code {
      0x01 => Some(Vk::MouseLeft       ),
      0x02 => Some(Vk::MouseRight      ),
      0x04 => Some(Vk::MouseMiddle     ),
      0x05 => Some(Vk::MouseX1         ),
      0x06 => Some(Vk::MouseX2         ),
      0xA6 => Some(Vk::BrowserBack     ),
      0xAB => Some(Vk::BrowserFavorites),
      0xA7 => Some(Vk::BrowserForward  ),
      0xAC => Some(Vk::BrowserHome     ),
      0xA8 => Some(Vk::BrowserRefresh  ),
      0xAA => Some(Vk::BrowserSearch   ),
      0xA9 => Some(Vk::BrowserStop     ),
      0xB4 => Some(Vk::StartMail       ),
      0xB5 => Some(Vk::SelectMedia     ),
      0xAD => Some(Vk::VolumeMute      ),
      0xB3 => Some(Vk::MediaPlayPause  ),
      0xB0 => Some(Vk::NextTrack       ),
      0xB1 => Some(Vk::PrevTrack       ),
      0xB2 => Some(Vk::MediaStop       ),
      0xAE => Some(Vk::VolumeDown      ),
      0xAF => Some(Vk::VolumeUp        ),
      0x5F => Some(Vk::Sleep           ),
      0x30 => Some(Vk::_0              ),
      0x31 => Some(Vk::_1              ),
      0x32 => Some(Vk::_2              ),
      0x33 => Some(Vk::_3              ),
      0x34 => Some(Vk::_4              ),
      0x35 => Some(Vk::_5              ),
      0x36 => Some(Vk::_6              ),
      0x37 => Some(Vk::_7              ),
      0x38 => Some(Vk::_8              ),
      0x39 => Some(Vk::_9              ),
      0x41 => Some(Vk::A               ),
      0x5D => Some(Vk::Apps            ),
      0x42 => Some(Vk::B               ),
      0xDC => Some(Vk::Oem5            ),
      0x43 => Some(Vk::C               ),
      0x03 => Some(Vk::Cancel          ),
      0x14 => Some(Vk::CapsLock        ),
      0x0C => Some(Vk::Clear           ),
      0xBC => Some(Vk::Comma           ),
      0x44 => Some(Vk::D               ),
      0xBD => Some(Vk::Minus           ),
      0x08 => Some(Vk::Backspace       ),
      0x2E => Some(Vk::Delete          ),
      0x28 => Some(Vk::DownArrow       ),
      0x45 => Some(Vk::E               ),
      0x23 => Some(Vk::End             ),
      0xBB => Some(Vk::Plus            ),
      0x1B => Some(Vk::Escape          ),
      0x2B => Some(Vk::Execute         ),
      0x46 => Some(Vk::F               ),
      0x70 => Some(Vk::F1              ),
      0x79 => Some(Vk::F10             ),
      0x7A => Some(Vk::F11             ),
      0x7B => Some(Vk::F12             ),
      0x7C => Some(Vk::F13             ),
      0x7D => Some(Vk::F14             ),
      0x7E => Some(Vk::F15             ),
      0x7F => Some(Vk::F16             ),
      0x80 => Some(Vk::F17             ),
      0x81 => Some(Vk::F18             ),
      0x82 => Some(Vk::F19             ),
      0x71 => Some(Vk::F2              ),
      0x83 => Some(Vk::F20             ),
      0x84 => Some(Vk::F21             ),
      0x85 => Some(Vk::F22             ),
      0x86 => Some(Vk::F23             ),
      0x87 => Some(Vk::F24             ),
      0x72 => Some(Vk::F3              ),
      0x73 => Some(Vk::F4              ),
      0x74 => Some(Vk::F5              ),
      0x75 => Some(Vk::F6              ),
      0x76 => Some(Vk::F7              ),
      0x77 => Some(Vk::F8              ),
      0x78 => Some(Vk::F9              ),
      0xBF => Some(Vk::Oem2            ),
      0x47 => Some(Vk::G               ),
      0xC0 => Some(Vk::Oem3            ),
      0x48 => Some(Vk::H               ),
      0x2F => Some(Vk::Help            ),
      0x24 => Some(Vk::Home            ),
      0x49 => Some(Vk::I               ),
      0x2D => Some(Vk::Insert          ),
      0x4A => Some(Vk::J               ),
      0x4B => Some(Vk::K               ),
      0x4C => Some(Vk::L               ),
      0xA4 => Some(Vk::LeftMenu        ),
      0x12 => Some(Vk::Alt             ),
      0xDE => Some(Vk::Oem7            ),
      0x25 => Some(Vk::LeftArrow       ),
      0xDB => Some(Vk::Oem4            ),
      0x11 => Some(Vk::Control         ),
      0xA2 => Some(Vk::LeftControl     ),
      0x5B => Some(Vk::LeftWin         ),
      0xA0 => Some(Vk::LeftShift       ),
      0x10 => Some(Vk::Shift           ),
      0x4D => Some(Vk::M               ),
      0x4E => Some(Vk::N               ),
      0xE2 => Some(Vk::Oem102          ),
      0x4F => Some(Vk::O               ),
      0x50 => Some(Vk::P               ),
      0x22 => Some(Vk::PageDown        ),
      0x21 => Some(Vk::PageUp          ),
      0x13 => Some(Vk::Pause           ),
      0xBE => Some(Vk::Period          ),
      0x2C => Some(Vk::PrintScreen     ),
      0x51 => Some(Vk::Q               ),
      0x52 => Some(Vk::R               ),
      0x0D => Some(Vk::Enter           ),
      0xA5 => Some(Vk::RightMenu       ),
      0x27 => Some(Vk::RightArrow      ),
      0xDD => Some(Vk::Oem6            ),
      0xA3 => Some(Vk::RightControl    ),
      0x5C => Some(Vk::RightWin        ),
      0xA1 => Some(Vk::RightShift      ),
      0x53 => Some(Vk::S               ),
      0x91 => Some(Vk::Scroll          ),
      0x29 => Some(Vk::Select          ),
      0xBA => Some(Vk::Oem1            ),
      0x20 => Some(Vk::Space           ),
      0x54 => Some(Vk::T               ),
      0x09 => Some(Vk::Tab             ),
      0x55 => Some(Vk::U               ),
      0x26 => Some(Vk::UpArrow         ),
      0x56 => Some(Vk::V               ),
      0x57 => Some(Vk::W               ),
      0x58 => Some(Vk::X               ),
      0x59 => Some(Vk::Y               ),
      0x5A => Some(Vk::Z               ),
      0x60 => Some(Vk::Numpad0         ),
      0x61 => Some(Vk::Numpad1         ),
      0x62 => Some(Vk::Numpad2         ),
      0x63 => Some(Vk::Numpad3         ),
      0x64 => Some(Vk::Numpad4         ),
      0x65 => Some(Vk::Numpad5         ),
      0x66 => Some(Vk::Numpad6         ),
      0x67 => Some(Vk::Numpad7         ),
      0x68 => Some(Vk::Numpad8         ),
      0x69 => Some(Vk::Numpad9         ),
      0x6D => Some(Vk::Subtract        ),
      0x6F => Some(Vk::Divide          ),
      0x90 => Some(Vk::Numlock         ),
      0x6E => Some(Vk::Decimal         ),
      0x6B => Some(Vk::Add             ),
      0x6A => Some(Vk::Multiply        ),
      0x1E => Some(Vk::Accept          ),
      0x1C => Some(Vk::Convert         ),
      0x18 => Some(Vk::Final           ),
      0x1A => Some(Vk::ImeOff          ),
      0x16 => Some(Vk::ImeOn           ),
      0x17 => Some(Vk::Junja           ),
      0x15 => Some(Vk::Kana            ),
      0x19 => Some(Vk::Kanji           ),
      0x1F => Some(Vk::ModeChange      ),
      0x1D => Some(Vk::NonConvert      ),
      0x2A => Some(Vk::Print           ),
      0x6C => Some(Vk::Separator       ),
      0xB6 => Some(Vk::StartApp1       ),
      0xB7 => Some(Vk::StartApp2       ),
      _ => None,
    }
  }
  unsafe fn from_code_unchecked(code: Self::Code) -> Vk {
    unsafe { crate::numeric::convert_from_code_unchecked(code) }
  }
  }

