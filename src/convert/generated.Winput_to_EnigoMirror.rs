// This file is auto-generated. Do not edit manually.


pub fn vk_to_enigo(value: Vk) -> Option<Enigo> {
  let result = match value {
    #[cfg(for_windows)]
    Vk::MouseLeft        => Enigo::LButton          ,
    #[cfg(for_windows)]
    Vk::MouseRight       => Enigo::RButton          ,
    #[cfg(for_windows)]
    Vk::MouseMiddle      => Enigo::MButton          ,
    #[cfg(for_windows)]
    Vk::MouseX1          => Enigo::XButton1         ,
    #[cfg(for_windows)]
    Vk::MouseX2          => Enigo::XButton2         ,
    #[cfg(for_windows)]
    Vk::BrowserBack      => Enigo::BrowserBack      ,
    #[cfg(for_windows)]
    Vk::BrowserFavorites => Enigo::BrowserFavorites ,
    #[cfg(for_windows)]
    Vk::BrowserForward   => Enigo::BrowserForward   ,
    #[cfg(for_windows)]
    Vk::BrowserHome      => Enigo::BrowserHome      ,
    #[cfg(for_windows)]
    Vk::BrowserRefresh   => Enigo::BrowserRefresh   ,
    #[cfg(for_windows)]
    Vk::BrowserSearch    => Enigo::BrowserSearch    ,
    #[cfg(for_windows)]
    Vk::BrowserStop      => Enigo::BrowserStop      ,
    #[cfg(for_windows)]
    Vk::StartMail        => Enigo::LaunchMail       ,
    #[cfg(for_windows)]
    Vk::SelectMedia      => Enigo::LaunchMediaSelect,
    Vk::VolumeMute       => Enigo::VolumeMute       ,
    Vk::MediaPlayPause   => Enigo::MediaPlayPause   ,
    Vk::NextTrack        => Enigo::MediaNextTrack   ,
    Vk::PrevTrack        => Enigo::MediaPrevTrack   ,
    #[cfg(any(for_windows, for_linux))]
    Vk::MediaStop        => Enigo::MediaStop        ,
    Vk::VolumeDown       => Enigo::VolumeDown       ,
    Vk::VolumeUp         => Enigo::VolumeUp         ,
    #[cfg(for_windows)]
    Vk::Sleep            => Enigo::Sleep            ,
    #[cfg(for_windows)]
    Vk::_0               => Enigo::Num0             ,
    #[cfg(for_windows)]
    Vk::_1               => Enigo::Num1             ,
    #[cfg(for_windows)]
    Vk::_2               => Enigo::Num2             ,
    #[cfg(for_windows)]
    Vk::_3               => Enigo::Num3             ,
    #[cfg(for_windows)]
    Vk::_4               => Enigo::Num4             ,
    #[cfg(for_windows)]
    Vk::_5               => Enigo::Num5             ,
    #[cfg(for_windows)]
    Vk::_6               => Enigo::Num6             ,
    #[cfg(for_windows)]
    Vk::_7               => Enigo::Num7             ,
    #[cfg(for_windows)]
    Vk::_8               => Enigo::Num8             ,
    #[cfg(for_windows)]
    Vk::_9               => Enigo::Num9             ,
    #[cfg(for_windows)]
    Vk::A                => Enigo::A                ,
    #[cfg(for_windows)]
    Vk::Apps             => Enigo::Apps             ,
    #[cfg(for_windows)]
    Vk::B                => Enigo::B                ,
    #[cfg(for_windows)]
    Vk::Oem5             => Enigo::OEM5             ,
    #[cfg(for_windows)]
    Vk::C                => Enigo::C                ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Cancel           => Enigo::Cancel           ,
    Vk::CapsLock         => Enigo::CapsLock         ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Clear            => Enigo::Clear            ,
    #[cfg(for_windows)]
    Vk::Comma            => Enigo::OEMComma         ,
    #[cfg(for_windows)]
    Vk::D                => Enigo::D                ,
    #[cfg(for_windows)]
    Vk::Minus            => Enigo::OEMMinus         ,
    Vk::Backspace        => Enigo::Backspace        ,
    Vk::Delete           => Enigo::Delete           ,
    Vk::DownArrow        => Enigo::DownArrow        ,
    #[cfg(for_windows)]
    Vk::E                => Enigo::E                ,
    Vk::End              => Enigo::End              ,
    #[cfg(for_windows)]
    Vk::Plus             => Enigo::OEMPlus          ,
    Vk::Escape           => Enigo::Escape           ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Execute          => Enigo::Execute          ,
    #[cfg(for_windows)]
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
    #[cfg(any(for_windows, for_linux))]
    Vk::F21              => Enigo::F21              ,
    #[cfg(any(for_windows, for_linux))]
    Vk::F22              => Enigo::F22              ,
    #[cfg(any(for_windows, for_linux))]
    Vk::F23              => Enigo::F23              ,
    #[cfg(any(for_windows, for_linux))]
    Vk::F24              => Enigo::F24              ,
    Vk::F3               => Enigo::F3               ,
    Vk::F4               => Enigo::F4               ,
    Vk::F5               => Enigo::F5               ,
    Vk::F6               => Enigo::F6               ,
    Vk::F7               => Enigo::F7               ,
    Vk::F8               => Enigo::F8               ,
    Vk::F9               => Enigo::F9               ,
    #[cfg(for_windows)]
    Vk::Oem2             => Enigo::OEM2             ,
    #[cfg(for_windows)]
    Vk::G                => Enigo::G                ,
    #[cfg(for_windows)]
    Vk::Oem3             => Enigo::OEM3             ,
    #[cfg(for_windows)]
    Vk::H                => Enigo::H                ,
    Vk::Help             => Enigo::Help             ,
    Vk::Home             => Enigo::Home             ,
    #[cfg(for_windows)]
    Vk::I                => Enigo::I                ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Insert           => Enigo::Insert           ,
    #[cfg(for_windows)]
    Vk::J                => Enigo::J                ,
    #[cfg(for_windows)]
    Vk::K                => Enigo::K                ,
    #[cfg(for_windows)]
    Vk::L                => Enigo::L                ,
    #[cfg(any(for_windows, for_linux))]
    Vk::LeftMenu         => Enigo::LMenu            ,
    Vk::Alt              => Enigo::Alt              ,
    #[cfg(for_windows)]
    Vk::Oem7             => Enigo::OEM7             ,
    Vk::LeftArrow        => Enigo::LeftArrow        ,
    #[cfg(for_windows)]
    Vk::Oem4             => Enigo::OEM4             ,
    Vk::Control          => Enigo::Control          ,
    Vk::LeftControl      => Enigo::LControl         ,
    #[cfg(for_windows)]
    Vk::LeftWin          => Enigo::LWin             ,
    Vk::LeftShift        => Enigo::LShift           ,
    Vk::Shift            => Enigo::Shift            ,
    #[cfg(for_windows)]
    Vk::M                => Enigo::M                ,
    #[cfg(for_windows)]
    Vk::N                => Enigo::N                ,
    #[cfg(for_windows)]
    Vk::Oem102           => Enigo::OEM102           ,
    #[cfg(for_windows)]
    Vk::O                => Enigo::O                ,
    #[cfg(for_windows)]
    Vk::P                => Enigo::P                ,
    Vk::PageDown         => Enigo::PageDown         ,
    Vk::PageUp           => Enigo::PageUp           ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Pause            => Enigo::Pause            ,
    #[cfg(for_windows)]
    Vk::Period           => Enigo::OEMPeriod        ,
    #[cfg(any(for_windows, for_linux))]
    Vk::PrintScreen      => Enigo::PrintScr         ,
    #[cfg(for_windows)]
    Vk::Q                => Enigo::Q                ,
    #[cfg(for_windows)]
    Vk::R                => Enigo::R                ,
    Vk::Enter            => Enigo::Return           ,
    #[cfg(for_windows)]
    Vk::RightMenu        => Enigo::RMenu            ,
    Vk::RightArrow       => Enigo::RightArrow       ,
    #[cfg(for_windows)]
    Vk::Oem6             => Enigo::OEM6             ,
    Vk::RightControl     => Enigo::RControl         ,
    #[cfg(for_windows)]
    Vk::RightWin         => Enigo::RWin             ,
    Vk::RightShift       => Enigo::RShift           ,
    #[cfg(for_windows)]
    Vk::S                => Enigo::S                ,
    #[cfg(for_windows)]
    Vk::Scroll           => Enigo::Scroll           ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Select           => Enigo::Select           ,
    #[cfg(for_windows)]
    Vk::Oem1             => Enigo::OEM1             ,
    Vk::Space            => Enigo::Space            ,
    #[cfg(for_windows)]
    Vk::T                => Enigo::T                ,
    Vk::Tab              => Enigo::Tab              ,
    #[cfg(for_windows)]
    Vk::U                => Enigo::U                ,
    Vk::UpArrow          => Enigo::UpArrow          ,
    #[cfg(for_windows)]
    Vk::V                => Enigo::V                ,
    #[cfg(for_windows)]
    Vk::W                => Enigo::W                ,
    #[cfg(for_windows)]
    Vk::X                => Enigo::X                ,
    #[cfg(for_windows)]
    Vk::Y                => Enigo::Y                ,
    #[cfg(for_windows)]
    Vk::Z                => Enigo::Z                ,
    #[cfg(for_windows)]
    Vk::Numpad0          => Enigo::Numpad0          ,
    #[cfg(for_windows)]
    Vk::Numpad1          => Enigo::Numpad1          ,
    #[cfg(for_windows)]
    Vk::Numpad2          => Enigo::Numpad2          ,
    #[cfg(for_windows)]
    Vk::Numpad3          => Enigo::Numpad3          ,
    #[cfg(for_windows)]
    Vk::Numpad4          => Enigo::Numpad4          ,
    #[cfg(for_windows)]
    Vk::Numpad5          => Enigo::Numpad5          ,
    #[cfg(for_windows)]
    Vk::Numpad6          => Enigo::Numpad6          ,
    #[cfg(for_windows)]
    Vk::Numpad7          => Enigo::Numpad7          ,
    #[cfg(for_windows)]
    Vk::Numpad8          => Enigo::Numpad8          ,
    #[cfg(for_windows)]
    Vk::Numpad9          => Enigo::Numpad9          ,
    #[cfg(for_windows)]
    Vk::Subtract         => Enigo::Subtract         ,
    #[cfg(for_windows)]
    Vk::Divide           => Enigo::Divide           ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Numlock          => Enigo::Numlock          ,
    #[cfg(for_windows)]
    Vk::Decimal          => Enigo::Decimal          ,
    #[cfg(for_windows)]
    Vk::Add              => Enigo::Add              ,
    #[cfg(for_windows)]
    Vk::Multiply         => Enigo::Multiply         ,
    #[cfg(for_windows)]
    Vk::Accept           => Enigo::Accept           ,
    #[cfg(for_windows)]
    Vk::Convert          => Enigo::Convert          ,
    #[cfg(for_windows)]
    Vk::Final            => Enigo::Final            ,
    #[cfg(for_windows)]
    Vk::ImeOff           => Enigo::IMEOff           ,
    #[cfg(for_windows)]
    Vk::ImeOn            => Enigo::IMEOn            ,
    #[cfg(for_windows)]
    Vk::Junja            => Enigo::Junja            ,
    #[cfg(for_windows)]
    Vk::Kana             => Enigo::Kana             ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Kanji            => Enigo::Kanji            ,
    #[cfg(any(for_windows, for_linux))]
    Vk::ModeChange       => Enigo::ModeChange       ,
    #[cfg(for_windows)]
    Vk::NonConvert       => Enigo::NonConvert       ,
    #[cfg(any(for_windows, for_linux))]
    Vk::Print            => Enigo::Print            ,
    #[cfg(for_windows)]
    Vk::Separator        => Enigo::Separator        ,
    #[cfg(for_windows)]
    Vk::StartApp1        => Enigo::LaunchApp1       ,
    #[cfg(for_windows)]
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
