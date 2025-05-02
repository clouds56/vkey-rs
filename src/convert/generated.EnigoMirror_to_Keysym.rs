// This file is auto-generated. Do not edit manually.


pub fn enigo_to_keysym(value: Enigo) -> Option<Keysym> {
  let result = match value {
    Enigo::VolumeMute     => Keysym::XF86_AudioMute       ,
    Enigo::MediaPlayPause => Keysym::XF86_AudioPlay       ,
    Enigo::MediaNextTrack => Keysym::XF86_AudioNext       ,
    Enigo::MediaPrevTrack => Keysym::XF86_AudioPrev       ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::MediaStop      => Keysym::XF86_AudioStop       ,
    Enigo::VolumeDown     => Keysym::XF86_AudioLowerVolume,
    Enigo::VolumeUp       => Keysym::XF86_AudioRaiseVolume,
    #[cfg(for_windows)]
    Enigo::Num0           => Keysym::_0                   ,
    #[cfg(for_windows)]
    Enigo::Num1           => Keysym::_1                   ,
    #[cfg(for_windows)]
    Enigo::Num2           => Keysym::_2                   ,
    #[cfg(for_windows)]
    Enigo::Num3           => Keysym::_3                   ,
    #[cfg(for_windows)]
    Enigo::Num4           => Keysym::_4                   ,
    #[cfg(for_windows)]
    Enigo::Num5           => Keysym::_5                   ,
    #[cfg(for_windows)]
    Enigo::Num6           => Keysym::_6                   ,
    #[cfg(for_windows)]
    Enigo::Num7           => Keysym::_7                   ,
    #[cfg(for_windows)]
    Enigo::Num8           => Keysym::_8                   ,
    #[cfg(for_windows)]
    Enigo::Num9           => Keysym::_9                   ,
    #[cfg(for_windows)]
    Enigo::A              => Keysym::A                    ,
    #[cfg(for_windows)]
    Enigo::B              => Keysym::B                    ,
    #[cfg(for_windows)]
    Enigo::C              => Keysym::C                    ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Cancel         => Keysym::Cancel               ,
    Enigo::CapsLock       => Keysym::Caps_Lock            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Clear          => Keysym::Clear                ,
    #[cfg(for_windows)]
    Enigo::D              => Keysym::D                    ,
    Enigo::Backspace      => Keysym::BackSpace            ,
    Enigo::Delete         => Keysym::Delete               ,
    Enigo::DownArrow      => Keysym::Down                 ,
    #[cfg(for_windows)]
    Enigo::E              => Keysym::E                    ,
    Enigo::End            => Keysym::End                  ,
    Enigo::Escape         => Keysym::Escape               ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Execute        => Keysym::Execute              ,
    #[cfg(for_windows)]
    Enigo::F              => Keysym::F                    ,
    Enigo::F1             => Keysym::F1                   ,
    Enigo::F10            => Keysym::F10                  ,
    Enigo::F11            => Keysym::F11                  ,
    Enigo::F12            => Keysym::F12                  ,
    Enigo::F13            => Keysym::F13                  ,
    Enigo::F14            => Keysym::F14                  ,
    Enigo::F15            => Keysym::F15                  ,
    Enigo::F16            => Keysym::F16                  ,
    Enigo::F17            => Keysym::F17                  ,
    Enigo::F18            => Keysym::F18                  ,
    Enigo::F19            => Keysym::F19                  ,
    Enigo::F2             => Keysym::F2                   ,
    Enigo::F20            => Keysym::F20                  ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F21            => Keysym::F21                  ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F22            => Keysym::F22                  ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F23            => Keysym::F23                  ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F24            => Keysym::F24                  ,
    Enigo::F3             => Keysym::F3                   ,
    Enigo::F4             => Keysym::F4                   ,
    Enigo::F5             => Keysym::F5                   ,
    Enigo::F6             => Keysym::F6                   ,
    Enigo::F7             => Keysym::F7                   ,
    Enigo::F8             => Keysym::F8                   ,
    Enigo::F9             => Keysym::F9                   ,
    #[cfg(for_windows)]
    Enigo::G              => Keysym::G                    ,
    #[cfg(for_windows)]
    Enigo::H              => Keysym::H                    ,
    Enigo::Help           => Keysym::Help                 ,
    Enigo::Home           => Keysym::Home                 ,
    #[cfg(for_windows)]
    Enigo::I              => Keysym::I                    ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Insert         => Keysym::Insert               ,
    #[cfg(for_windows)]
    Enigo::J              => Keysym::J                    ,
    #[cfg(for_windows)]
    Enigo::K              => Keysym::K                    ,
    #[cfg(for_windows)]
    Enigo::L              => Keysym::L                    ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::LMenu          => Keysym::Menu                 ,
    Enigo::Alt            => Keysym::Alt_L                ,
    Enigo::Option         => Keysym::Alt_L                ,
    Enigo::LeftArrow      => Keysym::Left                 ,
    Enigo::Control        => Keysym::Control_L            ,
    Enigo::LControl       => Keysym::Control_L            ,
    #[cfg(for_windows)]
    Enigo::LWin           => Keysym::Super_L              ,
    Enigo::Command        => Keysym::Super_L              ,
    Enigo::Meta           => Keysym::Super_L              ,
    Enigo::Super          => Keysym::Super_L              ,
    Enigo::Windows        => Keysym::Super_L              ,
    Enigo::LShift         => Keysym::Shift_L              ,
    Enigo::Shift          => Keysym::Shift_L              ,
    #[cfg(for_windows)]
    Enigo::M              => Keysym::M                    ,
    #[cfg(for_windows)]
    Enigo::N              => Keysym::N                    ,
    #[cfg(for_windows)]
    Enigo::O              => Keysym::O                    ,
    #[cfg(for_windows)]
    Enigo::P              => Keysym::P                    ,
    Enigo::PageDown       => Keysym::Page_Down            ,
    Enigo::PageUp         => Keysym::Page_Up              ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Pause          => Keysym::Pause                ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::PrintScr       => Keysym::Print                ,
    #[cfg(for_windows)]
    Enigo::Q              => Keysym::Q                    ,
    #[cfg(for_windows)]
    Enigo::R              => Keysym::R                    ,
    Enigo::Return         => Keysym::Return               ,
    #[cfg(for_windows)]
    Enigo::RMenu          => Keysym::Alt_R                ,
    #[cfg(for_macos)]
    Enigo::ROption        => Keysym::Alt_R                ,
    Enigo::RightArrow     => Keysym::Right                ,
    Enigo::RControl       => Keysym::Control_R            ,
    #[cfg(for_windows)]
    Enigo::RWin           => Keysym::Super_R              ,
    #[cfg(for_macos)]
    Enigo::RCommand       => Keysym::Super_R              ,
    Enigo::RShift         => Keysym::Shift_R              ,
    #[cfg(for_windows)]
    Enigo::S              => Keysym::S                    ,
    #[cfg(for_windows)]
    Enigo::Scroll         => Keysym::Scroll_Lock          ,
    #[cfg(for_linux)]
    Enigo::ScrollLock     => Keysym::Scroll_Lock          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Select         => Keysym::Select               ,
    Enigo::Space          => Keysym::space                ,
    #[cfg(for_windows)]
    Enigo::T              => Keysym::T                    ,
    Enigo::Tab            => Keysym::Tab                  ,
    #[cfg(for_windows)]
    Enigo::U              => Keysym::U                    ,
    Enigo::UpArrow        => Keysym::Up                   ,
    #[cfg(for_windows)]
    Enigo::V              => Keysym::V                    ,
    #[cfg(for_windows)]
    Enigo::W              => Keysym::W                    ,
    #[cfg(for_windows)]
    Enigo::X              => Keysym::X                    ,
    #[cfg(for_windows)]
    Enigo::Y              => Keysym::Y                    ,
    #[cfg(for_windows)]
    Enigo::Z              => Keysym::Z                    ,
    #[cfg(for_windows)]
    Enigo::Numpad0        => Keysym::KP_0                 ,
    #[cfg(for_windows)]
    Enigo::Numpad1        => Keysym::KP_1                 ,
    #[cfg(for_windows)]
    Enigo::Numpad2        => Keysym::KP_2                 ,
    #[cfg(for_windows)]
    Enigo::Numpad3        => Keysym::KP_3                 ,
    #[cfg(for_windows)]
    Enigo::Numpad4        => Keysym::KP_4                 ,
    #[cfg(for_windows)]
    Enigo::Numpad5        => Keysym::KP_5                 ,
    #[cfg(for_windows)]
    Enigo::Numpad6        => Keysym::KP_6                 ,
    #[cfg(for_windows)]
    Enigo::Numpad7        => Keysym::KP_7                 ,
    #[cfg(for_windows)]
    Enigo::Numpad8        => Keysym::KP_8                 ,
    #[cfg(for_windows)]
    Enigo::Numpad9        => Keysym::KP_9                 ,
    #[cfg(for_windows)]
    Enigo::Subtract       => Keysym::KP_Subtract          ,
    #[cfg(for_windows)]
    Enigo::Divide         => Keysym::KP_Divide            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Numlock        => Keysym::Num_Lock             ,
    #[cfg(for_windows)]
    Enigo::Decimal        => Keysym::KP_Decimal           ,
    #[cfg(for_windows)]
    Enigo::Add            => Keysym::KP_Add               ,
    #[cfg(for_windows)]
    Enigo::Multiply       => Keysym::KP_Multiply          ,
    #[cfg(for_linux)]
    Enigo::F25            => Keysym::F25                  ,
    #[cfg(for_linux)]
    Enigo::F26            => Keysym::F26                  ,
    #[cfg(for_linux)]
    Enigo::F27            => Keysym::F27                  ,
    #[cfg(for_linux)]
    Enigo::F28            => Keysym::F28                  ,
    #[cfg(for_linux)]
    Enigo::F29            => Keysym::F29                  ,
    #[cfg(for_linux)]
    Enigo::F30            => Keysym::F30                  ,
    #[cfg(for_linux)]
    Enigo::F31            => Keysym::F31                  ,
    #[cfg(for_linux)]
    Enigo::F32            => Keysym::F32                  ,
    #[cfg(for_linux)]
    Enigo::F33            => Keysym::F33                  ,
    #[cfg(for_linux)]
    Enigo::F34            => Keysym::F34                  ,
    #[cfg(for_linux)]
    Enigo::F35            => Keysym::F35                  ,
    #[cfg(for_linux)]
    Enigo::Begin          => Keysym::Begin                ,
    #[cfg(for_linux)]
    Enigo::Break          => Keysym::Break                ,
    #[cfg(for_linux)]
    Enigo::Find           => Keysym::Find                 ,
    #[cfg(for_linux)]
    Enigo::Linefeed       => Keysym::Linefeed             ,
    #[cfg(for_linux)]
    Enigo::MicMute        => Keysym::XF86_AudioMicMute    ,
    #[cfg(for_linux)]
    Enigo::Redo           => Keysym::Redo                 ,
    #[cfg(for_linux)]
    Enigo::ScriptSwitch   => Keysym::script_switch        ,
    #[cfg(for_linux)]
    Enigo::ShiftLock      => Keysym::Shift_Lock           ,
    #[cfg(for_linux)]
    Enigo::SysReq         => Keysym::Sys_Req              ,
    #[cfg(for_linux)]
    Enigo::Undo           => Keysym::Undo                 ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Hangul         => Keysym::Hangul               ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Kanji          => Keysym::Kanji                ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Hanja          => Keysym::Hangul_Hanja         ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::ModeChange     => Keysym::Mode_switch          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Print          => Keysym::Print                ,
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Enigo, Keysym> for crate::convert::Converter {
  fn convert(value: Enigo) -> Option<Keysym> {
    enigo_to_keysym(value)
  }
}
