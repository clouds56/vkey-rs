fn main() {
  let path = &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/cache/numeric");
  let file = |name: &str| std::fs::File::create(path.join(name)).unwrap();
  std::fs::create_dir_all(path).unwrap();


  println!("\n\n============== HUT 04 ===============");
  print_hut_04(&mut file("hut_04.txt"), true).ok();
  print_hut_04(&mut std::io::stdout(), false).ok();


  println!("\n\n============== WINDOWS ===============");
  print_windows(&mut file("windows.txt"), true).ok();
  print_windows(&mut std::io::stdout(), false).ok();


  println!("\n\n============== KEYSYM ===============");
  print_keysym(&mut file("keysym.txt"), true).ok();
  print_keysym(&mut std::io::stdout(), false).ok();


  println!("\n\n============== MACOS ===============");
  print_macos(&mut file("macos.txt"), true).ok();
  print_macos(&mut std::io::stdout(), false).ok();


  #[cfg(windows)] {
  println!("\n\n============== WIN MAKECODE1 ===============");
  print_win_makecode1(&mut file("win_makecode1.txt"), true).ok();
  print_win_makecode1(&mut std::io::stdout(), false).ok();
  }

}



fn print_hut_04(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {
  use vkey::deps::hut::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad};
          writeln!(w, "{:52}, 0x{:X},", "Button::Button(1)", Button::Button(1).usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Button::Button(2)", Button::Button(2).usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Button::Button(3)", Button::Button(3).usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Button::Button(4)", Button::Button(4).usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Button::Button(5)", Button::Button(5).usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACBack", Consumer::ACBack.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACBookmarks", Consumer::ACBookmarks.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACForward", Consumer::ACForward.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACHome", Consumer::ACHome.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACRefresh", Consumer::ACRefresh.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACSearch", Consumer::ACSearch.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ACStop", Consumer::ACStop.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ALEmailReader", Consumer::ALEmailReader.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::MediaSelection", Consumer::MediaSelection.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::Mute", Consumer::Mute.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::PlayPause", Consumer::PlayPause.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ScanNextTrack", Consumer::ScanNextTrack.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::ScanPreviousTrack", Consumer::ScanPreviousTrack.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::Stop", Consumer::Stop.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::VolumeDecrement", Consumer::VolumeDecrement.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "Consumer::VolumeIncrement", Consumer::VolumeIncrement.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "GenericDesktop::SystemSleep", GenericDesktop::SystemSleep.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard0andRightBracket", KeyboardKeypad::Keyboard0andRightBracket.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard1andBang", KeyboardKeypad::Keyboard1andBang.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard2andAt", KeyboardKeypad::Keyboard2andAt.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard3andHash", KeyboardKeypad::Keyboard3andHash.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard4andDollar", KeyboardKeypad::Keyboard4andDollar.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard5andPercent", KeyboardKeypad::Keyboard5andPercent.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard6andCaret", KeyboardKeypad::Keyboard6andCaret.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard7andAmpersand", KeyboardKeypad::Keyboard7andAmpersand.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard8andStar", KeyboardKeypad::Keyboard8andStar.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keyboard9andLeftBracket", KeyboardKeypad::Keyboard9andLeftBracket.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardA", KeyboardKeypad::KeyboardA.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardApplication", KeyboardKeypad::KeyboardApplication.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardB", KeyboardKeypad::KeyboardB.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardBackslashandPipe", KeyboardKeypad::KeyboardBackslashandPipe.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardC", KeyboardKeypad::KeyboardC.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardCancel", KeyboardKeypad::KeyboardCancel.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardCapsLock", KeyboardKeypad::KeyboardCapsLock.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardClear", KeyboardKeypad::KeyboardClear.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardCommaandLessThan", KeyboardKeypad::KeyboardCommaandLessThan.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardD", KeyboardKeypad::KeyboardD.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardDashandUnderscore", KeyboardKeypad::KeyboardDashandUnderscore.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardDelete", KeyboardKeypad::KeyboardDelete.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardDeleteForward", KeyboardKeypad::KeyboardDeleteForward.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardDownArrow", KeyboardKeypad::KeyboardDownArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardE", KeyboardKeypad::KeyboardE.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardEnd", KeyboardKeypad::KeyboardEnd.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardEqualsandPlus", KeyboardKeypad::KeyboardEqualsandPlus.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardEscape", KeyboardKeypad::KeyboardEscape.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardExecute", KeyboardKeypad::KeyboardExecute.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF", KeyboardKeypad::KeyboardF.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF1", KeyboardKeypad::KeyboardF1.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF10", KeyboardKeypad::KeyboardF10.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF11", KeyboardKeypad::KeyboardF11.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF12", KeyboardKeypad::KeyboardF12.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF13", KeyboardKeypad::KeyboardF13.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF14", KeyboardKeypad::KeyboardF14.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF15", KeyboardKeypad::KeyboardF15.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF16", KeyboardKeypad::KeyboardF16.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF17", KeyboardKeypad::KeyboardF17.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF18", KeyboardKeypad::KeyboardF18.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF19", KeyboardKeypad::KeyboardF19.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF2", KeyboardKeypad::KeyboardF2.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF20", KeyboardKeypad::KeyboardF20.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF21", KeyboardKeypad::KeyboardF21.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF22", KeyboardKeypad::KeyboardF22.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF23", KeyboardKeypad::KeyboardF23.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF24", KeyboardKeypad::KeyboardF24.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF3", KeyboardKeypad::KeyboardF3.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF4", KeyboardKeypad::KeyboardF4.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF5", KeyboardKeypad::KeyboardF5.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF6", KeyboardKeypad::KeyboardF6.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF7", KeyboardKeypad::KeyboardF7.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF8", KeyboardKeypad::KeyboardF8.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardF9", KeyboardKeypad::KeyboardF9.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardForwardSlashandQuestionMark", KeyboardKeypad::KeyboardForwardSlashandQuestionMark.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardG", KeyboardKeypad::KeyboardG.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardGraveAccentandTilde", KeyboardKeypad::KeyboardGraveAccentandTilde.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardH", KeyboardKeypad::KeyboardH.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardHelp", KeyboardKeypad::KeyboardHelp.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardHome", KeyboardKeypad::KeyboardHome.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardI", KeyboardKeypad::KeyboardI.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardInsert", KeyboardKeypad::KeyboardInsert.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardJ", KeyboardKeypad::KeyboardJ.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardK", KeyboardKeypad::KeyboardK.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardL", KeyboardKeypad::KeyboardL.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftAlt", KeyboardKeypad::KeyboardLeftAlt.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftAlt*", KeyboardKeypad::KeyboardLeftAlt.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftAlt*", KeyboardKeypad::KeyboardLeftAlt.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftAposandDouble", KeyboardKeypad::KeyboardLeftAposandDouble.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftArrow", KeyboardKeypad::KeyboardLeftArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftBrace", KeyboardKeypad::KeyboardLeftBrace.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftControl*", KeyboardKeypad::KeyboardLeftControl.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftControl", KeyboardKeypad::KeyboardLeftControl.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftGUI", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftShift", KeyboardKeypad::KeyboardLeftShift.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardLeftShift*", KeyboardKeypad::KeyboardLeftShift.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardM", KeyboardKeypad::KeyboardM.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardN", KeyboardKeypad::KeyboardN.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardNonUSBackslashandPipe", KeyboardKeypad::KeyboardNonUSBackslashandPipe.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardO", KeyboardKeypad::KeyboardO.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardP", KeyboardKeypad::KeyboardP.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardPageDown", KeyboardKeypad::KeyboardPageDown.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardPageUp", KeyboardKeypad::KeyboardPageUp.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardPause", KeyboardKeypad::KeyboardPause.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardPeriodandGreaterThan", KeyboardKeypad::KeyboardPeriodandGreaterThan.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardPrintScreen", KeyboardKeypad::KeyboardPrintScreen.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardPrintScreen*", KeyboardKeypad::KeyboardPrintScreen.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardQ", KeyboardKeypad::KeyboardQ.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardR", KeyboardKeypad::KeyboardR.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardReturnEnter", KeyboardKeypad::KeyboardReturnEnter.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightAlt", KeyboardKeypad::KeyboardRightAlt.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightAlt*", KeyboardKeypad::KeyboardRightAlt.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightArrow", KeyboardKeypad::KeyboardRightArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightBrace", KeyboardKeypad::KeyboardRightBrace.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightControl", KeyboardKeypad::KeyboardRightControl.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightGUI", KeyboardKeypad::KeyboardRightGUI.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightGUI*", KeyboardKeypad::KeyboardRightGUI.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardRightShift", KeyboardKeypad::KeyboardRightShift.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardS", KeyboardKeypad::KeyboardS.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardScrollLock", KeyboardKeypad::KeyboardScrollLock.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardScrollLock*", KeyboardKeypad::KeyboardScrollLock.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardSelect", KeyboardKeypad::KeyboardSelect.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardSemiColonandColon", KeyboardKeypad::KeyboardSemiColonandColon.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardSpacebar", KeyboardKeypad::KeyboardSpacebar.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardT", KeyboardKeypad::KeyboardT.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardTab", KeyboardKeypad::KeyboardTab.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardU", KeyboardKeypad::KeyboardU.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardUpArrow", KeyboardKeypad::KeyboardUpArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardV", KeyboardKeypad::KeyboardV.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardW", KeyboardKeypad::KeyboardW.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardX", KeyboardKeypad::KeyboardX.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardY", KeyboardKeypad::KeyboardY.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeyboardZ", KeyboardKeypad::KeyboardZ.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad0andInsert", KeyboardKeypad::Keypad0andInsert.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad1andEnd", KeyboardKeypad::Keypad1andEnd.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad2andDownArrow", KeyboardKeypad::Keypad2andDownArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad3andPageDn", KeyboardKeypad::Keypad3andPageDn.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad4andLeftArrow", KeyboardKeypad::Keypad4andLeftArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad5", KeyboardKeypad::Keypad5.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad6andRightArrow", KeyboardKeypad::Keypad6andRightArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad7andHome", KeyboardKeypad::Keypad7andHome.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad8andUpArrow", KeyboardKeypad::Keypad8andUpArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::Keypad9andPageUp", KeyboardKeypad::Keypad9andPageUp.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeypadDash", KeyboardKeypad::KeypadDash.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeypadForwardSlash", KeyboardKeypad::KeypadForwardSlash.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeypadNumLockandClear", KeyboardKeypad::KeypadNumLockandClear.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeypadPeriodandDelete", KeyboardKeypad::KeypadPeriodandDelete.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeypadPlus", KeyboardKeypad::KeypadPlus.usage_value())?;
          writeln!(w, "{:52}, 0x{:X},", "KeyboardKeypad::KeypadStar", KeyboardKeypad::KeypadStar.usage_value())?;
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:52}, {},", "None", "    ")?; }
  Ok(())
}


