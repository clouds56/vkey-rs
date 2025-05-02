// This file is auto-generated. Do not edit manually.


pub fn enigo_to_vk(value: Enigo) -> Option<Vk> {
  let result = match value {
    #[cfg(for_windows)]
    Enigo::LButton           => Vk::MouseLeft       ,
    #[cfg(for_windows)]
    Enigo::RButton           => Vk::MouseRight      ,
    #[cfg(for_windows)]
    Enigo::MButton           => Vk::MouseMiddle     ,
    #[cfg(for_windows)]
    Enigo::XButton1          => Vk::MouseX1         ,
    #[cfg(for_windows)]
    Enigo::XButton2          => Vk::MouseX2         ,
    #[cfg(for_windows)]
    Enigo::BrowserBack       => Vk::BrowserBack     ,
    #[cfg(for_windows)]
    Enigo::BrowserFavorites  => Vk::BrowserFavorites,
    #[cfg(for_windows)]
    Enigo::BrowserForward    => Vk::BrowserForward  ,
    #[cfg(for_windows)]
    Enigo::BrowserHome       => Vk::BrowserHome     ,
    #[cfg(for_windows)]
    Enigo::BrowserRefresh    => Vk::BrowserRefresh  ,
    #[cfg(for_windows)]
    Enigo::BrowserSearch     => Vk::BrowserSearch   ,
    #[cfg(for_windows)]
    Enigo::BrowserStop       => Vk::BrowserStop     ,
    #[cfg(for_windows)]
    Enigo::LaunchMail        => Vk::StartMail       ,
    #[cfg(for_windows)]
    Enigo::LaunchMediaSelect => Vk::SelectMedia     ,
    Enigo::VolumeMute        => Vk::VolumeMute      ,
    Enigo::MediaPlayPause    => Vk::MediaPlayPause  ,
    Enigo::MediaNextTrack    => Vk::NextTrack       ,
    Enigo::MediaPrevTrack    => Vk::PrevTrack       ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::MediaStop         => Vk::MediaStop       ,
    Enigo::VolumeDown        => Vk::VolumeDown      ,
    Enigo::VolumeUp          => Vk::VolumeUp        ,
    #[cfg(for_windows)]
    Enigo::Sleep             => Vk::Sleep           ,
    #[cfg(for_windows)]
    Enigo::Num0              => Vk::_0              ,
    #[cfg(for_windows)]
    Enigo::Num1              => Vk::_1              ,
    #[cfg(for_windows)]
    Enigo::Num2              => Vk::_2              ,
    #[cfg(for_windows)]
    Enigo::Num3              => Vk::_3              ,
    #[cfg(for_windows)]
    Enigo::Num4              => Vk::_4              ,
    #[cfg(for_windows)]
    Enigo::Num5              => Vk::_5              ,
    #[cfg(for_windows)]
    Enigo::Num6              => Vk::_6              ,
    #[cfg(for_windows)]
    Enigo::Num7              => Vk::_7              ,
    #[cfg(for_windows)]
    Enigo::Num8              => Vk::_8              ,
    #[cfg(for_windows)]
    Enigo::Num9              => Vk::_9              ,
    #[cfg(for_windows)]
    Enigo::A                 => Vk::A               ,
    #[cfg(for_windows)]
    Enigo::Apps              => Vk::Apps            ,
    #[cfg(for_windows)]
    Enigo::B                 => Vk::B               ,
    #[cfg(for_windows)]
    Enigo::OEM5              => Vk::Oem5            ,
    #[cfg(for_windows)]
    Enigo::C                 => Vk::C               ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Cancel            => Vk::Cancel          ,
    Enigo::CapsLock          => Vk::CapsLock        ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Clear             => Vk::Clear           ,
    #[cfg(for_windows)]
    Enigo::OEMComma          => Vk::Comma           ,
    #[cfg(for_windows)]
    Enigo::D                 => Vk::D               ,
    #[cfg(for_windows)]
    Enigo::OEMMinus          => Vk::Minus           ,
    Enigo::Backspace         => Vk::Backspace       ,
    Enigo::Delete            => Vk::Delete          ,
    Enigo::DownArrow         => Vk::DownArrow       ,
    #[cfg(for_windows)]
    Enigo::E                 => Vk::E               ,
    Enigo::End               => Vk::End             ,
    #[cfg(for_windows)]
    Enigo::OEMPlus           => Vk::Plus            ,
    Enigo::Escape            => Vk::Escape          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Execute           => Vk::Execute         ,
    #[cfg(for_windows)]
    Enigo::F                 => Vk::F               ,
    Enigo::F1                => Vk::F1              ,
    Enigo::F10               => Vk::F10             ,
    Enigo::F11               => Vk::F11             ,
    Enigo::F12               => Vk::F12             ,
    Enigo::F13               => Vk::F13             ,
    Enigo::F14               => Vk::F14             ,
    Enigo::F15               => Vk::F15             ,
    Enigo::F16               => Vk::F16             ,
    Enigo::F17               => Vk::F17             ,
    Enigo::F18               => Vk::F18             ,
    Enigo::F19               => Vk::F19             ,
    Enigo::F2                => Vk::F2              ,
    Enigo::F20               => Vk::F20             ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F21               => Vk::F21             ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F22               => Vk::F22             ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F23               => Vk::F23             ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F24               => Vk::F24             ,
    Enigo::F3                => Vk::F3              ,
    Enigo::F4                => Vk::F4              ,
    Enigo::F5                => Vk::F5              ,
    Enigo::F6                => Vk::F6              ,
    Enigo::F7                => Vk::F7              ,
    Enigo::F8                => Vk::F8              ,
    Enigo::F9                => Vk::F9              ,
    #[cfg(for_windows)]
    Enigo::OEM2              => Vk::Oem2            ,
    #[cfg(for_windows)]
    Enigo::G                 => Vk::G               ,
    #[cfg(for_windows)]
    Enigo::OEM3              => Vk::Oem3            ,
    #[cfg(for_windows)]
    Enigo::H                 => Vk::H               ,
    Enigo::Help              => Vk::Help            ,
    Enigo::Home              => Vk::Home            ,
    #[cfg(for_windows)]
    Enigo::I                 => Vk::I               ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Insert            => Vk::Insert          ,
    #[cfg(for_windows)]
    Enigo::J                 => Vk::J               ,
    #[cfg(for_windows)]
    Enigo::K                 => Vk::K               ,
    #[cfg(for_windows)]
    Enigo::L                 => Vk::L               ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::LMenu             => Vk::LeftMenu        ,
    Enigo::Alt               => Vk::Alt             ,
    Enigo::Option            => Vk::Alt             ,
    #[cfg(for_windows)]
    Enigo::OEM7              => Vk::Oem7            ,
    Enigo::LeftArrow         => Vk::LeftArrow       ,
    #[cfg(for_windows)]
    Enigo::OEM4              => Vk::Oem4            ,
    Enigo::Control           => Vk::Control         ,
    Enigo::LControl          => Vk::LeftControl     ,
    #[cfg(for_windows)]
    Enigo::LWin              => Vk::LeftWin         ,
    Enigo::Command           => Vk::LeftWin         ,
    Enigo::Meta              => Vk::LeftWin         ,
    Enigo::Super             => Vk::LeftWin         ,
    Enigo::Windows           => Vk::LeftWin         ,
    Enigo::LShift            => Vk::LeftShift       ,
    Enigo::Shift             => Vk::Shift           ,
    #[cfg(for_windows)]
    Enigo::M                 => Vk::M               ,
    #[cfg(for_windows)]
    Enigo::N                 => Vk::N               ,
    #[cfg(for_windows)]
    Enigo::OEM102            => Vk::Oem102          ,
    #[cfg(for_windows)]
    Enigo::O                 => Vk::O               ,
    #[cfg(for_windows)]
    Enigo::P                 => Vk::P               ,
    Enigo::PageDown          => Vk::PageDown        ,
    Enigo::PageUp            => Vk::PageUp          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Pause             => Vk::Pause           ,
    #[cfg(for_windows)]
    Enigo::OEMPeriod         => Vk::Period          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::PrintScr          => Vk::PrintScreen     ,
    #[cfg(for_windows)]
    Enigo::Snapshot          => Vk::PrintScreen     ,
    #[cfg(for_windows)]
    Enigo::Q                 => Vk::Q               ,
    #[cfg(for_windows)]
    Enigo::R                 => Vk::R               ,
    Enigo::Return            => Vk::Enter           ,
    #[cfg(for_windows)]
    Enigo::RMenu             => Vk::RightMenu       ,
    #[cfg(for_macos)]
    Enigo::ROption           => Vk::RightMenu       ,
    Enigo::RightArrow        => Vk::RightArrow      ,
    #[cfg(for_windows)]
    Enigo::OEM6              => Vk::Oem6            ,
    Enigo::RControl          => Vk::RightControl    ,
    #[cfg(for_windows)]
    Enigo::RWin              => Vk::RightWin        ,
    #[cfg(for_macos)]
    Enigo::RCommand          => Vk::RightWin        ,
    Enigo::RShift            => Vk::RightShift      ,
    #[cfg(for_windows)]
    Enigo::S                 => Vk::S               ,
    #[cfg(for_windows)]
    Enigo::Scroll            => Vk::Scroll          ,
    #[cfg(for_linux)]
    Enigo::ScrollLock        => Vk::Scroll          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Select            => Vk::Select          ,
    #[cfg(for_windows)]
    Enigo::OEM1              => Vk::Oem1            ,
    Enigo::Space             => Vk::Space           ,
    #[cfg(for_windows)]
    Enigo::T                 => Vk::T               ,
    Enigo::Tab               => Vk::Tab             ,
    #[cfg(for_windows)]
    Enigo::U                 => Vk::U               ,
    Enigo::UpArrow           => Vk::UpArrow         ,
    #[cfg(for_windows)]
    Enigo::V                 => Vk::V               ,
    #[cfg(for_windows)]
    Enigo::W                 => Vk::W               ,
    #[cfg(for_windows)]
    Enigo::X                 => Vk::X               ,
    #[cfg(for_windows)]
    Enigo::Y                 => Vk::Y               ,
    #[cfg(for_windows)]
    Enigo::Z                 => Vk::Z               ,
    #[cfg(for_windows)]
    Enigo::Numpad0           => Vk::Numpad0         ,
    #[cfg(for_windows)]
    Enigo::Numpad1           => Vk::Numpad1         ,
    #[cfg(for_windows)]
    Enigo::Numpad2           => Vk::Numpad2         ,
    #[cfg(for_windows)]
    Enigo::Numpad3           => Vk::Numpad3         ,
    #[cfg(for_windows)]
    Enigo::Numpad4           => Vk::Numpad4         ,
    #[cfg(for_windows)]
    Enigo::Numpad5           => Vk::Numpad5         ,
    #[cfg(for_windows)]
    Enigo::Numpad6           => Vk::Numpad6         ,
    #[cfg(for_windows)]
    Enigo::Numpad7           => Vk::Numpad7         ,
    #[cfg(for_windows)]
    Enigo::Numpad8           => Vk::Numpad8         ,
    #[cfg(for_windows)]
    Enigo::Numpad9           => Vk::Numpad9         ,
    #[cfg(for_windows)]
    Enigo::Subtract          => Vk::Subtract        ,
    #[cfg(for_windows)]
    Enigo::Divide            => Vk::Divide          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Numlock           => Vk::Numlock         ,
    #[cfg(for_windows)]
    Enigo::Decimal           => Vk::Decimal         ,
    #[cfg(for_windows)]
    Enigo::Add               => Vk::Add             ,
    #[cfg(for_windows)]
    Enigo::Multiply          => Vk::Multiply        ,
    #[cfg(for_windows)]
    Enigo::Accept            => Vk::Accept          ,
    #[cfg(for_windows)]
    Enigo::Convert           => Vk::Convert         ,
    #[cfg(for_windows)]
    Enigo::Final             => Vk::Final           ,
    #[cfg(for_windows)]
    Enigo::IMEOff            => Vk::ImeOff          ,
    #[cfg(for_windows)]
    Enigo::IMEOn             => Vk::ImeOn           ,
    #[cfg(for_windows)]
    Enigo::Junja             => Vk::Junja           ,
    #[cfg(for_windows)]
    Enigo::Kana              => Vk::Kana            ,
    #[cfg(for_windows)]
    Enigo::Hangeul           => Vk::Kana            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Hangul            => Vk::Kana            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Kanji             => Vk::Kanji           ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Hanja             => Vk::Kanji           ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::ModeChange        => Vk::ModeChange      ,
    #[cfg(for_windows)]
    Enigo::NonConvert        => Vk::NonConvert      ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Print             => Vk::Print           ,
    #[cfg(for_windows)]
    Enigo::Separator         => Vk::Separator       ,
    #[cfg(for_windows)]
    Enigo::LaunchApp1        => Vk::StartApp1       ,
    #[cfg(for_windows)]
    Enigo::LaunchApp2        => Vk::StartApp2       ,
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Enigo, Vk> for crate::convert::Converter {
  fn convert(value: Enigo) -> Option<Vk> {
    enigo_to_vk(value)
  }
}
