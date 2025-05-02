// This file is auto-generated. Do not edit manually.


pub fn enigo_to_keysym(value: Enigo) -> Option<Keysym> {
  let result = match value {
    Enigo::VolumeMute     => Keysym::XF86_AudioMute       ,
    Enigo::MediaPlayPause => Keysym::XF86_AudioPlay       ,
    Enigo::MediaNextTrack => Keysym::XF86_AudioNext       ,
    Enigo::MediaPrevTrack => Keysym::XF86_AudioPrev       ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::MediaStop      => Keysym::XF86_AudioStop       ,
    Enigo::VolumeDown     => Keysym::XF86_AudioLowerVolume,
    Enigo::VolumeUp       => Keysym::XF86_AudioRaiseVolume,
    #[cfg(target_os = "windows")]
    Enigo::Num0           => Keysym::_0                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num1           => Keysym::_1                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num2           => Keysym::_2                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num3           => Keysym::_3                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num4           => Keysym::_4                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num5           => Keysym::_5                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num6           => Keysym::_6                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num7           => Keysym::_7                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num8           => Keysym::_8                   ,
    #[cfg(target_os = "windows")]
    Enigo::Num9           => Keysym::_9                   ,
    #[cfg(target_os = "windows")]
    Enigo::A              => Keysym::A                    ,
    #[cfg(target_os = "windows")]
    Enigo::B              => Keysym::B                    ,
    #[cfg(target_os = "windows")]
    Enigo::C              => Keysym::C                    ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Cancel         => Keysym::Cancel               ,
    Enigo::CapsLock       => Keysym::Caps_Lock            ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Clear          => Keysym::Clear                ,
    #[cfg(target_os = "windows")]
    Enigo::D              => Keysym::D                    ,
    Enigo::Backspace      => Keysym::BackSpace            ,
    Enigo::Delete         => Keysym::Delete               ,
    Enigo::DownArrow      => Keysym::Down                 ,
    #[cfg(target_os = "windows")]
    Enigo::E              => Keysym::E                    ,
    Enigo::End            => Keysym::End                  ,
    Enigo::Escape         => Keysym::Escape               ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Execute        => Keysym::Execute              ,
    #[cfg(target_os = "windows")]
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
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F21            => Keysym::F21                  ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F22            => Keysym::F22                  ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F23            => Keysym::F23                  ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::F24            => Keysym::F24                  ,
    Enigo::F3             => Keysym::F3                   ,
    Enigo::F4             => Keysym::F4                   ,
    Enigo::F5             => Keysym::F5                   ,
    Enigo::F6             => Keysym::F6                   ,
    Enigo::F7             => Keysym::F7                   ,
    Enigo::F8             => Keysym::F8                   ,
    Enigo::F9             => Keysym::F9                   ,
    #[cfg(target_os = "windows")]
    Enigo::G              => Keysym::G                    ,
    #[cfg(target_os = "windows")]
    Enigo::H              => Keysym::H                    ,
    Enigo::Help           => Keysym::Help                 ,
    Enigo::Home           => Keysym::Home                 ,
    #[cfg(target_os = "windows")]
    Enigo::I              => Keysym::I                    ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Insert         => Keysym::Insert               ,
    #[cfg(target_os = "windows")]
    Enigo::J              => Keysym::J                    ,
    #[cfg(target_os = "windows")]
    Enigo::K              => Keysym::K                    ,
    #[cfg(target_os = "windows")]
    Enigo::L              => Keysym::L                    ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::LMenu          => Keysym::Menu                 ,
    Enigo::Alt            => Keysym::Alt_L                ,
    Enigo::Option         => Keysym::Alt_L                ,
    Enigo::LeftArrow      => Keysym::Left                 ,
    Enigo::Control        => Keysym::Control_L            ,
    Enigo::LControl       => Keysym::Control_L            ,
    #[cfg(target_os = "windows")]
    Enigo::LWin           => Keysym::Super_L              ,
    Enigo::Command        => Keysym::Super_L              ,
    Enigo::Meta           => Keysym::Super_L              ,
    Enigo::Super          => Keysym::Super_L              ,
    Enigo::Windows        => Keysym::Super_L              ,
    Enigo::LShift         => Keysym::Shift_L              ,
    Enigo::Shift          => Keysym::Shift_L              ,
    #[cfg(target_os = "windows")]
    Enigo::M              => Keysym::M                    ,
    #[cfg(target_os = "windows")]
    Enigo::N              => Keysym::N                    ,
    #[cfg(target_os = "windows")]
    Enigo::O              => Keysym::O                    ,
    #[cfg(target_os = "windows")]
    Enigo::P              => Keysym::P                    ,
    Enigo::PageDown       => Keysym::Page_Down            ,
    Enigo::PageUp         => Keysym::Page_Up              ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Pause          => Keysym::Pause                ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::PrintScr       => Keysym::Print                ,
    #[cfg(target_os = "windows")]
    Enigo::Q              => Keysym::Q                    ,
    #[cfg(target_os = "windows")]
    Enigo::R              => Keysym::R                    ,
    Enigo::Return         => Keysym::Return               ,
    #[cfg(target_os = "windows")]
    Enigo::RMenu          => Keysym::Alt_R                ,
    #[cfg(target_os = "macos")]
    Enigo::ROption        => Keysym::Alt_R                ,
    Enigo::RightArrow     => Keysym::Right                ,
    Enigo::RControl       => Keysym::Control_R            ,
    #[cfg(target_os = "windows")]
    Enigo::RWin           => Keysym::Super_R              ,
    #[cfg(target_os = "macos")]
    Enigo::RCommand       => Keysym::Super_R              ,
    Enigo::RShift         => Keysym::Shift_R              ,
    #[cfg(target_os = "windows")]
    Enigo::S              => Keysym::S                    ,
    #[cfg(target_os = "windows")]
    Enigo::Scroll         => Keysym::Scroll_Lock          ,
    #[cfg(target_os = "linux")]
    Enigo::ScrollLock     => Keysym::Scroll_Lock          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Select         => Keysym::Select               ,
    Enigo::Space          => Keysym::space                ,
    #[cfg(target_os = "windows")]
    Enigo::T              => Keysym::T                    ,
    Enigo::Tab            => Keysym::Tab                  ,
    #[cfg(target_os = "windows")]
    Enigo::U              => Keysym::U                    ,
    Enigo::UpArrow        => Keysym::Up                   ,
    #[cfg(target_os = "windows")]
    Enigo::V              => Keysym::V                    ,
    #[cfg(target_os = "windows")]
    Enigo::W              => Keysym::W                    ,
    #[cfg(target_os = "windows")]
    Enigo::X              => Keysym::X                    ,
    #[cfg(target_os = "windows")]
    Enigo::Y              => Keysym::Y                    ,
    #[cfg(target_os = "windows")]
    Enigo::Z              => Keysym::Z                    ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad0        => Keysym::KP_0                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad1        => Keysym::KP_1                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad2        => Keysym::KP_2                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad3        => Keysym::KP_3                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad4        => Keysym::KP_4                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad5        => Keysym::KP_5                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad6        => Keysym::KP_6                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad7        => Keysym::KP_7                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad8        => Keysym::KP_8                 ,
    #[cfg(target_os = "windows")]
    Enigo::Numpad9        => Keysym::KP_9                 ,
    #[cfg(target_os = "windows")]
    Enigo::Subtract       => Keysym::KP_Subtract          ,
    #[cfg(target_os = "windows")]
    Enigo::Divide         => Keysym::KP_Divide            ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Numlock        => Keysym::Num_Lock             ,
    #[cfg(target_os = "windows")]
    Enigo::Decimal        => Keysym::KP_Decimal           ,
    #[cfg(target_os = "windows")]
    Enigo::Add            => Keysym::KP_Add               ,
    #[cfg(target_os = "windows")]
    Enigo::Multiply       => Keysym::KP_Multiply          ,
    #[cfg(target_os = "linux")]
    Enigo::F25            => Keysym::F25                  ,
    #[cfg(target_os = "linux")]
    Enigo::F26            => Keysym::F26                  ,
    #[cfg(target_os = "linux")]
    Enigo::F27            => Keysym::F27                  ,
    #[cfg(target_os = "linux")]
    Enigo::F28            => Keysym::F28                  ,
    #[cfg(target_os = "linux")]
    Enigo::F29            => Keysym::F29                  ,
    #[cfg(target_os = "linux")]
    Enigo::F30            => Keysym::F30                  ,
    #[cfg(target_os = "linux")]
    Enigo::F31            => Keysym::F31                  ,
    #[cfg(target_os = "linux")]
    Enigo::F32            => Keysym::F32                  ,
    #[cfg(target_os = "linux")]
    Enigo::F33            => Keysym::F33                  ,
    #[cfg(target_os = "linux")]
    Enigo::F34            => Keysym::F34                  ,
    #[cfg(target_os = "linux")]
    Enigo::F35            => Keysym::F35                  ,
    #[cfg(target_os = "linux")]
    Enigo::Begin          => Keysym::Begin                ,
    #[cfg(target_os = "linux")]
    Enigo::Break          => Keysym::Break                ,
    #[cfg(target_os = "linux")]
    Enigo::Find           => Keysym::Find                 ,
    #[cfg(target_os = "linux")]
    Enigo::Linefeed       => Keysym::Linefeed             ,
    #[cfg(target_os = "linux")]
    Enigo::MicMute        => Keysym::XF86_AudioMicMute    ,
    #[cfg(target_os = "linux")]
    Enigo::Redo           => Keysym::Redo                 ,
    #[cfg(target_os = "linux")]
    Enigo::ScriptSwitch   => Keysym::script_switch        ,
    #[cfg(target_os = "linux")]
    Enigo::ShiftLock      => Keysym::Shift_Lock           ,
    #[cfg(target_os = "linux")]
    Enigo::SysReq         => Keysym::Sys_Req              ,
    #[cfg(target_os = "linux")]
    Enigo::Undo           => Keysym::Undo                 ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Hangul         => Keysym::Hangul               ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Kanji          => Keysym::Kanji                ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::Hanja          => Keysym::Hangul_Hanja         ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Enigo::ModeChange     => Keysym::Mode_switch          ,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
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