fn print_windows(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {
  use vkey::mirror::windows as keys;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LBUTTON", keys::VK_LBUTTON.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_RBUTTON", keys::VK_RBUTTON.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MBUTTON", keys::VK_MBUTTON.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_XBUTTON1", keys::VK_XBUTTON1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_XBUTTON2", keys::VK_XBUTTON2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_BACK", keys::VK_BROWSER_BACK.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_FAVORITES", keys::VK_BROWSER_FAVORITES.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_FORWARD", keys::VK_BROWSER_FORWARD.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_HOME", keys::VK_BROWSER_HOME.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_REFRESH", keys::VK_BROWSER_REFRESH.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_SEARCH", keys::VK_BROWSER_SEARCH.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BROWSER_STOP", keys::VK_BROWSER_STOP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LAUNCH_MAIL", keys::VK_LAUNCH_MAIL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LAUNCH_MEDIA_SELECT", keys::VK_LAUNCH_MEDIA_SELECT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_VOLUME_MUTE", keys::VK_VOLUME_MUTE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MEDIA_PLAY_PAUSE", keys::VK_MEDIA_PLAY_PAUSE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MEDIA_NEXT_TRACK", keys::VK_MEDIA_NEXT_TRACK.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MEDIA_PREV_TRACK", keys::VK_MEDIA_PREV_TRACK.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MEDIA_STOP", keys::VK_MEDIA_STOP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_VOLUME_DOWN", keys::VK_VOLUME_DOWN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_VOLUME_UP", keys::VK_VOLUME_UP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SLEEP", keys::VK_SLEEP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_0", keys::VK_0.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_1", keys::VK_1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_2", keys::VK_2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_3", keys::VK_3.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_4", keys::VK_4.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_5", keys::VK_5.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_6", keys::VK_6.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_7", keys::VK_7.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_8", keys::VK_8.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_9", keys::VK_9.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_A", keys::VK_A.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_APPS", keys::VK_APPS.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_B", keys::VK_B.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_5", keys::VK_OEM_5.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_C", keys::VK_C.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_CANCEL", keys::VK_CANCEL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_CAPITAL", keys::VK_CAPITAL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_CLEAR", keys::VK_CLEAR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_COMMA", keys::VK_OEM_COMMA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_D", keys::VK_D.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_MINUS", keys::VK_OEM_MINUS.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_BACK", keys::VK_BACK.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DELETE", keys::VK_DELETE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DOWN", keys::VK_DOWN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_E", keys::VK_E.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_END", keys::VK_END.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_PLUS", keys::VK_OEM_PLUS.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ESCAPE", keys::VK_ESCAPE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_EXECUTE", keys::VK_EXECUTE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F", keys::VK_F.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F1", keys::VK_F1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F10", keys::VK_F10.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F11", keys::VK_F11.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F12", keys::VK_F12.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F13", keys::VK_F13.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F14", keys::VK_F14.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F15", keys::VK_F15.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F16", keys::VK_F16.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F17", keys::VK_F17.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F18", keys::VK_F18.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F19", keys::VK_F19.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F2", keys::VK_F2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F20", keys::VK_F20.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F21", keys::VK_F21.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F22", keys::VK_F22.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F23", keys::VK_F23.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F24", keys::VK_F24.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F3", keys::VK_F3.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F4", keys::VK_F4.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F5", keys::VK_F5.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F6", keys::VK_F6.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F7", keys::VK_F7.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F8", keys::VK_F8.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_F9", keys::VK_F9.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_2", keys::VK_OEM_2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_G", keys::VK_G.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_3", keys::VK_OEM_3.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_H", keys::VK_H.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_HELP", keys::VK_HELP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_HOME", keys::VK_HOME.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_I", keys::VK_I.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_INSERT", keys::VK_INSERT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_J", keys::VK_J.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_K", keys::VK_K.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_L", keys::VK_L.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LMENU", keys::VK_LMENU.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MENU", keys::VK_MENU.0)?;
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_MENU*", keys::VK_MENU.0)?;}
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_7", keys::VK_OEM_7.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LEFT", keys::VK_LEFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_4", keys::VK_OEM_4.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_CONTROL", keys::VK_CONTROL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LCONTROL", keys::VK_LCONTROL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LWIN", keys::VK_LWIN.0)?;
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_LWIN*", keys::VK_LWIN.0)?;}
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_LWIN*", keys::VK_LWIN.0)?;}
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_LWIN*", keys::VK_LWIN.0)?;}
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_LWIN*", keys::VK_LWIN.0)?;}
          writeln!(w, "{:35}, 0x{:02X},", "VK_LSHIFT", keys::VK_LSHIFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SHIFT", keys::VK_SHIFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_M", keys::VK_M.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_N", keys::VK_N.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_102", keys::VK_OEM_102.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_O", keys::VK_O.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_P", keys::VK_P.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NEXT", keys::VK_NEXT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PRIOR", keys::VK_PRIOR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PAUSE", keys::VK_PAUSE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_PERIOD", keys::VK_OEM_PERIOD.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SNAPSHOT", keys::VK_SNAPSHOT.0)?;
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_SNAPSHOT*", keys::VK_SNAPSHOT.0)?;}
          writeln!(w, "{:35}, 0x{:02X},", "VK_Q", keys::VK_Q.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_R", keys::VK_R.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_RETURN", keys::VK_RETURN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_RMENU", keys::VK_RMENU.0)?;
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_RMENU*", keys::VK_RMENU.0)?;}
          writeln!(w, "{:35}, 0x{:02X},", "VK_RIGHT", keys::VK_RIGHT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_6", keys::VK_OEM_6.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_RCONTROL", keys::VK_RCONTROL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_RWIN", keys::VK_RWIN.0)?;
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_RWIN*", keys::VK_RWIN.0)?;}
          writeln!(w, "{:35}, 0x{:02X},", "VK_RSHIFT", keys::VK_RSHIFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_S", keys::VK_S.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SCROLL", keys::VK_SCROLL.0)?;
  if any {writeln!(w, "{:35}, 0x{:02X},", "VK_SCROLL*", keys::VK_SCROLL.0)?;}
          writeln!(w, "{:35}, 0x{:02X},", "VK_SELECT", keys::VK_SELECT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_1", keys::VK_OEM_1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SPACE", keys::VK_SPACE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_T", keys::VK_T.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_TAB", keys::VK_TAB.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_U", keys::VK_U.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_UP", keys::VK_UP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_V", keys::VK_V.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_W", keys::VK_W.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_X", keys::VK_X.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_Y", keys::VK_Y.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_Z", keys::VK_Z.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD0", keys::VK_NUMPAD0.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD1", keys::VK_NUMPAD1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD2", keys::VK_NUMPAD2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD3", keys::VK_NUMPAD3.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD4", keys::VK_NUMPAD4.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD5", keys::VK_NUMPAD5.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD6", keys::VK_NUMPAD6.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD7", keys::VK_NUMPAD7.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD8", keys::VK_NUMPAD8.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMPAD9", keys::VK_NUMPAD9.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SUBTRACT", keys::VK_SUBTRACT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DIVIDE", keys::VK_DIVIDE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NUMLOCK", keys::VK_NUMLOCK.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DECIMAL", keys::VK_DECIMAL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ADD", keys::VK_ADD.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MULTIPLY", keys::VK_MULTIPLY.0)?;
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
          writeln!(w, "{:35}, 0x{:02X},", "VK__none_", keys::VK__none_.0)?;
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:35}, {},", "None", "    ")?; }
          writeln!(w, "{:35}, 0x{:02X},", "VK_ABNT_C1", keys::VK_ABNT_C1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ABNT_C2", keys::VK_ABNT_C2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ATTN", keys::VK_ATTN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_CRSEL", keys::VK_CRSEL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_ALPHANUMERIC", keys::VK_DBE_ALPHANUMERIC.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_CODEINPUT", keys::VK_DBE_CODEINPUT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_DBCSCHAR", keys::VK_DBE_DBCSCHAR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_DETERMINESTRING", keys::VK_DBE_DETERMINESTRING.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_ENTERDLGCONVERSIONMODE", keys::VK_DBE_ENTERDLGCONVERSIONMODE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_ENTERIMECONFIGMODE", keys::VK_DBE_ENTERIMECONFIGMODE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_ENTERWORDREGISTERMODE", keys::VK_DBE_ENTERWORDREGISTERMODE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_FLUSHSTRING", keys::VK_DBE_FLUSHSTRING.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_HIRAGANA", keys::VK_DBE_HIRAGANA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_KATAKANA", keys::VK_DBE_KATAKANA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_NOCODEINPUT", keys::VK_DBE_NOCODEINPUT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_NOROMAN", keys::VK_DBE_NOROMAN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_ROMAN", keys::VK_DBE_ROMAN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_DBE_SBCSCHAR", keys::VK_DBE_SBCSCHAR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_EREOF", keys::VK_EREOF.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_EXSEL", keys::VK_EXSEL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_A", keys::VK_GAMEPAD_A.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_B", keys::VK_GAMEPAD_B.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_DPAD_DOWN", keys::VK_GAMEPAD_DPAD_DOWN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_DPAD_LEFT", keys::VK_GAMEPAD_DPAD_LEFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_DPAD_RIGHT", keys::VK_GAMEPAD_DPAD_RIGHT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_DPAD_UP", keys::VK_GAMEPAD_DPAD_UP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_SHOULDER", keys::VK_GAMEPAD_LEFT_SHOULDER.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON", keys::VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_THUMBSTICK_DOWN", keys::VK_GAMEPAD_LEFT_THUMBSTICK_DOWN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_THUMBSTICK_LEFT", keys::VK_GAMEPAD_LEFT_THUMBSTICK_LEFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT", keys::VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_THUMBSTICK_UP", keys::VK_GAMEPAD_LEFT_THUMBSTICK_UP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_LEFT_TRIGGER", keys::VK_GAMEPAD_LEFT_TRIGGER.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_MENU", keys::VK_GAMEPAD_MENU.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_SHOULDER", keys::VK_GAMEPAD_RIGHT_SHOULDER.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON", keys::VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN", keys::VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT", keys::VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT", keys::VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_THUMBSTICK_UP", keys::VK_GAMEPAD_RIGHT_THUMBSTICK_UP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_RIGHT_TRIGGER", keys::VK_GAMEPAD_RIGHT_TRIGGER.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_VIEW", keys::VK_GAMEPAD_VIEW.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_X", keys::VK_GAMEPAD_X.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_GAMEPAD_Y", keys::VK_GAMEPAD_Y.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ICO_00", keys::VK_ICO_00.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ICO_CLEAR", keys::VK_ICO_CLEAR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ICO_HELP", keys::VK_ICO_HELP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_ACCEPT", keys::VK_NAVIGATION_ACCEPT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_CANCEL", keys::VK_NAVIGATION_CANCEL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_DOWN", keys::VK_NAVIGATION_DOWN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_LEFT", keys::VK_NAVIGATION_LEFT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_MENU", keys::VK_NAVIGATION_MENU.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_RIGHT", keys::VK_NAVIGATION_RIGHT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_UP", keys::VK_NAVIGATION_UP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NAVIGATION_VIEW", keys::VK_NAVIGATION_VIEW.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NONAME", keys::VK_NONAME.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_8", keys::VK_OEM_8.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_ATTN", keys::VK_OEM_ATTN.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_AUTO", keys::VK_OEM_AUTO.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_AX", keys::VK_OEM_AX.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_BACKTAB", keys::VK_OEM_BACKTAB.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_CLEAR", keys::VK_OEM_CLEAR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_COPY", keys::VK_OEM_COPY.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_CUSEL", keys::VK_OEM_CUSEL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_ENLW", keys::VK_OEM_ENLW.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_FINISH", keys::VK_OEM_FINISH.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_FJ_JISHO", keys::VK_OEM_FJ_JISHO.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_FJ_LOYA", keys::VK_OEM_FJ_LOYA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_FJ_MASSHOU", keys::VK_OEM_FJ_MASSHOU.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_FJ_ROYA", keys::VK_OEM_FJ_ROYA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_FJ_TOUROKU", keys::VK_OEM_FJ_TOUROKU.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_JUMP", keys::VK_OEM_JUMP.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_NEC_EQUAL", keys::VK_OEM_NEC_EQUAL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_PA1", keys::VK_OEM_PA1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_PA2", keys::VK_OEM_PA2.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_PA3", keys::VK_OEM_PA3.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_RESET", keys::VK_OEM_RESET.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_OEM_WSCTRL", keys::VK_OEM_WSCTRL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PA1", keys::VK_PA1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PACKET", keys::VK_PACKET.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PLAY", keys::VK_PLAY.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PROCESSKEY", keys::VK_PROCESSKEY.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ZOOM", keys::VK_ZOOM.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_ACCEPT", keys::VK_ACCEPT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_CONVERT", keys::VK_CONVERT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_FINAL", keys::VK_FINAL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_IME_OFF", keys::VK_IME_OFF.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_IME_ON", keys::VK_IME_ON.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_JUNJA", keys::VK_JUNJA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_KANA", keys::VK_KANA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_HANGEUL", keys::VK_HANGEUL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_HANGUL", keys::VK_HANGUL.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_KANJI", keys::VK_KANJI.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_HANJA", keys::VK_HANJA.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_MODECHANGE", keys::VK_MODECHANGE.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_NONCONVERT", keys::VK_NONCONVERT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_PRINT", keys::VK_PRINT.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_SEPARATOR", keys::VK_SEPARATOR.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LAUNCH_APP1", keys::VK_LAUNCH_APP1.0)?;
          writeln!(w, "{:35}, 0x{:02X},", "VK_LAUNCH_APP2", keys::VK_LAUNCH_APP2.0)?;
  Ok(())
}


fn print_keysym(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {
  use vkey::deps::xkeysym::Keysym;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioMute", Keysym::XF86_AudioMute.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioPlay", Keysym::XF86_AudioPlay.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioNext", Keysym::XF86_AudioNext.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioPrev", Keysym::XF86_AudioPrev.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioStop", Keysym::XF86_AudioStop.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioLowerVolume", Keysym::XF86_AudioLowerVolume.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioRaiseVolume", Keysym::XF86_AudioRaiseVolume.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_0", Keysym::_0.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_1", Keysym::_1.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_2", Keysym::_2.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_3", Keysym::_3.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_4", Keysym::_4.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_5", Keysym::_5.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_6", Keysym::_6.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_7", Keysym::_7.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_8", Keysym::_8.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::_9", Keysym::_9.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::A", Keysym::A.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::B", Keysym::B.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::C", Keysym::C.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Cancel", Keysym::Cancel.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Caps_Lock", Keysym::Caps_Lock.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Clear", Keysym::Clear.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::D", Keysym::D.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::BackSpace", Keysym::BackSpace.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Delete", Keysym::Delete.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Down", Keysym::Down.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::E", Keysym::E.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::End", Keysym::End.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Escape", Keysym::Escape.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Execute", Keysym::Execute.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F", Keysym::F.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F1", Keysym::F1.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F10", Keysym::F10.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F11", Keysym::F11.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F12", Keysym::F12.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F13", Keysym::F13.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F14", Keysym::F14.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F15", Keysym::F15.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F16", Keysym::F16.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F17", Keysym::F17.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F18", Keysym::F18.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F19", Keysym::F19.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F2", Keysym::F2.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F20", Keysym::F20.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F21", Keysym::F21.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F22", Keysym::F22.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F23", Keysym::F23.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F24", Keysym::F24.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F3", Keysym::F3.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F4", Keysym::F4.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F5", Keysym::F5.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F6", Keysym::F6.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F7", Keysym::F7.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F8", Keysym::F8.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F9", Keysym::F9.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::G", Keysym::G.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::H", Keysym::H.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Help", Keysym::Help.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Home", Keysym::Home.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::I", Keysym::I.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Insert", Keysym::Insert.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::J", Keysym::J.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::K", Keysym::K.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::L", Keysym::L.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Menu", Keysym::Menu.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Alt_L", Keysym::Alt_L.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Alt_L*", Keysym::Alt_L.raw())?;}
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Left", Keysym::Left.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Control_L", Keysym::Control_L.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Control_L", Keysym::Control_L.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_L*", Keysym::Super_L.raw())?;}
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_L*", Keysym::Super_L.raw())?;}
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_L", Keysym::Super_L.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_L*", Keysym::Super_L.raw())?;}
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_L*", Keysym::Super_L.raw())?;}
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Shift_L", Keysym::Shift_L.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Shift_L*", Keysym::Shift_L.raw())?;}
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::M", Keysym::M.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::N", Keysym::N.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::O", Keysym::O.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::P", Keysym::P.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Page_Down", Keysym::Page_Down.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Page_Up", Keysym::Page_Up.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Pause", Keysym::Pause.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Print", Keysym::Print.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Q", Keysym::Q.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::R", Keysym::R.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Return", Keysym::Return.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Alt_R", Keysym::Alt_R.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Alt_R*", Keysym::Alt_R.raw())?;}
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Right", Keysym::Right.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Control_R", Keysym::Control_R.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_R", Keysym::Super_R.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Super_R*", Keysym::Super_R.raw())?;}
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Shift_R", Keysym::Shift_R.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::S", Keysym::S.raw())?;
  if any {writeln!(w, "{:30}, 0x{:04X},", "Keysym::Scroll_Lock*", Keysym::Scroll_Lock.raw())?;}
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Scroll_Lock", Keysym::Scroll_Lock.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Select", Keysym::Select.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::space", Keysym::space.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::T", Keysym::T.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Tab", Keysym::Tab.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::U", Keysym::U.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Up", Keysym::Up.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::V", Keysym::V.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::W", Keysym::W.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::X", Keysym::X.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Y", Keysym::Y.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Z", Keysym::Z.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_0", Keysym::KP_0.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_1", Keysym::KP_1.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_2", Keysym::KP_2.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_3", Keysym::KP_3.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_4", Keysym::KP_4.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_5", Keysym::KP_5.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_6", Keysym::KP_6.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_7", Keysym::KP_7.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_8", Keysym::KP_8.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_9", Keysym::KP_9.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_Subtract", Keysym::KP_Subtract.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_Divide", Keysym::KP_Divide.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Num_Lock", Keysym::Num_Lock.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_Decimal", Keysym::KP_Decimal.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_Add", Keysym::KP_Add.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::KP_Multiply", Keysym::KP_Multiply.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F25", Keysym::F25.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F26", Keysym::F26.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F27", Keysym::F27.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F28", Keysym::F28.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F29", Keysym::F29.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F30", Keysym::F30.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F31", Keysym::F31.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F32", Keysym::F32.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F33", Keysym::F33.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F34", Keysym::F34.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::F35", Keysym::F35.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Begin", Keysym::Begin.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Break", Keysym::Break.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Find", Keysym::Find.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Linefeed", Keysym::Linefeed.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::XF86_AudioMicMute", Keysym::XF86_AudioMicMute.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Redo", Keysym::Redo.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::script_switch", Keysym::script_switch.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Shift_Lock", Keysym::Shift_Lock.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Sys_Req", Keysym::Sys_Req.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Undo", Keysym::Undo.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Hangul", Keysym::Hangul.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Kanji", Keysym::Kanji.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Hangul_Hanja", Keysym::Hangul_Hanja.raw())?;
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Mode_switch", Keysym::Mode_switch.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
          writeln!(w, "{:30}, 0x{:04X},", "Keysym::Print", Keysym::Print.raw())?;
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:30}, {},", "None", "    ")?; }
  Ok(())
}


