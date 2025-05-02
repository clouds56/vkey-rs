// This file is auto-generated. Do not edit manually.


pub fn enigo_to_vk(value: Enigo) -> Option<Vk> {
  let result = match value {
    #[cfg(for_windows)]
    Enigo::LButton                      => keys::VK_LBUTTON                        ,
    #[cfg(for_windows)]
    Enigo::RButton                      => keys::VK_RBUTTON                        ,
    #[cfg(for_windows)]
    Enigo::MButton                      => keys::VK_MBUTTON                        ,
    #[cfg(for_windows)]
    Enigo::XButton1                     => keys::VK_XBUTTON1                       ,
    #[cfg(for_windows)]
    Enigo::XButton2                     => keys::VK_XBUTTON2                       ,
    #[cfg(for_windows)]
    Enigo::BrowserBack                  => keys::VK_BROWSER_BACK                   ,
    #[cfg(for_windows)]
    Enigo::BrowserFavorites             => keys::VK_BROWSER_FAVORITES              ,
    #[cfg(for_windows)]
    Enigo::BrowserForward               => keys::VK_BROWSER_FORWARD                ,
    #[cfg(for_windows)]
    Enigo::BrowserHome                  => keys::VK_BROWSER_HOME                   ,
    #[cfg(for_windows)]
    Enigo::BrowserRefresh               => keys::VK_BROWSER_REFRESH                ,
    #[cfg(for_windows)]
    Enigo::BrowserSearch                => keys::VK_BROWSER_SEARCH                 ,
    #[cfg(for_windows)]
    Enigo::BrowserStop                  => keys::VK_BROWSER_STOP                   ,
    #[cfg(for_windows)]
    Enigo::LaunchMail                   => keys::VK_LAUNCH_MAIL                    ,
    #[cfg(for_windows)]
    Enigo::LaunchMediaSelect            => keys::VK_LAUNCH_MEDIA_SELECT            ,
    Enigo::VolumeMute                   => keys::VK_VOLUME_MUTE                    ,
    Enigo::MediaPlayPause               => keys::VK_MEDIA_PLAY_PAUSE               ,
    Enigo::MediaNextTrack               => keys::VK_MEDIA_NEXT_TRACK               ,
    Enigo::MediaPrevTrack               => keys::VK_MEDIA_PREV_TRACK               ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::MediaStop                    => keys::VK_MEDIA_STOP                     ,
    Enigo::VolumeDown                   => keys::VK_VOLUME_DOWN                    ,
    Enigo::VolumeUp                     => keys::VK_VOLUME_UP                      ,
    #[cfg(for_windows)]
    Enigo::Sleep                        => keys::VK_SLEEP                          ,
    #[cfg(for_windows)]
    Enigo::Num0                         => keys::VK_0                              ,
    #[cfg(for_windows)]
    Enigo::Num1                         => keys::VK_1                              ,
    #[cfg(for_windows)]
    Enigo::Num2                         => keys::VK_2                              ,
    #[cfg(for_windows)]
    Enigo::Num3                         => keys::VK_3                              ,
    #[cfg(for_windows)]
    Enigo::Num4                         => keys::VK_4                              ,
    #[cfg(for_windows)]
    Enigo::Num5                         => keys::VK_5                              ,
    #[cfg(for_windows)]
    Enigo::Num6                         => keys::VK_6                              ,
    #[cfg(for_windows)]
    Enigo::Num7                         => keys::VK_7                              ,
    #[cfg(for_windows)]
    Enigo::Num8                         => keys::VK_8                              ,
    #[cfg(for_windows)]
    Enigo::Num9                         => keys::VK_9                              ,
    #[cfg(for_windows)]
    Enigo::A                            => keys::VK_A                              ,
    #[cfg(for_windows)]
    Enigo::Apps                         => keys::VK_APPS                           ,
    #[cfg(for_windows)]
    Enigo::B                            => keys::VK_B                              ,
    #[cfg(for_windows)]
    Enigo::OEM5                         => keys::VK_OEM_5                          ,
    #[cfg(for_windows)]
    Enigo::C                            => keys::VK_C                              ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Cancel                       => keys::VK_CANCEL                         ,
    Enigo::CapsLock                     => keys::VK_CAPITAL                        ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Clear                        => keys::VK_CLEAR                          ,
    #[cfg(for_windows)]
    Enigo::OEMComma                     => keys::VK_OEM_COMMA                      ,
    #[cfg(for_windows)]
    Enigo::D                            => keys::VK_D                              ,
    #[cfg(for_windows)]
    Enigo::OEMMinus                     => keys::VK_OEM_MINUS                      ,
    Enigo::Backspace                    => keys::VK_BACK                           ,
    Enigo::Delete                       => keys::VK_DELETE                         ,
    Enigo::DownArrow                    => keys::VK_DOWN                           ,
    #[cfg(for_windows)]
    Enigo::E                            => keys::VK_E                              ,
    Enigo::End                          => keys::VK_END                            ,
    #[cfg(for_windows)]
    Enigo::OEMPlus                      => keys::VK_OEM_PLUS                       ,
    Enigo::Escape                       => keys::VK_ESCAPE                         ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Execute                      => keys::VK_EXECUTE                        ,
    #[cfg(for_windows)]
    Enigo::F                            => keys::VK_F                              ,
    Enigo::F1                           => keys::VK_F1                             ,
    Enigo::F10                          => keys::VK_F10                            ,
    Enigo::F11                          => keys::VK_F11                            ,
    Enigo::F12                          => keys::VK_F12                            ,
    Enigo::F13                          => keys::VK_F13                            ,
    Enigo::F14                          => keys::VK_F14                            ,
    Enigo::F15                          => keys::VK_F15                            ,
    Enigo::F16                          => keys::VK_F16                            ,
    Enigo::F17                          => keys::VK_F17                            ,
    Enigo::F18                          => keys::VK_F18                            ,
    Enigo::F19                          => keys::VK_F19                            ,
    Enigo::F2                           => keys::VK_F2                             ,
    Enigo::F20                          => keys::VK_F20                            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F21                          => keys::VK_F21                            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F22                          => keys::VK_F22                            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F23                          => keys::VK_F23                            ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::F24                          => keys::VK_F24                            ,
    Enigo::F3                           => keys::VK_F3                             ,
    Enigo::F4                           => keys::VK_F4                             ,
    Enigo::F5                           => keys::VK_F5                             ,
    Enigo::F6                           => keys::VK_F6                             ,
    Enigo::F7                           => keys::VK_F7                             ,
    Enigo::F8                           => keys::VK_F8                             ,
    Enigo::F9                           => keys::VK_F9                             ,
    #[cfg(for_windows)]
    Enigo::OEM2                         => keys::VK_OEM_2                          ,
    #[cfg(for_windows)]
    Enigo::G                            => keys::VK_G                              ,
    #[cfg(for_windows)]
    Enigo::OEM3                         => keys::VK_OEM_3                          ,
    #[cfg(for_windows)]
    Enigo::H                            => keys::VK_H                              ,
    Enigo::Help                         => keys::VK_HELP                           ,
    Enigo::Home                         => keys::VK_HOME                           ,
    #[cfg(for_windows)]
    Enigo::I                            => keys::VK_I                              ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Insert                       => keys::VK_INSERT                         ,
    #[cfg(for_windows)]
    Enigo::J                            => keys::VK_J                              ,
    #[cfg(for_windows)]
    Enigo::K                            => keys::VK_K                              ,
    #[cfg(for_windows)]
    Enigo::L                            => keys::VK_L                              ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::LMenu                        => keys::VK_LMENU                          ,
    Enigo::Alt                          => keys::VK_MENU                           ,
    Enigo::Option                       => keys::VK_MENU                           ,
    #[cfg(for_windows)]
    Enigo::OEM7                         => keys::VK_OEM_7                          ,
    Enigo::LeftArrow                    => keys::VK_LEFT                           ,
    #[cfg(for_windows)]
    Enigo::OEM4                         => keys::VK_OEM_4                          ,
    Enigo::Control                      => keys::VK_CONTROL                        ,
    Enigo::LControl                     => keys::VK_LCONTROL                       ,
    #[cfg(for_windows)]
    Enigo::LWin                         => keys::VK_LWIN                           ,
    Enigo::Command                      => keys::VK_LWIN                           ,
    Enigo::Meta                         => keys::VK_LWIN                           ,
    Enigo::Super                        => keys::VK_LWIN                           ,
    Enigo::Windows                      => keys::VK_LWIN                           ,
    Enigo::LShift                       => keys::VK_LSHIFT                         ,
    Enigo::Shift                        => keys::VK_SHIFT                          ,
    #[cfg(for_windows)]
    Enigo::M                            => keys::VK_M                              ,
    #[cfg(for_windows)]
    Enigo::N                            => keys::VK_N                              ,
    #[cfg(for_windows)]
    Enigo::OEM102                       => keys::VK_OEM_102                        ,
    #[cfg(for_windows)]
    Enigo::O                            => keys::VK_O                              ,
    #[cfg(for_windows)]
    Enigo::P                            => keys::VK_P                              ,
    Enigo::PageDown                     => keys::VK_NEXT                           ,
    Enigo::PageUp                       => keys::VK_PRIOR                          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Pause                        => keys::VK_PAUSE                          ,
    #[cfg(for_windows)]
    Enigo::OEMPeriod                    => keys::VK_OEM_PERIOD                     ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::PrintScr                     => keys::VK_SNAPSHOT                       ,
    #[cfg(for_windows)]
    Enigo::Snapshot                     => keys::VK_SNAPSHOT                       ,
    #[cfg(for_windows)]
    Enigo::Q                            => keys::VK_Q                              ,
    #[cfg(for_windows)]
    Enigo::R                            => keys::VK_R                              ,
    Enigo::Return                       => keys::VK_RETURN                         ,
    #[cfg(for_windows)]
    Enigo::RMenu                        => keys::VK_RMENU                          ,
    Enigo::RightArrow                   => keys::VK_RIGHT                          ,
    #[cfg(for_windows)]
    Enigo::OEM6                         => keys::VK_OEM_6                          ,
    Enigo::RControl                     => keys::VK_RCONTROL                       ,
    #[cfg(for_windows)]
    Enigo::RWin                         => keys::VK_RWIN                           ,
    Enigo::RShift                       => keys::VK_RSHIFT                         ,
    #[cfg(for_windows)]
    Enigo::S                            => keys::VK_S                              ,
    #[cfg(for_windows)]
    Enigo::Scroll                       => keys::VK_SCROLL                         ,
    #[cfg(for_linux)]
    Enigo::ScrollLock                   => keys::VK_SCROLL                         ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Select                       => keys::VK_SELECT                         ,
    #[cfg(for_windows)]
    Enigo::OEM1                         => keys::VK_OEM_1                          ,
    Enigo::Space                        => keys::VK_SPACE                          ,
    #[cfg(for_windows)]
    Enigo::T                            => keys::VK_T                              ,
    Enigo::Tab                          => keys::VK_TAB                            ,
    #[cfg(for_windows)]
    Enigo::U                            => keys::VK_U                              ,
    Enigo::UpArrow                      => keys::VK_UP                             ,
    #[cfg(for_windows)]
    Enigo::V                            => keys::VK_V                              ,
    #[cfg(for_windows)]
    Enigo::W                            => keys::VK_W                              ,
    #[cfg(for_windows)]
    Enigo::X                            => keys::VK_X                              ,
    #[cfg(for_windows)]
    Enigo::Y                            => keys::VK_Y                              ,
    #[cfg(for_windows)]
    Enigo::Z                            => keys::VK_Z                              ,
    #[cfg(for_windows)]
    Enigo::Numpad0                      => keys::VK_NUMPAD0                        ,
    #[cfg(for_windows)]
    Enigo::Numpad1                      => keys::VK_NUMPAD1                        ,
    #[cfg(for_windows)]
    Enigo::Numpad2                      => keys::VK_NUMPAD2                        ,
    #[cfg(for_windows)]
    Enigo::Numpad3                      => keys::VK_NUMPAD3                        ,
    #[cfg(for_windows)]
    Enigo::Numpad4                      => keys::VK_NUMPAD4                        ,
    #[cfg(for_windows)]
    Enigo::Numpad5                      => keys::VK_NUMPAD5                        ,
    #[cfg(for_windows)]
    Enigo::Numpad6                      => keys::VK_NUMPAD6                        ,
    #[cfg(for_windows)]
    Enigo::Numpad7                      => keys::VK_NUMPAD7                        ,
    #[cfg(for_windows)]
    Enigo::Numpad8                      => keys::VK_NUMPAD8                        ,
    #[cfg(for_windows)]
    Enigo::Numpad9                      => keys::VK_NUMPAD9                        ,
    #[cfg(for_windows)]
    Enigo::Subtract                     => keys::VK_SUBTRACT                       ,
    #[cfg(for_windows)]
    Enigo::Divide                       => keys::VK_DIVIDE                         ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Numlock                      => keys::VK_NUMLOCK                        ,
    #[cfg(for_windows)]
    Enigo::Decimal                      => keys::VK_DECIMAL                        ,
    #[cfg(for_windows)]
    Enigo::Add                          => keys::VK_ADD                            ,
    #[cfg(for_windows)]
    Enigo::Multiply                     => keys::VK_MULTIPLY                       ,
    #[cfg(for_windows)]
    Enigo::None                         => keys::VK__none_                         ,
    #[cfg(for_windows)]
    Enigo::AbntC1                       => keys::VK_ABNT_C1                        ,
    #[cfg(for_windows)]
    Enigo::AbntC2                       => keys::VK_ABNT_C2                        ,
    #[cfg(for_windows)]
    Enigo::Attn                         => keys::VK_ATTN                           ,
    #[cfg(for_windows)]
    Enigo::Crsel                        => keys::VK_CRSEL                          ,
    #[cfg(for_windows)]
    Enigo::DBEAlphanumeric              => keys::VK_DBE_ALPHANUMERIC               ,
    #[cfg(for_windows)]
    Enigo::DBECodeinput                 => keys::VK_DBE_CODEINPUT                  ,
    #[cfg(for_windows)]
    Enigo::DBESChar                     => keys::VK_DBE_DBCSCHAR                   ,
    #[cfg(for_windows)]
    Enigo::DBEDetermineString           => keys::VK_DBE_DETERMINESTRING            ,
    #[cfg(for_windows)]
    Enigo::DBEEnterDLGConversionMode    => keys::VK_DBE_ENTERDLGCONVERSIONMODE     ,
    #[cfg(for_windows)]
    Enigo::DBEEnterIMEConfigMode        => keys::VK_DBE_ENTERIMECONFIGMODE         ,
    #[cfg(for_windows)]
    Enigo::DBEEnterWordRegisterMode     => keys::VK_DBE_ENTERWORDREGISTERMODE      ,
    #[cfg(for_windows)]
    Enigo::DBEFlushString               => keys::VK_DBE_FLUSHSTRING                ,
    #[cfg(for_windows)]
    Enigo::DBEHiragana                  => keys::VK_DBE_HIRAGANA                   ,
    #[cfg(for_windows)]
    Enigo::DBEKatakana                  => keys::VK_DBE_KATAKANA                   ,
    #[cfg(for_windows)]
    Enigo::DBENoCodepoint               => keys::VK_DBE_NOCODEINPUT                ,
    #[cfg(for_windows)]
    Enigo::DBENoRoman                   => keys::VK_DBE_NOROMAN                    ,
    #[cfg(for_windows)]
    Enigo::DBERoman                     => keys::VK_DBE_ROMAN                      ,
    #[cfg(for_windows)]
    Enigo::DBESBCSChar                  => keys::VK_DBE_SBCSCHAR                   ,
    #[cfg(for_windows)]
    Enigo::Ereof                        => keys::VK_EREOF                          ,
    #[cfg(for_windows)]
    Enigo::Exsel                        => keys::VK_EXSEL                          ,
    #[cfg(for_windows)]
    Enigo::GamepadA                     => keys::VK_GAMEPAD_A                      ,
    #[cfg(for_windows)]
    Enigo::GamepadB                     => keys::VK_GAMEPAD_B                      ,
    #[cfg(for_windows)]
    Enigo::GamepadDPadDown              => keys::VK_GAMEPAD_DPAD_DOWN              ,
    #[cfg(for_windows)]
    Enigo::GamepadDPadLeft              => keys::VK_GAMEPAD_DPAD_LEFT              ,
    #[cfg(for_windows)]
    Enigo::GamepadDPadRight             => keys::VK_GAMEPAD_DPAD_RIGHT             ,
    #[cfg(for_windows)]
    Enigo::GamepadDPadUp                => keys::VK_GAMEPAD_DPAD_UP                ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftShoulder          => keys::VK_GAMEPAD_LEFT_SHOULDER          ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftThumbstickButton  => keys::VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftThumbstickDown    => keys::VK_GAMEPAD_LEFT_THUMBSTICK_DOWN   ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftThumbstickLeft    => keys::VK_GAMEPAD_LEFT_THUMBSTICK_LEFT   ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftThumbstickRight   => keys::VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT  ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftThumbstickUp      => keys::VK_GAMEPAD_LEFT_THUMBSTICK_UP     ,
    #[cfg(for_windows)]
    Enigo::GamepadLeftTrigger           => keys::VK_GAMEPAD_LEFT_TRIGGER           ,
    #[cfg(for_windows)]
    Enigo::GamepadMenu                  => keys::VK_GAMEPAD_MENU                   ,
    #[cfg(for_windows)]
    Enigo::GamepadRightShoulder         => keys::VK_GAMEPAD_RIGHT_SHOULDER         ,
    #[cfg(for_windows)]
    Enigo::GamepadRightThumbstickButton => keys::VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
    #[cfg(for_windows)]
    Enigo::GamepadRightThumbstickDown   => keys::VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN  ,
    #[cfg(for_windows)]
    Enigo::GamepadRightThumbstickLeft   => keys::VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT  ,
    #[cfg(for_windows)]
    Enigo::GamepadRightThumbstickRight  => keys::VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT ,
    #[cfg(for_windows)]
    Enigo::GamepadRightThumbstickUp     => keys::VK_GAMEPAD_RIGHT_THUMBSTICK_UP    ,
    #[cfg(for_windows)]
    Enigo::GamepadRightTrigger          => keys::VK_GAMEPAD_RIGHT_TRIGGER          ,
    #[cfg(for_windows)]
    Enigo::GamepadView                  => keys::VK_GAMEPAD_VIEW                   ,
    #[cfg(for_windows)]
    Enigo::GamepadX                     => keys::VK_GAMEPAD_X                      ,
    #[cfg(for_windows)]
    Enigo::GamepadY                     => keys::VK_GAMEPAD_Y                      ,
    #[cfg(for_windows)]
    Enigo::Ico00                        => keys::VK_ICO_00                         ,
    #[cfg(for_windows)]
    Enigo::IcoClear                     => keys::VK_ICO_CLEAR                      ,
    #[cfg(for_windows)]
    Enigo::IcoHelp                      => keys::VK_ICO_HELP                       ,
    #[cfg(for_windows)]
    Enigo::NavigationAccept             => keys::VK_NAVIGATION_ACCEPT              ,
    #[cfg(for_windows)]
    Enigo::NavigationCancel             => keys::VK_NAVIGATION_CANCEL              ,
    #[cfg(for_windows)]
    Enigo::NavigationDown               => keys::VK_NAVIGATION_DOWN                ,
    #[cfg(for_windows)]
    Enigo::NavigationLeft               => keys::VK_NAVIGATION_LEFT                ,
    #[cfg(for_windows)]
    Enigo::NavigationMenu               => keys::VK_NAVIGATION_MENU                ,
    #[cfg(for_windows)]
    Enigo::NavigationRight              => keys::VK_NAVIGATION_RIGHT               ,
    #[cfg(for_windows)]
    Enigo::NavigationUp                 => keys::VK_NAVIGATION_UP                  ,
    #[cfg(for_windows)]
    Enigo::NavigationView               => keys::VK_NAVIGATION_VIEW                ,
    #[cfg(for_windows)]
    Enigo::NoName                       => keys::VK_NONAME                         ,
    #[cfg(for_windows)]
    Enigo::OEM8                         => keys::VK_OEM_8                          ,
    #[cfg(for_windows)]
    Enigo::OEMAttn                      => keys::VK_OEM_ATTN                       ,
    #[cfg(for_windows)]
    Enigo::OEMAuto                      => keys::VK_OEM_AUTO                       ,
    #[cfg(for_windows)]
    Enigo::OEMAx                        => keys::VK_OEM_AX                         ,
    #[cfg(for_windows)]
    Enigo::OEMBacktab                   => keys::VK_OEM_BACKTAB                    ,
    #[cfg(for_windows)]
    Enigo::OEMClear                     => keys::VK_OEM_CLEAR                      ,
    #[cfg(for_windows)]
    Enigo::OEMCopy                      => keys::VK_OEM_COPY                       ,
    #[cfg(for_windows)]
    Enigo::OEMCusel                     => keys::VK_OEM_CUSEL                      ,
    #[cfg(for_windows)]
    Enigo::OEMEnlw                      => keys::VK_OEM_ENLW                       ,
    #[cfg(for_windows)]
    Enigo::OEMFinish                    => keys::VK_OEM_FINISH                     ,
    #[cfg(for_windows)]
    Enigo::OEMFJJisho                   => keys::VK_OEM_FJ_JISHO                   ,
    #[cfg(for_windows)]
    Enigo::OEMFJLoya                    => keys::VK_OEM_FJ_LOYA                    ,
    #[cfg(for_windows)]
    Enigo::OEMFJMasshou                 => keys::VK_OEM_FJ_MASSHOU                 ,
    #[cfg(for_windows)]
    Enigo::OEMFJRoya                    => keys::VK_OEM_FJ_ROYA                    ,
    #[cfg(for_windows)]
    Enigo::OEMFJTouroku                 => keys::VK_OEM_FJ_TOUROKU                 ,
    #[cfg(for_windows)]
    Enigo::OEMJump                      => keys::VK_OEM_JUMP                       ,
    #[cfg(for_windows)]
    Enigo::OEMNECEqual                  => keys::VK_OEM_NEC_EQUAL                  ,
    #[cfg(for_windows)]
    Enigo::OEMPA1                       => keys::VK_OEM_PA1                        ,
    #[cfg(for_windows)]
    Enigo::OEMPA2                       => keys::VK_OEM_PA2                        ,
    #[cfg(for_windows)]
    Enigo::OEMPA3                       => keys::VK_OEM_PA3                        ,
    #[cfg(for_windows)]
    Enigo::OEMReset                     => keys::VK_OEM_RESET                      ,
    #[cfg(for_windows)]
    Enigo::OEMWsctrl                    => keys::VK_OEM_WSCTRL                     ,
    #[cfg(for_windows)]
    Enigo::PA1                          => keys::VK_PA1                            ,
    #[cfg(for_windows)]
    Enigo::Packet                       => keys::VK_PACKET                         ,
    #[cfg(for_windows)]
    Enigo::Play                         => keys::VK_PLAY                           ,
    #[cfg(for_windows)]
    Enigo::Processkey                   => keys::VK_PROCESSKEY                     ,
    #[cfg(for_windows)]
    Enigo::Zoom                         => keys::VK_ZOOM                           ,
    #[cfg(for_windows)]
    Enigo::Accept                       => keys::VK_ACCEPT                         ,
    #[cfg(for_windows)]
    Enigo::Convert                      => keys::VK_CONVERT                        ,
    #[cfg(for_windows)]
    Enigo::Final                        => keys::VK_FINAL                          ,
    #[cfg(for_windows)]
    Enigo::IMEOff                       => keys::VK_IME_OFF                        ,
    #[cfg(for_windows)]
    Enigo::IMEOn                        => keys::VK_IME_ON                         ,
    #[cfg(for_windows)]
    Enigo::Junja                        => keys::VK_JUNJA                          ,
    #[cfg(for_windows)]
    Enigo::Kana                         => keys::VK_KANA                           ,
    #[cfg(for_windows)]
    Enigo::Hangeul                      => keys::VK_HANGEUL                        ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Hangul                       => keys::VK_HANGUL                         ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Kanji                        => keys::VK_KANJI                          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Hanja                        => keys::VK_HANJA                          ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::ModeChange                   => keys::VK_MODECHANGE                     ,
    #[cfg(for_windows)]
    Enigo::NonConvert                   => keys::VK_NONCONVERT                     ,
    #[cfg(any(for_windows, for_linux))]
    Enigo::Print                        => keys::VK_PRINT                          ,
    #[cfg(for_windows)]
    Enigo::Separator                    => keys::VK_SEPARATOR                      ,
    #[cfg(for_windows)]
    Enigo::LaunchApp1                   => keys::VK_LAUNCH_APP1                    ,
    #[cfg(for_windows)]
    Enigo::LaunchApp2                   => keys::VK_LAUNCH_APP2                    ,
    _ => return None,
  };
  Some(result)
}

impl crate::convert::Convert<Enigo, Vk> for crate::convert::Converter {
  fn convert(value: Enigo) -> Option<Vk> {
    enigo_to_vk(value)
  }
}
