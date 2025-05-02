// This file is auto-generated. Do not edit manually.


pub fn enigo_to_vk(value: Enigo) -> Option<Vk> {
  let result = match value {
    #[cfg(target_os = "windows")]
    Enigo::LButton           => Vk::MouseLeft       ,
    #[cfg(target_os = "windows")]
    Enigo::RButton           => Vk::MouseRight      ,
    #[cfg(target_os = "windows")]
    Enigo::MButton           => Vk::MouseMiddle     ,
    #[cfg(target_os = "windows")]
    Enigo::XButton1          => Vk::MouseX1         ,
    #[cfg(target_os = "windows")]
    Enigo::XButton2          => Vk::MouseX2         ,
    #[cfg(target_os = "windows")]
    Enigo::BrowserBack       => Vk::BrowserBack     ,
    #[cfg(target_os = "windows")]
    Enigo::BrowserFavorites  => Vk::BrowserFavorites,
    #[cfg(target_os = "windows")]
    Enigo::BrowserForward    => Vk::BrowserForward  ,
    #[cfg(target_os = "windows")]
    Enigo::BrowserHome       => Vk::BrowserHome     ,
    #[cfg(target_os = "windows")]
    Enigo::BrowserRefresh    => Vk::BrowserRefresh  ,
    #[cfg(target_os = "windows")]
    Enigo::BrowserSearch     => Vk::BrowserSearch   ,
    #[cfg(target_os = "windows")]
    Enigo::BrowserStop       => Vk::BrowserStop     ,
    #[cfg(target_os = "windows")]
    Enigo::LaunchMail        => Vk::StartMail       ,
    #[cfg(target_os = "windows")]
    Enigo::LaunchMediaSelect => Vk::SelectMedia     ,
    Enigo::VolumeMute        => Vk::VolumeMute      ,
    Enigo::MediaPlayPause    => Vk::MediaPlayPause  ,
    Enigo::MediaNextTrack    => Vk::NextTrack       ,
    Enigo::MediaPrevTrack    => Vk::PrevTrack       ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::MediaStop         => Vk::MediaStop       ,
    Enigo::VolumeDown        => Vk::VolumeDown      ,
    Enigo::VolumeUp          => Vk::VolumeUp        ,
    #[cfg(target_os = "windows")]
    Enigo::Sleep             => Vk::Sleep           ,
    #[cfg(target_os = "windows")]
    Enigo::Num0              => Vk::_0              ,
    #[cfg(target_os = "windows")]
    Enigo::Num1              => Vk::_1              ,
    #[cfg(target_os = "windows")]
    Enigo::Num2              => Vk::_2              ,
    #[cfg(target_os = "windows")]
    Enigo::Num3              => Vk::_3              ,
    #[cfg(target_os = "windows")]
    Enigo::Num4              => Vk::_4              ,
    #[cfg(target_os = "windows")]
    Enigo::Num5              => Vk::_5              ,
    #[cfg(target_os = "windows")]
    Enigo::Num6              => Vk::_6              ,
    #[cfg(target_os = "windows")]
    Enigo::Num7              => Vk::_7              ,
    #[cfg(target_os = "windows")]
    Enigo::Num8              => Vk::_8              ,
    #[cfg(target_os = "windows")]
    Enigo::Num9              => Vk::_9              ,
    #[cfg(target_os = "windows")]
    Enigo::A                 => Vk::A               ,
    #[cfg(target_os = "windows")]
    Enigo::Apps              => Vk::Apps            ,
    #[cfg(target_os = "windows")]
    Enigo::B                 => Vk::B               ,
    #[cfg(target_os = "windows")]
    Enigo::OEM5              => Vk::Oem5            ,
    #[cfg(target_os = "windows")]
    Enigo::C                 => Vk::C               ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Cancel            => Vk::Cancel          ,
    Enigo::CapsLock          => Vk::CapsLock        ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Clear             => Vk::Clear           ,
    #[cfg(target_os = "windows")]
    Enigo::OEMComma          => Vk::Comma           ,
    #[cfg(target_os = "windows")]
    Enigo::D                 => Vk::D               ,
    #[cfg(target_os = "windows")]
    Enigo::OEMMinus          => Vk::Minus           ,
    Enigo::Backspace         => Vk::Backspace       ,
    Enigo::Delete            => Vk::Delete          ,
    Enigo::DownArrow         => Vk::DownArrow       ,
    #[cfg(target_os = "windows")]
    Enigo::E                 => Vk::E               ,
    Enigo::End               => Vk::End             ,
    #[cfg(target_os = "windows")]
    Enigo::OEMPlus           => Vk::Plus            ,
    Enigo::Escape            => Vk::Escape          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Execute           => Vk::Execute         ,
    #[cfg(target_os = "windows")]
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
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F21               => Vk::F21             ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F22               => Vk::F22             ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F23               => Vk::F23             ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F24               => Vk::F24             ,
    Enigo::F3                => Vk::F3              ,
    Enigo::F4                => Vk::F4              ,
    Enigo::F5                => Vk::F5              ,
    Enigo::F6                => Vk::F6              ,
    Enigo::F7                => Vk::F7              ,
    Enigo::F8                => Vk::F8              ,
    Enigo::F9                => Vk::F9              ,
    #[cfg(target_os = "windows")]
    Enigo::OEM2              => Vk::Oem2            ,
    #[cfg(target_os = "windows")]
    Enigo::G                 => Vk::G               ,
    #[cfg(target_os = "windows")]
    Enigo::OEM3              => Vk::Oem3            ,
    #[cfg(target_os = "windows")]
    Enigo::H                 => Vk::H               ,
    Enigo::Help              => Vk::Help            ,
    Enigo::Home              => Vk::Home            ,
    #[cfg(target_os = "windows")]
    Enigo::I                 => Vk::I               ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Insert            => Vk::Insert          ,
    #[cfg(target_os = "windows")]
    Enigo::J                 => Vk::J               ,
    #[cfg(target_os = "windows")]
    Enigo::K                 => Vk::K               ,
    #[cfg(target_os = "windows")]
    Enigo::L                 => Vk::L               ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::LMenu             => Vk::LeftMenu        ,
    Enigo::Alt               => Vk::Alt             ,
    Enigo::Option            => Vk::Alt             ,
    #[cfg(target_os = "windows")]
    Enigo::OEM7              => Vk::Oem7            ,
    Enigo::LeftArrow         => Vk::LeftArrow       ,
    #[cfg(target_os = "windows")]
    Enigo::OEM4              => Vk::Oem4            ,
    Enigo::Control           => Vk::Control         ,
    Enigo::LControl          => Vk::LeftControl     ,
    #[cfg(target_os = "windows")]
    Enigo::LWin              => Vk::LeftWin         ,
    Enigo::Command           => Vk::LeftWin         ,
    Enigo::Meta              => Vk::LeftWin         ,
    Enigo::Super             => Vk::LeftWin         ,
    Enigo::Windows           => Vk::LeftWin         ,
    Enigo::LShift            => Vk::LeftShift       ,
    Enigo::Shift             => Vk::Shift           ,
    #[cfg(target_os = "windows")]
    Enigo::M                 => Vk::M               ,
    #[cfg(target_os = "windows")]
    Enigo::N                 => Vk::N               ,
    #[cfg(target_os = "windows")]
    Enigo::OEM102            => Vk::Oem102          ,
    #[cfg(target_os = "windows")]
    Enigo::O                 => Vk::O               ,
    #[cfg(target_os = "windows")]
    Enigo::P                 => Vk::P               ,
    Enigo::PageDown          => Vk::PageDown        ,
    Enigo::PageUp            => Vk::PageUp          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Pause             => Vk::Pause           ,
    #[cfg(target_os = "windows")]
    Enigo::OEMPeriod         => Vk::Period          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::PrintScr          => Vk::PrintScreen     ,
    #[cfg(target_os = "windows")]
    Enigo::Snapshot          => Vk::PrintScreen     ,
    #[cfg(target_os = "windows")]
    Enigo::Q                 => Vk::Q               ,
    #[cfg(target_os = "windows")]
    Enigo::R                 => Vk::R               ,
    Enigo::Return            => Vk::Enter           ,
    #[cfg(target_os = "windows")]
    Enigo::RMenu             => Vk::RightMenu       ,
    Enigo::RightArrow        => Vk::RightArrow      ,
    #[cfg(target_os = "windows")]
    Enigo::OEM6              => Vk::Oem6            ,
    Enigo::RControl          => Vk::RightControl    ,
    #[cfg(target_os = "windows")]
    Enigo::RWin              => Vk::RightWin        ,
    Enigo::RShift            => Vk::RightShift      ,
    #[cfg(target_os = "windows")]
    Enigo::S                 => Vk::S               ,
    #[cfg(target_os = "windows")]
    Enigo::Scroll            => Vk::Scroll          ,
    #[cfg(target_os = "linux")]
    Enigo::ScrollLock        => Vk::Scroll          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Select            => Vk::Select          ,
    #[cfg(target_os = "windows")]
    Enigo::OEM1              => Vk::Oem1            ,
    Enigo::Space             => Vk::Space           ,
    #[cfg(target_os = "windows")]
    Enigo::T                 => Vk::T               ,
    Enigo::Tab               => Vk::Tab             ,
    #[cfg(target_os = "windows")]
    Enigo::U                 => Vk::U               ,
    Enigo::UpArrow           => Vk::UpArrow         ,
    #[cfg(target_os = "windows")]
    Enigo::V                 => Vk::V               ,
    #[cfg(target_os = "windows")]
    Enigo::W                 => Vk::W               ,
    #[cfg(target_os = "windows")]
    Enigo::X                 => Vk::X               ,
    #[cfg(target_os = "windows")]
    Enigo::Y                 => Vk::Y               ,
    #[cfg(target_os = "windows")]
    Enigo::Z                 => Vk::Z               ,
    Enigo::Numpad0           => Vk::Numpad0         ,
    Enigo::Numpad1           => Vk::Numpad1         ,
    Enigo::Numpad2           => Vk::Numpad2         ,
    Enigo::Numpad3           => Vk::Numpad3         ,
    Enigo::Numpad4           => Vk::Numpad4         ,
    Enigo::Numpad5           => Vk::Numpad5         ,
    Enigo::Numpad6           => Vk::Numpad6         ,
    Enigo::Numpad7           => Vk::Numpad7         ,
    Enigo::Numpad8           => Vk::Numpad8         ,
    Enigo::Numpad9           => Vk::Numpad9         ,
    Enigo::Subtract          => Vk::Subtract        ,
    Enigo::Divide            => Vk::Divide          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Numlock           => Vk::Numlock         ,
    Enigo::Decimal           => Vk::Decimal         ,
    Enigo::Add               => Vk::Add             ,
    Enigo::Multiply          => Vk::Multiply        ,
    #[cfg(target_os = "windows")]
    Enigo::Accept            => Vk::Accept          ,
    #[cfg(target_os = "windows")]
    Enigo::Convert           => Vk::Convert         ,
    #[cfg(target_os = "windows")]
    Enigo::Final             => Vk::Final           ,
    #[cfg(target_os = "windows")]
    Enigo::IMEOff            => Vk::ImeOff          ,
    #[cfg(target_os = "windows")]
    Enigo::IMEOn             => Vk::ImeOn           ,
    #[cfg(target_os = "windows")]
    Enigo::Junja             => Vk::Junja           ,
    #[cfg(target_os = "windows")]
    Enigo::Kana              => Vk::Kana            ,
    #[cfg(target_os = "windows")]
    Enigo::Hangeul           => Vk::Kana            ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Hangul            => Vk::Kana            ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Kanji             => Vk::Kanji           ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Hanja             => Vk::Kanji           ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::ModeChange        => Vk::ModeChange      ,
    #[cfg(target_os = "windows")]
    Enigo::NonConvert        => Vk::NonConvert      ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Print             => Vk::Print           ,
    #[cfg(target_os = "windows")]
    Enigo::Separator         => Vk::Separator       ,
    #[cfg(target_os = "windows")]
    Enigo::LaunchApp1        => Vk::StartApp1       ,
    #[cfg(target_os = "windows")]
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