fn print_macos(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {
  use vkey::mirror::{macos::KeyCode, macos_ext::{CGKeyCode, KeyCodeExt}};
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::MUTE", CGKeyCode::from(KeyCode::MUTE).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::VOLUME_DOWN", CGKeyCode::from(KeyCode::VOLUME_DOWN).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::VOLUME_UP", CGKeyCode::from(KeyCode::VOLUME_UP).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_0", CGKeyCode::from(KeyCodeExt::kVK_ANSI_0).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_1", CGKeyCode::from(KeyCodeExt::kVK_ANSI_1).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_2", CGKeyCode::from(KeyCodeExt::kVK_ANSI_2).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_3", CGKeyCode::from(KeyCodeExt::kVK_ANSI_3).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_4", CGKeyCode::from(KeyCodeExt::kVK_ANSI_4).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_5", CGKeyCode::from(KeyCodeExt::kVK_ANSI_5).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_6", CGKeyCode::from(KeyCodeExt::kVK_ANSI_6).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_7", CGKeyCode::from(KeyCodeExt::kVK_ANSI_7).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_8", CGKeyCode::from(KeyCodeExt::kVK_ANSI_8).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_9", CGKeyCode::from(KeyCodeExt::kVK_ANSI_9).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_A", CGKeyCode::from(KeyCodeExt::kVK_ANSI_A).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_B", CGKeyCode::from(KeyCodeExt::kVK_ANSI_B).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_C", CGKeyCode::from(KeyCodeExt::kVK_ANSI_C).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::CAPS_LOCK", CGKeyCode::from(KeyCode::CAPS_LOCK).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_D", CGKeyCode::from(KeyCodeExt::kVK_ANSI_D).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::DELETE", CGKeyCode::from(KeyCode::DELETE).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::FORWARD_DELETE", CGKeyCode::from(KeyCode::FORWARD_DELETE).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::DOWN_ARROW", CGKeyCode::from(KeyCode::DOWN_ARROW).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_E", CGKeyCode::from(KeyCodeExt::kVK_ANSI_E).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::END", CGKeyCode::from(KeyCode::END).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::ESCAPE", CGKeyCode::from(KeyCode::ESCAPE).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_F", CGKeyCode::from(KeyCodeExt::kVK_ANSI_F).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F1", CGKeyCode::from(KeyCode::F1).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F10", CGKeyCode::from(KeyCode::F10).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F11", CGKeyCode::from(KeyCode::F11).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F12", CGKeyCode::from(KeyCode::F12).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F13", CGKeyCode::from(KeyCode::F13).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F14", CGKeyCode::from(KeyCode::F14).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F15", CGKeyCode::from(KeyCode::F15).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F16", CGKeyCode::from(KeyCode::F16).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F17", CGKeyCode::from(KeyCode::F17).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F18", CGKeyCode::from(KeyCode::F18).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F19", CGKeyCode::from(KeyCode::F19).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F2", CGKeyCode::from(KeyCode::F2).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F20", CGKeyCode::from(KeyCode::F20).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F3", CGKeyCode::from(KeyCode::F3).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F4", CGKeyCode::from(KeyCode::F4).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F5", CGKeyCode::from(KeyCode::F5).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F6", CGKeyCode::from(KeyCode::F6).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F7", CGKeyCode::from(KeyCode::F7).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F8", CGKeyCode::from(KeyCode::F8).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::F9", CGKeyCode::from(KeyCode::F9).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_G", CGKeyCode::from(KeyCodeExt::kVK_ANSI_G).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_H", CGKeyCode::from(KeyCodeExt::kVK_ANSI_H).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::HELP", CGKeyCode::from(KeyCode::HELP).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::HOME", CGKeyCode::from(KeyCode::HOME).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_I", CGKeyCode::from(KeyCodeExt::kVK_ANSI_I).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_J", CGKeyCode::from(KeyCodeExt::kVK_ANSI_J).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_K", CGKeyCode::from(KeyCodeExt::kVK_ANSI_K).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_L", CGKeyCode::from(KeyCodeExt::kVK_ANSI_L).0)?;
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::OPTION*", CGKeyCode::from(KeyCode::OPTION).0)?;}
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::OPTION*", CGKeyCode::from(KeyCode::OPTION).0)?;}
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::OPTION", CGKeyCode::from(KeyCode::OPTION).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::LEFT_ARROW", CGKeyCode::from(KeyCode::LEFT_ARROW).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::CONTROL", CGKeyCode::from(KeyCode::CONTROL).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::CONTROL", CGKeyCode::from(KeyCode::CONTROL).0)?;
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::COMMAND*", CGKeyCode::from(KeyCode::COMMAND).0)?;}
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::COMMAND*", CGKeyCode::from(KeyCode::COMMAND).0)?;}
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::COMMAND", CGKeyCode::from(KeyCode::COMMAND).0)?;
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::COMMAND*", CGKeyCode::from(KeyCode::COMMAND).0)?;}
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::COMMAND*", CGKeyCode::from(KeyCode::COMMAND).0)?;}
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::SHIFT*", CGKeyCode::from(KeyCode::SHIFT).0)?;}
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::SHIFT", CGKeyCode::from(KeyCode::SHIFT).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_M", CGKeyCode::from(KeyCodeExt::kVK_ANSI_M).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_N", CGKeyCode::from(KeyCodeExt::kVK_ANSI_N).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_O", CGKeyCode::from(KeyCodeExt::kVK_ANSI_O).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_P", CGKeyCode::from(KeyCodeExt::kVK_ANSI_P).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::PAGE_DOWN", CGKeyCode::from(KeyCode::PAGE_DOWN).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::PAGE_UP", CGKeyCode::from(KeyCode::PAGE_UP).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Q", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Q).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_R", CGKeyCode::from(KeyCodeExt::kVK_ANSI_R).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RETURN", CGKeyCode::from(KeyCode::RETURN).0)?;
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_OPTION*", CGKeyCode::from(KeyCode::RIGHT_OPTION).0)?;}
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_OPTION", CGKeyCode::from(KeyCode::RIGHT_OPTION).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_ARROW", CGKeyCode::from(KeyCode::RIGHT_ARROW).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_CONTROL", CGKeyCode::from(KeyCode::RIGHT_CONTROL).0)?;
  if any {writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_COMMAND*", CGKeyCode::from(KeyCode::RIGHT_COMMAND).0)?;}
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_COMMAND", CGKeyCode::from(KeyCode::RIGHT_COMMAND).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::RIGHT_SHIFT", CGKeyCode::from(KeyCode::RIGHT_SHIFT).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_S", CGKeyCode::from(KeyCodeExt::kVK_ANSI_S).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::SPACE", CGKeyCode::from(KeyCode::SPACE).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_T", CGKeyCode::from(KeyCodeExt::kVK_ANSI_T).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::TAB", CGKeyCode::from(KeyCode::TAB).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_U", CGKeyCode::from(KeyCodeExt::kVK_ANSI_U).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::UP_ARROW", CGKeyCode::from(KeyCode::UP_ARROW).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_V", CGKeyCode::from(KeyCodeExt::kVK_ANSI_V).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_W", CGKeyCode::from(KeyCodeExt::kVK_ANSI_W).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_X", CGKeyCode::from(KeyCodeExt::kVK_ANSI_X).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Y", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Y).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Z", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Z).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad0", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad0).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad1", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad1).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad2", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad2).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad3", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad3).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad4", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad4).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad5", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad5).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad6", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad6).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad7", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad7).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad8", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad8).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_Keypad9", CGKeyCode::from(KeyCodeExt::kVK_ANSI_Keypad9).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_KeypadMinus", CGKeyCode::from(KeyCodeExt::kVK_ANSI_KeypadMinus).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_KeypadDivide", CGKeyCode::from(KeyCodeExt::kVK_ANSI_KeypadDivide).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_KeypadDecimal", CGKeyCode::from(KeyCodeExt::kVK_ANSI_KeypadDecimal).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_KeypadPlus", CGKeyCode::from(KeyCodeExt::kVK_ANSI_KeypadPlus).0)?;
          writeln!(w, "{:36}, 0x{:02X},", "KeyCodeExt::kVK_ANSI_KeypadMultiply", CGKeyCode::from(KeyCodeExt::kVK_ANSI_KeypadMultiply).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "KeyCode::FUNCTION", CGKeyCode::from(KeyCode::FUNCTION).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "CGKeyCode(131).0", CGKeyCode::from(CGKeyCode(131).0).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
          writeln!(w, "{:36}, 0x{:02X},", "CGKeyCode(160).0", CGKeyCode::from(CGKeyCode(160).0).0)?;
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:36}, {},", "None", "    ")?; }
  Ok(())
}


