// This file is auto-generated. Do not edit manually.


pub fn enigo_to_cgkeycode(value: Enigo) -> Option<CGKeyCode> {
  let result = match value {
    Enigo::VolumeMute     => CGKeyCode( KeyCode::MUTE                       ),
    Enigo::VolumeDown     => CGKeyCode( KeyCode::VOLUME_DOWN                ),
    Enigo::VolumeUp       => CGKeyCode( KeyCode::VOLUME_UP                  ),
    #[cfg(for_windows)]
    Enigo::Num0           => CGKeyCode( KeyCodeExt::kVK_ANSI_0              ),
    #[cfg(for_windows)]
    Enigo::Num1           => CGKeyCode( KeyCodeExt::kVK_ANSI_1              ),
    #[cfg(for_windows)]
    Enigo::Num2           => CGKeyCode( KeyCodeExt::kVK_ANSI_2              ),
    #[cfg(for_windows)]
    Enigo::Num3           => CGKeyCode( KeyCodeExt::kVK_ANSI_3              ),
    #[cfg(for_windows)]
    Enigo::Num4           => CGKeyCode( KeyCodeExt::kVK_ANSI_4              ),
    #[cfg(for_windows)]
    Enigo::Num5           => CGKeyCode( KeyCodeExt::kVK_ANSI_5              ),
    #[cfg(for_windows)]
    Enigo::Num6           => CGKeyCode( KeyCodeExt::kVK_ANSI_6              ),
    #[cfg(for_windows)]
    Enigo::Num7           => CGKeyCode( KeyCodeExt::kVK_ANSI_7              ),
    #[cfg(for_windows)]
    Enigo::Num8           => CGKeyCode( KeyCodeExt::kVK_ANSI_8              ),
    #[cfg(for_windows)]
    Enigo::Num9           => CGKeyCode( KeyCodeExt::kVK_ANSI_9              ),
    #[cfg(for_windows)]
    Enigo::A              => CGKeyCode( KeyCodeExt::kVK_ANSI_A              ),
    #[cfg(for_windows)]
    Enigo::B              => CGKeyCode( KeyCodeExt::kVK_ANSI_B              ),
    #[cfg(for_windows)]
    Enigo::C              => CGKeyCode( KeyCodeExt::kVK_ANSI_C              ),
    Enigo::CapsLock       => CGKeyCode( KeyCode::CAPS_LOCK                  ),
    #[cfg(for_windows)]
    Enigo::D              => CGKeyCode( KeyCodeExt::kVK_ANSI_D              ),
    Enigo::Backspace      => CGKeyCode( KeyCode::DELETE                     ),
    Enigo::Delete         => CGKeyCode( KeyCode::FORWARD_DELETE             ),
    Enigo::DownArrow      => CGKeyCode( KeyCode::DOWN_ARROW                 ),
    #[cfg(for_windows)]
    Enigo::E              => CGKeyCode( KeyCodeExt::kVK_ANSI_E              ),
    Enigo::End            => CGKeyCode( KeyCode::END                        ),
    Enigo::Escape         => CGKeyCode( KeyCode::ESCAPE                     ),
    #[cfg(for_windows)]
    Enigo::F              => CGKeyCode( KeyCodeExt::kVK_ANSI_F              ),
    Enigo::F1             => CGKeyCode( KeyCode::F1                         ),
    Enigo::F10            => CGKeyCode( KeyCode::F10                        ),
    Enigo::F11            => CGKeyCode( KeyCode::F11                        ),
    Enigo::F12            => CGKeyCode( KeyCode::F12                        ),
    Enigo::F13            => CGKeyCode( KeyCode::F13                        ),
    Enigo::F14            => CGKeyCode( KeyCode::F14                        ),
    Enigo::F15            => CGKeyCode( KeyCode::F15                        ),
    Enigo::F16            => CGKeyCode( KeyCode::F16                        ),
    Enigo::F17            => CGKeyCode( KeyCode::F17                        ),
    Enigo::F18            => CGKeyCode( KeyCode::F18                        ),
    Enigo::F19            => CGKeyCode( KeyCode::F19                        ),
    Enigo::F2             => CGKeyCode( KeyCode::F2                         ),
    Enigo::F20            => CGKeyCode( KeyCode::F20                        ),
    Enigo::F3             => CGKeyCode( KeyCode::F3                         ),
    Enigo::F4             => CGKeyCode( KeyCode::F4                         ),
    Enigo::F5             => CGKeyCode( KeyCode::F5                         ),
    Enigo::F6             => CGKeyCode( KeyCode::F6                         ),
    Enigo::F7             => CGKeyCode( KeyCode::F7                         ),
    Enigo::F8             => CGKeyCode( KeyCode::F8                         ),
    Enigo::F9             => CGKeyCode( KeyCode::F9                         ),
    #[cfg(for_windows)]
    Enigo::G              => CGKeyCode( KeyCodeExt::kVK_ANSI_G              ),
    #[cfg(for_windows)]
    Enigo::H              => CGKeyCode( KeyCodeExt::kVK_ANSI_H              ),
    Enigo::Help           => CGKeyCode( KeyCode::HELP                       ),
    Enigo::Home           => CGKeyCode( KeyCode::HOME                       ),
    #[cfg(for_windows)]
    Enigo::I              => CGKeyCode( KeyCodeExt::kVK_ANSI_I              ),
    #[cfg(for_windows)]
    Enigo::J              => CGKeyCode( KeyCodeExt::kVK_ANSI_J              ),
    #[cfg(for_windows)]
    Enigo::K              => CGKeyCode( KeyCodeExt::kVK_ANSI_K              ),
    #[cfg(for_windows)]
    Enigo::L              => CGKeyCode( KeyCodeExt::kVK_ANSI_L              ),
    #[cfg(any(for_windows, for_linux))]
    Enigo::LMenu          => CGKeyCode( KeyCode::OPTION                     ),
    Enigo::Alt            => CGKeyCode( KeyCode::OPTION                     ),
    Enigo::Option         => CGKeyCode( KeyCode::OPTION                     ),
    Enigo::LeftArrow      => CGKeyCode( KeyCode::LEFT_ARROW                 ),
    Enigo::Control        => CGKeyCode( KeyCode::CONTROL                    ),
    Enigo::LControl       => CGKeyCode( KeyCode::CONTROL                    ),
    #[cfg(for_windows)]
    Enigo::LWin           => CGKeyCode( KeyCode::COMMAND                    ),
    Enigo::Command        => CGKeyCode( KeyCode::COMMAND                    ),
    Enigo::Meta           => CGKeyCode( KeyCode::COMMAND                    ),
    Enigo::Super          => CGKeyCode( KeyCode::COMMAND                    ),
    Enigo::Windows        => CGKeyCode( KeyCode::COMMAND                    ),
    Enigo::LShift         => CGKeyCode( KeyCode::SHIFT                      ),
    Enigo::Shift          => CGKeyCode( KeyCode::SHIFT                      ),
    #[cfg(for_windows)]
    Enigo::M              => CGKeyCode( KeyCodeExt::kVK_ANSI_M              ),
    #[cfg(for_windows)]
    Enigo::N              => CGKeyCode( KeyCodeExt::kVK_ANSI_N              ),
    #[cfg(for_windows)]
    Enigo::O              => CGKeyCode( KeyCodeExt::kVK_ANSI_O              ),
    #[cfg(for_windows)]
    Enigo::P              => CGKeyCode( KeyCodeExt::kVK_ANSI_P              ),
    Enigo::PageDown       => CGKeyCode( KeyCode::PAGE_DOWN                  ),
    Enigo::PageUp         => CGKeyCode( KeyCode::PAGE_UP                    ),
    #[cfg(for_windows)]
    Enigo::Q              => CGKeyCode( KeyCodeExt::kVK_ANSI_Q              ),
    #[cfg(for_windows)]
    Enigo::R              => CGKeyCode( KeyCodeExt::kVK_ANSI_R              ),
    Enigo::Return         => CGKeyCode( KeyCode::RETURN                     ),
    #[cfg(for_windows)]
    Enigo::RMenu          => CGKeyCode( KeyCode::RIGHT_OPTION               ),
    #[cfg(for_macos)]
    Enigo::ROption        => CGKeyCode( KeyCode::RIGHT_OPTION               ),
    Enigo::RightArrow     => CGKeyCode( KeyCode::RIGHT_ARROW                ),
    Enigo::RControl       => CGKeyCode( KeyCode::RIGHT_CONTROL              ),
    #[cfg(for_windows)]
    Enigo::RWin           => CGKeyCode( KeyCode::RIGHT_COMMAND              ),
    #[cfg(for_macos)]
    Enigo::RCommand       => CGKeyCode( KeyCode::RIGHT_COMMAND              ),
    Enigo::RShift         => CGKeyCode( KeyCode::RIGHT_SHIFT                ),
    #[cfg(for_windows)]
    Enigo::S              => CGKeyCode( KeyCodeExt::kVK_ANSI_S              ),
    Enigo::Space          => CGKeyCode( KeyCode::SPACE                      ),
    #[cfg(for_windows)]
    Enigo::T              => CGKeyCode( KeyCodeExt::kVK_ANSI_T              ),
    Enigo::Tab            => CGKeyCode( KeyCode::TAB                        ),
    #[cfg(for_windows)]
    Enigo::U              => CGKeyCode( KeyCodeExt::kVK_ANSI_U              ),
    Enigo::UpArrow        => CGKeyCode( KeyCode::UP_ARROW                   ),
    #[cfg(for_windows)]
    Enigo::V              => CGKeyCode( KeyCodeExt::kVK_ANSI_V              ),
    #[cfg(for_windows)]
    Enigo::W              => CGKeyCode( KeyCodeExt::kVK_ANSI_W              ),
    #[cfg(for_windows)]
    Enigo::X              => CGKeyCode( KeyCodeExt::kVK_ANSI_X              ),
    #[cfg(for_windows)]
    Enigo::Y              => CGKeyCode( KeyCodeExt::kVK_ANSI_Y              ),
    #[cfg(for_windows)]
    Enigo::Z              => CGKeyCode( KeyCodeExt::kVK_ANSI_Z              ),
    #[cfg(for_windows)]
    Enigo::Numpad0        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad0        ),
    #[cfg(for_windows)]
    Enigo::Numpad1        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad1        ),
    #[cfg(for_windows)]
    Enigo::Numpad2        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad2        ),
    #[cfg(for_windows)]
    Enigo::Numpad3        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad3        ),
    #[cfg(for_windows)]
    Enigo::Numpad4        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad4        ),
    #[cfg(for_windows)]
    Enigo::Numpad5        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad5        ),
    #[cfg(for_windows)]
    Enigo::Numpad6        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad6        ),
    #[cfg(for_windows)]
    Enigo::Numpad7        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad7        ),
    #[cfg(for_windows)]
    Enigo::Numpad8        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad8        ),
    #[cfg(for_windows)]
    Enigo::Numpad9        => CGKeyCode( KeyCodeExt::kVK_ANSI_Keypad9        ),
    #[cfg(for_windows)]
    Enigo::Subtract       => CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadMinus    ),
    #[cfg(for_windows)]
    Enigo::Divide         => CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadDivide   ),
    #[cfg(for_windows)]
    Enigo::Decimal        => CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadDecimal  ),
    #[cfg(for_windows)]
    Enigo::Add            => CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadPlus     ),
    #[cfg(for_windows)]
    Enigo::Multiply       => CGKeyCode( KeyCodeExt::kVK_ANSI_KeypadMultiply ),
    #[cfg(for_macos)]
    Enigo::Function       => CGKeyCode( KeyCode::FUNCTION                   ),
    #[cfg(for_macos)]
    Enigo::Launchpad      => CGKeyCode( CGKeyCode(131).0                    ),
    #[cfg(for_macos)]
    Enigo::MissionControl => CGKeyCode( CGKeyCode(160).0                    ),
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Enigo, CGKeyCode> for crate::convert::Converter {
  fn convert(value: Enigo) -> Option<CGKeyCode> {
    enigo_to_cgkeycode(value)
  }
}
