// This file is auto-generated. Do not edit manually.


pub fn vk_to_enigo(value: Vk) -> Option<Enigo> {
  let result = match value {
    #[cfg(target_os = "windows")]
    Vk::MouseLeft        => Enigo::LButton          ,
    #[cfg(target_os = "windows")]
    Vk::MouseRight       => Enigo::RButton          ,
    #[cfg(target_os = "windows")]
    Vk::MouseMiddle      => Enigo::MButton          ,
    #[cfg(target_os = "windows")]
    Vk::MouseX1          => Enigo::XButton1         ,
    #[cfg(target_os = "windows")]
    Vk::MouseX2          => Enigo::XButton2         ,
    #[cfg(target_os = "windows")]
    Vk::BrowserBack      => Enigo::BrowserBack      ,
    #[cfg(target_os = "windows")]
    Vk::BrowserFavorites => Enigo::BrowserFavorites ,
    #[cfg(target_os = "windows")]
    Vk::BrowserForward   => Enigo::BrowserForward   ,
    #[cfg(target_os = "windows")]
    Vk::BrowserHome      => Enigo::BrowserHome      ,
    #[cfg(target_os = "windows")]
    Vk::BrowserRefresh   => Enigo::BrowserRefresh   ,
    #[cfg(target_os = "windows")]
    Vk::BrowserSearch    => Enigo::BrowserSearch    ,
    #[cfg(target_os = "windows")]
    Vk::BrowserStop      => Enigo::BrowserStop      ,
    #[cfg(target_os = "windows")]
    Vk::StartMail        => Enigo::LaunchMail       ,
    #[cfg(target_os = "windows")]
    Vk::SelectMedia      => Enigo::LaunchMediaSelect,
    Vk::VolumeMute       => Enigo::VolumeMute       ,
    Vk::MediaPlayPause   => Enigo::MediaPlayPause   ,
    Vk::NextTrack        => Enigo::MediaNextTrack   ,
    Vk::PrevTrack        => Enigo::MediaPrevTrack   ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::MediaStop        => Enigo::MediaStop        ,
    Vk::VolumeDown       => Enigo::VolumeDown       ,
    Vk::VolumeUp         => Enigo::VolumeUp         ,
    #[cfg(target_os = "windows")]
    Vk::Sleep            => Enigo::Sleep            ,
    #[cfg(target_os = "windows")]
    Vk::_0               => Enigo::Num0             ,
    #[cfg(target_os = "windows")]
    Vk::_1               => Enigo::Num1             ,
    #[cfg(target_os = "windows")]
    Vk::_2               => Enigo::Num2             ,
    #[cfg(target_os = "windows")]
    Vk::_3               => Enigo::Num3             ,
    #[cfg(target_os = "windows")]
    Vk::_4               => Enigo::Num4             ,
    #[cfg(target_os = "windows")]
    Vk::_5               => Enigo::Num5             ,
    #[cfg(target_os = "windows")]
    Vk::_6               => Enigo::Num6             ,
    #[cfg(target_os = "windows")]
    Vk::_7               => Enigo::Num7             ,
    #[cfg(target_os = "windows")]
    Vk::_8               => Enigo::Num8             ,
    #[cfg(target_os = "windows")]
    Vk::_9               => Enigo::Num9             ,
    #[cfg(target_os = "windows")]
    Vk::A                => Enigo::A                ,
    #[cfg(target_os = "windows")]
    Vk::Apps             => Enigo::Apps             ,
    #[cfg(target_os = "windows")]
    Vk::B                => Enigo::B                ,
    #[cfg(target_os = "windows")]
    Vk::Oem5             => Enigo::OEM5             ,
    #[cfg(target_os = "windows")]
    Vk::C                => Enigo::C                ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Cancel           => Enigo::Cancel           ,
    Vk::CapsLock         => Enigo::CapsLock         ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Clear            => Enigo::Clear            ,
    #[cfg(target_os = "windows")]
    Vk::Comma            => Enigo::OEMComma         ,
    #[cfg(target_os = "windows")]
    Vk::D                => Enigo::D                ,
    #[cfg(target_os = "windows")]
    Vk::Minus            => Enigo::OEMMinus         ,
    Vk::Backspace        => Enigo::Backspace        ,
    Vk::Delete           => Enigo::Delete           ,
    Vk::DownArrow        => Enigo::DownArrow        ,
    #[cfg(target_os = "windows")]
    Vk::E                => Enigo::E                ,
    Vk::End              => Enigo::End              ,
    #[cfg(target_os = "windows")]
    Vk::Plus             => Enigo::OEMPlus          ,
    Vk::Escape           => Enigo::Escape           ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Execute          => Enigo::Execute          ,
    #[cfg(target_os = "windows")]
    Vk::F                => Enigo::F                ,
    Vk::F1               => Enigo::F1               ,
    Vk::F10              => Enigo::F10              ,
    Vk::F11              => Enigo::F11              ,
    Vk::F12              => Enigo::F12              ,
    Vk::F13              => Enigo::F13              ,
    Vk::F14              => Enigo::F14              ,
    Vk::F15              => Enigo::F15              ,
    Vk::F16              => Enigo::F16              ,
    Vk::F17              => Enigo::F17              ,
    Vk::F18              => Enigo::F18              ,
    Vk::F19              => Enigo::F19              ,
    Vk::F2               => Enigo::F2               ,
    Vk::F20              => Enigo::F20              ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::F21              => Enigo::F21              ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::F22              => Enigo::F22              ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::F23              => Enigo::F23              ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::F24              => Enigo::F24              ,
    Vk::F3               => Enigo::F3               ,
    Vk::F4               => Enigo::F4               ,
    Vk::F5               => Enigo::F5               ,
    Vk::F6               => Enigo::F6               ,
    Vk::F7               => Enigo::F7               ,
    Vk::F8               => Enigo::F8               ,
    Vk::F9               => Enigo::F9               ,
    #[cfg(target_os = "windows")]
    Vk::Oem2             => Enigo::OEM2             ,
    #[cfg(target_os = "windows")]
    Vk::G                => Enigo::G                ,
    #[cfg(target_os = "windows")]
    Vk::Oem3             => Enigo::OEM3             ,
    #[cfg(target_os = "windows")]
    Vk::H                => Enigo::H                ,
    Vk::Help             => Enigo::Help             ,
    Vk::Home             => Enigo::Home             ,
    #[cfg(target_os = "windows")]
    Vk::I                => Enigo::I                ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Insert           => Enigo::Insert           ,
    #[cfg(target_os = "windows")]
    Vk::J                => Enigo::J                ,
    #[cfg(target_os = "windows")]
    Vk::K                => Enigo::K                ,
    #[cfg(target_os = "windows")]
    Vk::L                => Enigo::L                ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::LeftMenu         => Enigo::LMenu            ,
    Vk::Alt              => Enigo::Alt              ,
    #[cfg(target_os = "windows")]
    Vk::Oem7             => Enigo::OEM7             ,
    Vk::LeftArrow        => Enigo::LeftArrow        ,
    #[cfg(target_os = "windows")]
    Vk::Oem4             => Enigo::OEM4             ,
    Vk::Control          => Enigo::Control          ,
    Vk::LeftControl      => Enigo::LControl         ,
    #[cfg(target_os = "windows")]
    Vk::LeftWin          => Enigo::LWin             ,
    Vk::LeftShift        => Enigo::LShift           ,
    Vk::Shift            => Enigo::Shift            ,
    #[cfg(target_os = "windows")]
    Vk::M                => Enigo::M                ,
    #[cfg(target_os = "windows")]
    Vk::N                => Enigo::N                ,
    #[cfg(target_os = "windows")]
    Vk::Oem102           => Enigo::OEM102           ,
    #[cfg(target_os = "windows")]
    Vk::O                => Enigo::O                ,
    #[cfg(target_os = "windows")]
    Vk::P                => Enigo::P                ,
    Vk::PageDown         => Enigo::PageDown         ,
    Vk::PageUp           => Enigo::PageUp           ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Pause            => Enigo::Pause            ,
    #[cfg(target_os = "windows")]
    Vk::Period           => Enigo::OEMPeriod        ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::PrintScreen      => Enigo::PrintScr         ,
    #[cfg(target_os = "windows")]
    Vk::Q                => Enigo::Q                ,
    #[cfg(target_os = "windows")]
    Vk::R                => Enigo::R                ,
    Vk::Enter            => Enigo::Return           ,
    #[cfg(target_os = "windows")]
    Vk::RightMenu        => Enigo::RMenu            ,
    Vk::RightArrow       => Enigo::RightArrow       ,
    #[cfg(target_os = "windows")]
    Vk::Oem6             => Enigo::OEM6             ,
    Vk::RightControl     => Enigo::RControl         ,
    #[cfg(target_os = "windows")]
    Vk::RightWin         => Enigo::RWin             ,
    Vk::RightShift       => Enigo::RShift           ,
    #[cfg(target_os = "windows")]
    Vk::S                => Enigo::S                ,
    #[cfg(target_os = "windows")]
    Vk::Scroll           => Enigo::Scroll           ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Select           => Enigo::Select           ,
    #[cfg(target_os = "windows")]
    Vk::Oem1             => Enigo::OEM1             ,
    Vk::Space            => Enigo::Space            ,
    #[cfg(target_os = "windows")]
    Vk::T                => Enigo::T                ,
    Vk::Tab              => Enigo::Tab              ,
    #[cfg(target_os = "windows")]
    Vk::U                => Enigo::U                ,
    Vk::UpArrow          => Enigo::UpArrow          ,
    #[cfg(target_os = "windows")]
    Vk::V                => Enigo::V                ,
    #[cfg(target_os = "windows")]
    Vk::W                => Enigo::W                ,
    #[cfg(target_os = "windows")]
    Vk::X                => Enigo::X                ,
    #[cfg(target_os = "windows")]
    Vk::Y                => Enigo::Y                ,
    #[cfg(target_os = "windows")]
    Vk::Z                => Enigo::Z                ,
    #[cfg(target_os = "windows")]
    Vk::Numpad0          => Enigo::Numpad0          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad1          => Enigo::Numpad1          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad2          => Enigo::Numpad2          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad3          => Enigo::Numpad3          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad4          => Enigo::Numpad4          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad5          => Enigo::Numpad5          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad6          => Enigo::Numpad6          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad7          => Enigo::Numpad7          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad8          => Enigo::Numpad8          ,
    #[cfg(target_os = "windows")]
    Vk::Numpad9          => Enigo::Numpad9          ,
    #[cfg(target_os = "windows")]
    Vk::Subtract         => Enigo::Subtract         ,
    #[cfg(target_os = "windows")]
    Vk::Divide           => Enigo::Divide           ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Numlock          => Enigo::Numlock          ,
    #[cfg(target_os = "windows")]
    Vk::Decimal          => Enigo::Decimal          ,
    #[cfg(target_os = "windows")]
    Vk::Add              => Enigo::Add              ,
    #[cfg(target_os = "windows")]
    Vk::Multiply         => Enigo::Multiply         ,
    #[cfg(target_os = "windows")]
    Vk::Accept           => Enigo::Accept           ,
    #[cfg(target_os = "windows")]
    Vk::Convert          => Enigo::Convert          ,
    #[cfg(target_os = "windows")]
    Vk::Final            => Enigo::Final            ,
    #[cfg(target_os = "windows")]
    Vk::ImeOff           => Enigo::IMEOff           ,
    #[cfg(target_os = "windows")]
    Vk::ImeOn            => Enigo::IMEOn            ,
    #[cfg(target_os = "windows")]
    Vk::Junja            => Enigo::Junja            ,
    #[cfg(target_os = "windows")]
    Vk::Kana             => Enigo::Kana             ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Kanji            => Enigo::Kanji            ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::ModeChange       => Enigo::ModeChange       ,
    #[cfg(target_os = "windows")]
    Vk::NonConvert       => Enigo::NonConvert       ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vk::Print            => Enigo::Print            ,
    #[cfg(target_os = "windows")]
    Vk::Separator        => Enigo::Separator        ,
    #[cfg(target_os = "windows")]
    Vk::StartApp1        => Enigo::LaunchApp1       ,
    #[cfg(target_os = "windows")]
    Vk::StartApp2        => Enigo::LaunchApp2       ,
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Vk, Enigo> for crate::convert::Converter {
  fn convert(value: Vk) -> Option<Enigo> {
    vk_to_enigo(value)
  }
}
