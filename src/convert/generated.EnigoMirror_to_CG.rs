// This file is auto-generated. Do not edit manually.


pub fn enigo_to_cgkeycode(value: Enigo) -> Option<CGKeyCode> {
  let result = match value {
    Enigo::VolumeMute     => KeyCode::MUTE                      .into_::<CGKeyCode>(),
    Enigo::VolumeDown     => KeyCode::VOLUME_DOWN               .into_::<CGKeyCode>(),
    Enigo::VolumeUp       => KeyCode::VOLUME_UP                 .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num0           => KeyCodeExt::kVK_ANSI_0             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num1           => KeyCodeExt::kVK_ANSI_1             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num2           => KeyCodeExt::kVK_ANSI_2             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num3           => KeyCodeExt::kVK_ANSI_3             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num4           => KeyCodeExt::kVK_ANSI_4             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num5           => KeyCodeExt::kVK_ANSI_5             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num6           => KeyCodeExt::kVK_ANSI_6             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num7           => KeyCodeExt::kVK_ANSI_7             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num8           => KeyCodeExt::kVK_ANSI_8             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Num9           => KeyCodeExt::kVK_ANSI_9             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::A              => KeyCodeExt::kVK_ANSI_A             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::B              => KeyCodeExt::kVK_ANSI_B             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::C              => KeyCodeExt::kVK_ANSI_C             .into_::<CGKeyCode>(),
    Enigo::CapsLock       => KeyCode::CAPS_LOCK                 .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::D              => KeyCodeExt::kVK_ANSI_D             .into_::<CGKeyCode>(),
    Enigo::Backspace      => KeyCode::DELETE                    .into_::<CGKeyCode>(),
    Enigo::Delete         => KeyCode::FORWARD_DELETE            .into_::<CGKeyCode>(),
    Enigo::DownArrow      => KeyCode::DOWN_ARROW                .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::E              => KeyCodeExt::kVK_ANSI_E             .into_::<CGKeyCode>(),
    Enigo::End            => KeyCode::END                       .into_::<CGKeyCode>(),
    Enigo::Escape         => KeyCode::ESCAPE                    .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::F              => KeyCodeExt::kVK_ANSI_F             .into_::<CGKeyCode>(),
    Enigo::F1             => KeyCode::F1                        .into_::<CGKeyCode>(),
    Enigo::F10            => KeyCode::F10                       .into_::<CGKeyCode>(),
    Enigo::F11            => KeyCode::F11                       .into_::<CGKeyCode>(),
    Enigo::F12            => KeyCode::F12                       .into_::<CGKeyCode>(),
    Enigo::F13            => KeyCode::F13                       .into_::<CGKeyCode>(),
    Enigo::F14            => KeyCode::F14                       .into_::<CGKeyCode>(),
    Enigo::F15            => KeyCode::F15                       .into_::<CGKeyCode>(),
    Enigo::F16            => KeyCode::F16                       .into_::<CGKeyCode>(),
    Enigo::F17            => KeyCode::F17                       .into_::<CGKeyCode>(),
    Enigo::F18            => KeyCode::F18                       .into_::<CGKeyCode>(),
    Enigo::F19            => KeyCode::F19                       .into_::<CGKeyCode>(),
    Enigo::F2             => KeyCode::F2                        .into_::<CGKeyCode>(),
    Enigo::F20            => KeyCode::F20                       .into_::<CGKeyCode>(),
    Enigo::F3             => KeyCode::F3                        .into_::<CGKeyCode>(),
    Enigo::F4             => KeyCode::F4                        .into_::<CGKeyCode>(),
    Enigo::F5             => KeyCode::F5                        .into_::<CGKeyCode>(),
    Enigo::F6             => KeyCode::F6                        .into_::<CGKeyCode>(),
    Enigo::F7             => KeyCode::F7                        .into_::<CGKeyCode>(),
    Enigo::F8             => KeyCode::F8                        .into_::<CGKeyCode>(),
    Enigo::F9             => KeyCode::F9                        .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::G              => KeyCodeExt::kVK_ANSI_G             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::H              => KeyCodeExt::kVK_ANSI_H             .into_::<CGKeyCode>(),
    Enigo::Help           => KeyCode::HELP                      .into_::<CGKeyCode>(),
    Enigo::Home           => KeyCode::HOME                      .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::I              => KeyCodeExt::kVK_ANSI_I             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::J              => KeyCodeExt::kVK_ANSI_J             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::K              => KeyCodeExt::kVK_ANSI_K             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::L              => KeyCodeExt::kVK_ANSI_L             .into_::<CGKeyCode>(),
    Enigo::Alt            => KeyCode::OPTION                    .into_::<CGKeyCode>(),
    Enigo::Option         => KeyCode::OPTION                    .into_::<CGKeyCode>(),
    Enigo::LeftArrow      => KeyCode::LEFT_ARROW                .into_::<CGKeyCode>(),
    Enigo::Control        => KeyCode::CONTROL                   .into_::<CGKeyCode>(),
    Enigo::LControl       => KeyCode::CONTROL                   .into_::<CGKeyCode>(),
    Enigo::Command        => KeyCode::COMMAND                   .into_::<CGKeyCode>(),
    Enigo::Meta           => KeyCode::COMMAND                   .into_::<CGKeyCode>(),
    Enigo::Super          => KeyCode::COMMAND                   .into_::<CGKeyCode>(),
    Enigo::Windows        => KeyCode::COMMAND                   .into_::<CGKeyCode>(),
    Enigo::LShift         => KeyCode::SHIFT                     .into_::<CGKeyCode>(),
    Enigo::Shift          => KeyCode::SHIFT                     .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::M              => KeyCodeExt::kVK_ANSI_M             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::N              => KeyCodeExt::kVK_ANSI_N             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::O              => KeyCodeExt::kVK_ANSI_O             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::P              => KeyCodeExt::kVK_ANSI_P             .into_::<CGKeyCode>(),
    Enigo::PageDown       => KeyCode::PAGE_DOWN                 .into_::<CGKeyCode>(),
    Enigo::PageUp         => KeyCode::PAGE_UP                   .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Q              => KeyCodeExt::kVK_ANSI_Q             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::R              => KeyCodeExt::kVK_ANSI_R             .into_::<CGKeyCode>(),
    Enigo::Return         => KeyCode::RETURN                    .into_::<CGKeyCode>(),
    Enigo::RightArrow     => KeyCode::RIGHT_ARROW               .into_::<CGKeyCode>(),
    Enigo::RControl       => KeyCode::RIGHT_CONTROL             .into_::<CGKeyCode>(),
    Enigo::RShift         => KeyCode::RIGHT_SHIFT               .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::S              => KeyCodeExt::kVK_ANSI_S             .into_::<CGKeyCode>(),
    Enigo::Space          => KeyCode::SPACE                     .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::T              => KeyCodeExt::kVK_ANSI_T             .into_::<CGKeyCode>(),
    Enigo::Tab            => KeyCode::TAB                       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::U              => KeyCodeExt::kVK_ANSI_U             .into_::<CGKeyCode>(),
    Enigo::UpArrow        => KeyCode::UP_ARROW                  .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::V              => KeyCodeExt::kVK_ANSI_V             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::W              => KeyCodeExt::kVK_ANSI_W             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::X              => KeyCodeExt::kVK_ANSI_X             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Y              => KeyCodeExt::kVK_ANSI_Y             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Z              => KeyCodeExt::kVK_ANSI_Z             .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad0        => KeyCodeExt::kVK_ANSI_Keypad0       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad1        => KeyCodeExt::kVK_ANSI_Keypad1       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad2        => KeyCodeExt::kVK_ANSI_Keypad2       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad3        => KeyCodeExt::kVK_ANSI_Keypad3       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad4        => KeyCodeExt::kVK_ANSI_Keypad4       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad5        => KeyCodeExt::kVK_ANSI_Keypad5       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad6        => KeyCodeExt::kVK_ANSI_Keypad6       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad7        => KeyCodeExt::kVK_ANSI_Keypad7       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad8        => KeyCodeExt::kVK_ANSI_Keypad8       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Numpad9        => KeyCodeExt::kVK_ANSI_Keypad9       .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Subtract       => KeyCodeExt::kVK_ANSI_KeypadMinus   .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Divide         => KeyCodeExt::kVK_ANSI_KeypadDivide  .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Decimal        => KeyCodeExt::kVK_ANSI_KeypadDecimal .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Add            => KeyCodeExt::kVK_ANSI_KeypadPlus    .into_::<CGKeyCode>(),
    #[cfg(for_windows)]
    Enigo::Multiply       => KeyCodeExt::kVK_ANSI_KeypadMultiply.into_::<CGKeyCode>(),
    #[cfg(for_macos)]
    Enigo::Function       => KeyCode::FUNCTION                  .into_::<CGKeyCode>(),
    #[cfg(for_macos)]
    Enigo::Launchpad      => CGKeyCode::from(131)               .into_::<CGKeyCode>(),
    #[cfg(for_macos)]
    Enigo::MissionControl => CGKeyCode::from(160)               .into_::<CGKeyCode>(),
    #[cfg(for_macos)]
    Enigo::RCommand       => KeyCode::RIGHT_COMMAND             .into_::<CGKeyCode>(),
    #[cfg(for_macos)]
    Enigo::ROption        => KeyCode::RIGHT_OPTION              .into_::<CGKeyCode>(),
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Enigo, CGKeyCode> for crate::convert::Converter {
  fn convert(value: Enigo) -> Option<CGKeyCode> {
    enigo_to_cgkeycode(value)
  }
}