#[cfg(windows)]
fn print_win_makecode1(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {
  use vkey::deps::windows::{MapVirtualKeyExW, MAPVK_VK_TO_VSC_EX};
  let makecode = |vkey| unsafe { MapVirtualKeyExW(vkey, MAPVK_VK_TO_VSC_EX, None) };
          writeln!(w, "{:5}, 0x{:02X},", "0x01", makecode(0x01))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x02", makecode(0x02))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x04", makecode(0x04))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x05", makecode(0x05))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x06", makecode(0x06))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA6", makecode(0xA6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xAB", makecode(0xAB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA7", makecode(0xA7))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xAC", makecode(0xAC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA8", makecode(0xA8))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xAA", makecode(0xAA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA9", makecode(0xA9))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB4", makecode(0xB4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB5", makecode(0xB5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xAD", makecode(0xAD))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB3", makecode(0xB3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB0", makecode(0xB0))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB1", makecode(0xB1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB2", makecode(0xB2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xAE", makecode(0xAE))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xAF", makecode(0xAF))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5F", makecode(0x5F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x30", makecode(0x30))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x31", makecode(0x31))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x32", makecode(0x32))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x33", makecode(0x33))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x34", makecode(0x34))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x35", makecode(0x35))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x36", makecode(0x36))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x37", makecode(0x37))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x38", makecode(0x38))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x39", makecode(0x39))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x41", makecode(0x41))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5D", makecode(0x5D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x42", makecode(0x42))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xDC", makecode(0xDC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x43", makecode(0x43))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x03", makecode(0x03))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x14", makecode(0x14))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x0C", makecode(0x0C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xBC", makecode(0xBC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x44", makecode(0x44))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xBD", makecode(0xBD))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x08", makecode(0x08))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2E", makecode(0x2E))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x28", makecode(0x28))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x45", makecode(0x45))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x23", makecode(0x23))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xBB", makecode(0xBB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x1B", makecode(0x1B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2B", makecode(0x2B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x46", makecode(0x46))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x70", makecode(0x70))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x79", makecode(0x79))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x7A", makecode(0x7A))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x7B", makecode(0x7B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x7C", makecode(0x7C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x7D", makecode(0x7D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x7E", makecode(0x7E))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x7F", makecode(0x7F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x80", makecode(0x80))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x81", makecode(0x81))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x82", makecode(0x82))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x71", makecode(0x71))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x83", makecode(0x83))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x84", makecode(0x84))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x85", makecode(0x85))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x86", makecode(0x86))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x87", makecode(0x87))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x72", makecode(0x72))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x73", makecode(0x73))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x74", makecode(0x74))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x75", makecode(0x75))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x76", makecode(0x76))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x77", makecode(0x77))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x78", makecode(0x78))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xBF", makecode(0xBF))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x47", makecode(0x47))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC0", makecode(0xC0))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x48", makecode(0x48))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2F", makecode(0x2F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x24", makecode(0x24))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x49", makecode(0x49))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2D", makecode(0x2D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x4A", makecode(0x4A))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x4B", makecode(0x4B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x4C", makecode(0x4C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA4", makecode(0xA4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x12", makecode(0x12))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x12", makecode(0x12))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xDE", makecode(0xDE))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x25", makecode(0x25))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xDB", makecode(0xDB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x11", makecode(0x11))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA2", makecode(0xA2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5B", makecode(0x5B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5B", makecode(0x5B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5B", makecode(0x5B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5B", makecode(0x5B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5B", makecode(0x5B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA0", makecode(0xA0))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x10", makecode(0x10))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x4D", makecode(0x4D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x4E", makecode(0x4E))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE2", makecode(0xE2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x4F", makecode(0x4F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x50", makecode(0x50))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x22", makecode(0x22))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x21", makecode(0x21))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x13", makecode(0x13))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xBE", makecode(0xBE))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2C", makecode(0x2C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2C", makecode(0x2C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x51", makecode(0x51))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x52", makecode(0x52))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x0D", makecode(0x0D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA5", makecode(0xA5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA5", makecode(0xA5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x27", makecode(0x27))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xDD", makecode(0xDD))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA3", makecode(0xA3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5C", makecode(0x5C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5C", makecode(0x5C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xA1", makecode(0xA1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x53", makecode(0x53))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x91", makecode(0x91))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x91", makecode(0x91))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x29", makecode(0x29))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xBA", makecode(0xBA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x20", makecode(0x20))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x54", makecode(0x54))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x09", makecode(0x09))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x55", makecode(0x55))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x26", makecode(0x26))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x56", makecode(0x56))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x57", makecode(0x57))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x58", makecode(0x58))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x59", makecode(0x59))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x5A", makecode(0x5A))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x60", makecode(0x60))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x61", makecode(0x61))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x62", makecode(0x62))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x63", makecode(0x63))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x64", makecode(0x64))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x65", makecode(0x65))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x66", makecode(0x66))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x67", makecode(0x67))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x68", makecode(0x68))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x69", makecode(0x69))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x6D", makecode(0x6D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x6F", makecode(0x6F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x90", makecode(0x90))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x6E", makecode(0x6E))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x6B", makecode(0x6B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x6A", makecode(0x6A))?;
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
          writeln!(w, "{:5}, 0x{:02X},", "0xFF", makecode(0xFF))?;
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
  if any { writeln!(w, "{:5}, {},", "None", "    ")?; }
          writeln!(w, "{:5}, 0x{:02X},", "0xC1", makecode(0xC1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC2", makecode(0xC2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF6", makecode(0xF6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF7", makecode(0xF7))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF0", makecode(0xF0))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFA", makecode(0xFA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF4", makecode(0xF4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFC", makecode(0xFC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFD", makecode(0xFD))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF8", makecode(0xF8))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF7", makecode(0xF7))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF9", makecode(0xF9))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF2", makecode(0xF2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF1", makecode(0xF1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFB", makecode(0xFB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF6", makecode(0xF6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF5", makecode(0xF5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF3", makecode(0xF3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF9", makecode(0xF9))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF8", makecode(0xF8))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC3", makecode(0xC3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC4", makecode(0xC4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xCC", makecode(0xCC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xCD", makecode(0xCD))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xCE", makecode(0xCE))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xCB", makecode(0xCB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC8", makecode(0xC8))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD1", makecode(0xD1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD4", makecode(0xD4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD6", makecode(0xD6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD5", makecode(0xD5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD3", makecode(0xD3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC9", makecode(0xC9))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xCF", makecode(0xCF))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC7", makecode(0xC7))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD2", makecode(0xD2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD8", makecode(0xD8))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xDA", makecode(0xDA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD9", makecode(0xD9))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD7", makecode(0xD7))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xCA", makecode(0xCA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xD0", makecode(0xD0))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC5", makecode(0xC5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xC6", makecode(0xC6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE4", makecode(0xE4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE6", makecode(0xE6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE3", makecode(0xE3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x8E", makecode(0x8E))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x8F", makecode(0x8F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x8B", makecode(0x8B))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x8C", makecode(0x8C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x89", makecode(0x89))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x8D", makecode(0x8D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x8A", makecode(0x8A))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x88", makecode(0x88))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFC", makecode(0xFC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xDF", makecode(0xDF))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF0", makecode(0xF0))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF3", makecode(0xF3))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE1", makecode(0xE1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF5", makecode(0xF5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFE", makecode(0xFE))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF2", makecode(0xF2))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xEF", makecode(0xEF))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF4", makecode(0xF4))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xF1", makecode(0xF1))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x92", makecode(0x92))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x95", makecode(0x95))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x93", makecode(0x93))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x96", makecode(0x96))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x94", makecode(0x94))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xEA", makecode(0xEA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x92", makecode(0x92))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xEB", makecode(0xEB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xEC", makecode(0xEC))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xED", makecode(0xED))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE9", makecode(0xE9))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xEE", makecode(0xEE))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFD", makecode(0xFD))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE7", makecode(0xE7))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFA", makecode(0xFA))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xE5", makecode(0xE5))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xFB", makecode(0xFB))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x1E", makecode(0x1E))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x1C", makecode(0x1C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x18", makecode(0x18))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x1A", makecode(0x1A))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x16", makecode(0x16))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x17", makecode(0x17))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x15", makecode(0x15))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x15", makecode(0x15))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x15", makecode(0x15))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x19", makecode(0x19))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x19", makecode(0x19))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x1F", makecode(0x1F))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x1D", makecode(0x1D))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x2A", makecode(0x2A))?;
          writeln!(w, "{:5}, 0x{:02X},", "0x6C", makecode(0x6C))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB6", makecode(0xB6))?;
          writeln!(w, "{:5}, 0x{:02X},", "0xB7", makecode(0xB7))?;
  Ok(())
}

