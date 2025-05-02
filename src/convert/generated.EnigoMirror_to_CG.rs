// This file is auto-generated. Do not edit manually.


pub fn enigo_to_cgkeycode(value: Enigo) -> Option<CGKeyCode> {
  let result = match value {
    Enigo::VolumeMute     => KeyCode::MUTE                      .into(),
    Enigo::VolumeDown     => KeyCode::VOLUME_DOWN               .into(),
    Enigo::VolumeUp       => KeyCode::VOLUME_UP                 .into(),
    #[cfg(for_windows)]
    Enigo::Num0           => KeyCodeExt::kVK_ANSI_0             .into(),
    #[cfg(for_windows)]
    Enigo::Num1           => KeyCodeExt::kVK_ANSI_1             .into(),
    #[cfg(for_windows)]
    Enigo::Num2           => KeyCodeExt::kVK_ANSI_2             .into(),
    #[cfg(for_windows)]
    Enigo::Num3           => KeyCodeExt::kVK_ANSI_3             .into(),
    #[cfg(for_windows)]
    Enigo::Num4           => KeyCodeExt::kVK_ANSI_4             .into(),
    #[cfg(for_windows)]
    Enigo::Num5           => KeyCodeExt::kVK_ANSI_5             .into(),
    #[cfg(for_windows)]
    Enigo::Num6           => KeyCodeExt::kVK_ANSI_6             .into(),
    #[cfg(for_windows)]
    Enigo::Num7           => KeyCodeExt::kVK_ANSI_7             .into(),
    #[cfg(for_windows)]
    Enigo::Num8           => KeyCodeExt::kVK_ANSI_8             .into(),
    #[cfg(for_windows)]
    Enigo::Num9           => KeyCodeExt::kVK_ANSI_9             .into(),
    #[cfg(for_windows)]
    Enigo::A              => KeyCodeExt::kVK_ANSI_A             .into(),
    #[cfg(for_windows)]
    Enigo::B              => KeyCodeExt::kVK_ANSI_B             .into(),
    #[cfg(for_windows)]
    Enigo::C              => KeyCodeExt::kVK_ANSI_C             .into(),
    Enigo::CapsLock       => KeyCode::CAPS_LOCK                 .into(),
    #[cfg(for_windows)]
    Enigo::D              => KeyCodeExt::kVK_ANSI_D             .into(),
    Enigo::Backspace      => KeyCode::DELETE                    .into(),
    Enigo::Delete         => KeyCode::FORWARD_DELETE            .into(),
    Enigo::DownArrow      => KeyCode::DOWN_ARROW                .into(),
    #[cfg(for_windows)]
    Enigo::E              => KeyCodeExt::kVK_ANSI_E             .into(),
    Enigo::End            => KeyCode::END                       .into(),
    Enigo::Escape         => KeyCode::ESCAPE                    .into(),
    #[cfg(for_windows)]
    Enigo::F              => KeyCodeExt::kVK_ANSI_F             .into(),
    Enigo::F1             => KeyCode::F1                        .into(),
    Enigo::F10            => KeyCode::F10                       .into(),
    Enigo::F11            => KeyCode::F11                       .into(),
    Enigo::F12            => KeyCode::F12                       .into(),
    Enigo::F13            => KeyCode::F13                       .into(),
    Enigo::F14            => KeyCode::F14                       .into(),
    Enigo::F15            => KeyCode::F15                       .into(),
    Enigo::F16            => KeyCode::F16                       .into(),
    Enigo::F17            => KeyCode::F17                       .into(),
    Enigo::F18            => KeyCode::F18                       .into(),
    Enigo::F19            => KeyCode::F19                       .into(),
    Enigo::F2             => KeyCode::F2                        .into(),
    Enigo::F20            => KeyCode::F20                       .into(),
    Enigo::F3             => KeyCode::F3                        .into(),
    Enigo::F4             => KeyCode::F4                        .into(),
    Enigo::F5             => KeyCode::F5                        .into(),
    Enigo::F6             => KeyCode::F6                        .into(),
    Enigo::F7             => KeyCode::F7                        .into(),
    Enigo::F8             => KeyCode::F8                        .into(),
    Enigo::F9             => KeyCode::F9                        .into(),
    #[cfg(for_windows)]
    Enigo::G              => KeyCodeExt::kVK_ANSI_G             .into(),
    #[cfg(for_windows)]
    Enigo::H              => KeyCodeExt::kVK_ANSI_H             .into(),
    Enigo::Help           => KeyCode::HELP                      .into(),
    Enigo::Home           => KeyCode::HOME                      .into(),
    #[cfg(for_windows)]
    Enigo::I              => KeyCodeExt::kVK_ANSI_I             .into(),
    #[cfg(for_windows)]
    Enigo::J              => KeyCodeExt::kVK_ANSI_J             .into(),
    #[cfg(for_windows)]
    Enigo::K              => KeyCodeExt::kVK_ANSI_K             .into(),
    #[cfg(for_windows)]
    Enigo::L              => KeyCodeExt::kVK_ANSI_L             .into(),
    Enigo::Alt            => KeyCode::OPTION                    .into(),
    Enigo::Option         => KeyCode::OPTION                    .into(),
    Enigo::LeftArrow      => KeyCode::LEFT_ARROW                .into(),
    Enigo::Control        => KeyCode::CONTROL                   .into(),
    Enigo::LControl       => KeyCode::CONTROL                   .into(),
    Enigo::Command        => KeyCode::COMMAND                   .into(),
    Enigo::Meta           => KeyCode::COMMAND                   .into(),
    Enigo::Super          => KeyCode::COMMAND                   .into(),
    Enigo::Windows        => KeyCode::COMMAND                   .into(),
    Enigo::LShift         => KeyCode::SHIFT                     .into(),
    Enigo::Shift          => KeyCode::SHIFT                     .into(),
    #[cfg(for_windows)]
    Enigo::M              => KeyCodeExt::kVK_ANSI_M             .into(),
    #[cfg(for_windows)]
    Enigo::N              => KeyCodeExt::kVK_ANSI_N             .into(),
    #[cfg(for_windows)]
    Enigo::O              => KeyCodeExt::kVK_ANSI_O             .into(),
    #[cfg(for_windows)]
    Enigo::P              => KeyCodeExt::kVK_ANSI_P             .into(),
    Enigo::PageDown       => KeyCode::PAGE_DOWN                 .into(),
    Enigo::PageUp         => KeyCode::PAGE_UP                   .into(),
    #[cfg(for_windows)]
    Enigo::Q              => KeyCodeExt::kVK_ANSI_Q             .into(),
    #[cfg(for_windows)]
    Enigo::R              => KeyCodeExt::kVK_ANSI_R             .into(),
    Enigo::Return         => KeyCode::RETURN                    .into(),
    Enigo::RightArrow     => KeyCode::RIGHT_ARROW               .into(),
    Enigo::RControl       => KeyCode::RIGHT_CONTROL             .into(),
    Enigo::RShift         => KeyCode::RIGHT_SHIFT               .into(),
    #[cfg(for_windows)]
    Enigo::S              => KeyCodeExt::kVK_ANSI_S             .into(),
    Enigo::Space          => KeyCode::SPACE                     .into(),
    #[cfg(for_windows)]
    Enigo::T              => KeyCodeExt::kVK_ANSI_T             .into(),
    Enigo::Tab            => KeyCode::TAB                       .into(),
    #[cfg(for_windows)]
    Enigo::U              => KeyCodeExt::kVK_ANSI_U             .into(),
    Enigo::UpArrow        => KeyCode::UP_ARROW                  .into(),
    #[cfg(for_windows)]
    Enigo::V              => KeyCodeExt::kVK_ANSI_V             .into(),
    #[cfg(for_windows)]
    Enigo::W              => KeyCodeExt::kVK_ANSI_W             .into(),
    #[cfg(for_windows)]
    Enigo::X              => KeyCodeExt::kVK_ANSI_X             .into(),
    #[cfg(for_windows)]
    Enigo::Y              => KeyCodeExt::kVK_ANSI_Y             .into(),
    #[cfg(for_windows)]
    Enigo::Z              => KeyCodeExt::kVK_ANSI_Z             .into(),
    #[cfg(for_windows)]
    Enigo::Numpad0        => KeyCodeExt::kVK_ANSI_Keypad0       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad1        => KeyCodeExt::kVK_ANSI_Keypad1       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad2        => KeyCodeExt::kVK_ANSI_Keypad2       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad3        => KeyCodeExt::kVK_ANSI_Keypad3       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad4        => KeyCodeExt::kVK_ANSI_Keypad4       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad5        => KeyCodeExt::kVK_ANSI_Keypad5       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad6        => KeyCodeExt::kVK_ANSI_Keypad6       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad7        => KeyCodeExt::kVK_ANSI_Keypad7       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad8        => KeyCodeExt::kVK_ANSI_Keypad8       .into(),
    #[cfg(for_windows)]
    Enigo::Numpad9        => KeyCodeExt::kVK_ANSI_Keypad9       .into(),
    #[cfg(for_windows)]
    Enigo::Subtract       => KeyCodeExt::kVK_ANSI_KeypadMinus   .into(),
    #[cfg(for_windows)]
    Enigo::Divide         => KeyCodeExt::kVK_ANSI_KeypadDivide  .into(),
    #[cfg(for_windows)]
    Enigo::Decimal        => KeyCodeExt::kVK_ANSI_KeypadDecimal .into(),
    #[cfg(for_windows)]
    Enigo::Add            => KeyCodeExt::kVK_ANSI_KeypadPlus    .into(),
    #[cfg(for_windows)]
    Enigo::Multiply       => KeyCodeExt::kVK_ANSI_KeypadMultiply.into(),
    #[cfg(for_macos)]
    Enigo::Function       => KeyCode::FUNCTION                  .into(),
    #[cfg(for_macos)]
    Enigo::Launchpad      => CGKeyCode::from(131)               .into(),
    #[cfg(for_macos)]
    Enigo::MissionControl => CGKeyCode::from(160)               .into(),
    #[cfg(for_macos)]
    Enigo::RCommand       => KeyCode::RIGHT_COMMAND             .into(),
    #[cfg(for_macos)]
    Enigo::ROption        => KeyCode::RIGHT_OPTION              .into(),
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Enigo, CGKeyCode> for crate::convert::Converter {
  fn convert(value: Enigo) -> Option<CGKeyCode> {
    enigo_to_cgkeycode(value)
  }
}
