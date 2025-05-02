fn main() {
  let path = &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/cache/numeric");
  let file = |name: &str| std::fs::File::create(path.join(name)).unwrap();
  std::fs::create_dir_all(path).unwrap();
  println!("============== HUT 04 ===============");
  print_hut_04(&mut file("hut_04.txt"), true).ok();
  print_hut_04(&mut std::io::stdout(), false).ok();
}


fn print_hut_04(w: &mut dyn std::io::Write, any: bool) -> std::io::Result<()> {
  use hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad};
          writeln!(w, "{:52}, 0x{:X}", "Button::Button(1)", Button::Button(1).usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Button::Button(2)", Button::Button(2).usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Button::Button(3)", Button::Button(3).usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Button::Button(4)", Button::Button(4).usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Button::Button(5)", Button::Button(5).usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACBack", Consumer::ACBack.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACBookmarks", Consumer::ACBookmarks.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACForward", Consumer::ACForward.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACHome", Consumer::ACHome.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACRefresh", Consumer::ACRefresh.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACSearch", Consumer::ACSearch.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ACStop", Consumer::ACStop.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ALEmailReader", Consumer::ALEmailReader.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::MediaSelection", Consumer::MediaSelection.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::Mute", Consumer::Mute.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::PlayPause", Consumer::PlayPause.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ScanNextTrack", Consumer::ScanNextTrack.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::ScanPreviousTrack", Consumer::ScanPreviousTrack.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::Stop", Consumer::Stop.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::VolumeDecrement", Consumer::VolumeDecrement.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "Consumer::VolumeIncrement", Consumer::VolumeIncrement.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "GenericDesktop::SystemSleep", GenericDesktop::SystemSleep.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard0andRightBracket", KeyboardKeypad::Keyboard0andRightBracket.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard1andBang", KeyboardKeypad::Keyboard1andBang.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard2andAt", KeyboardKeypad::Keyboard2andAt.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard3andHash", KeyboardKeypad::Keyboard3andHash.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard4andDollar", KeyboardKeypad::Keyboard4andDollar.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard5andPercent", KeyboardKeypad::Keyboard5andPercent.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard6andCaret", KeyboardKeypad::Keyboard6andCaret.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard7andAmpersand", KeyboardKeypad::Keyboard7andAmpersand.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard8andStar", KeyboardKeypad::Keyboard8andStar.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keyboard9andLeftBracket", KeyboardKeypad::Keyboard9andLeftBracket.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardA", KeyboardKeypad::KeyboardA.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardApplication", KeyboardKeypad::KeyboardApplication.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardB", KeyboardKeypad::KeyboardB.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardBackslashandPipe", KeyboardKeypad::KeyboardBackslashandPipe.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardC", KeyboardKeypad::KeyboardC.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardCancel", KeyboardKeypad::KeyboardCancel.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardCapsLock", KeyboardKeypad::KeyboardCapsLock.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardClear", KeyboardKeypad::KeyboardClear.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardCommaandLessThan", KeyboardKeypad::KeyboardCommaandLessThan.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardD", KeyboardKeypad::KeyboardD.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardDashandUnderscore", KeyboardKeypad::KeyboardDashandUnderscore.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardDelete", KeyboardKeypad::KeyboardDelete.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardDeleteForward", KeyboardKeypad::KeyboardDeleteForward.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardDownArrow", KeyboardKeypad::KeyboardDownArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardE", KeyboardKeypad::KeyboardE.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardEnd", KeyboardKeypad::KeyboardEnd.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardEqualsandPlus", KeyboardKeypad::KeyboardEqualsandPlus.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardEscape", KeyboardKeypad::KeyboardEscape.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardExecute", KeyboardKeypad::KeyboardExecute.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF", KeyboardKeypad::KeyboardF.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF1", KeyboardKeypad::KeyboardF1.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF10", KeyboardKeypad::KeyboardF10.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF11", KeyboardKeypad::KeyboardF11.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF12", KeyboardKeypad::KeyboardF12.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF13", KeyboardKeypad::KeyboardF13.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF14", KeyboardKeypad::KeyboardF14.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF15", KeyboardKeypad::KeyboardF15.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF16", KeyboardKeypad::KeyboardF16.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF17", KeyboardKeypad::KeyboardF17.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF18", KeyboardKeypad::KeyboardF18.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF19", KeyboardKeypad::KeyboardF19.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF2", KeyboardKeypad::KeyboardF2.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF20", KeyboardKeypad::KeyboardF20.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF21", KeyboardKeypad::KeyboardF21.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF22", KeyboardKeypad::KeyboardF22.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF23", KeyboardKeypad::KeyboardF23.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF24", KeyboardKeypad::KeyboardF24.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF3", KeyboardKeypad::KeyboardF3.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF4", KeyboardKeypad::KeyboardF4.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF5", KeyboardKeypad::KeyboardF5.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF6", KeyboardKeypad::KeyboardF6.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF7", KeyboardKeypad::KeyboardF7.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF8", KeyboardKeypad::KeyboardF8.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardF9", KeyboardKeypad::KeyboardF9.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardForwardSlashandQuestionMark", KeyboardKeypad::KeyboardForwardSlashandQuestionMark.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardG", KeyboardKeypad::KeyboardG.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardGraveAccentandTilde", KeyboardKeypad::KeyboardGraveAccentandTilde.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardH", KeyboardKeypad::KeyboardH.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardHelp", KeyboardKeypad::KeyboardHelp.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardHome", KeyboardKeypad::KeyboardHome.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardI", KeyboardKeypad::KeyboardI.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardInsert", KeyboardKeypad::KeyboardInsert.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardJ", KeyboardKeypad::KeyboardJ.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardK", KeyboardKeypad::KeyboardK.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardL", KeyboardKeypad::KeyboardL.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftAlt", KeyboardKeypad::KeyboardLeftAlt.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftAlt*", KeyboardKeypad::KeyboardLeftAlt.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftAlt*", KeyboardKeypad::KeyboardLeftAlt.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftAposandDouble", KeyboardKeypad::KeyboardLeftAposandDouble.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftArrow", KeyboardKeypad::KeyboardLeftArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftBrace", KeyboardKeypad::KeyboardLeftBrace.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftControl", KeyboardKeypad::KeyboardLeftControl.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftControl", KeyboardKeypad::KeyboardLeftControl.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftGUI", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftGUI*", KeyboardKeypad::KeyboardLeftGUI.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftShift", KeyboardKeypad::KeyboardLeftShift.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardLeftShift*", KeyboardKeypad::KeyboardLeftShift.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardM", KeyboardKeypad::KeyboardM.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardN", KeyboardKeypad::KeyboardN.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardNonUSBackslashandPipe", KeyboardKeypad::KeyboardNonUSBackslashandPipe.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardO", KeyboardKeypad::KeyboardO.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardP", KeyboardKeypad::KeyboardP.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardPageDown", KeyboardKeypad::KeyboardPageDown.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardPageUp", KeyboardKeypad::KeyboardPageUp.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardPause", KeyboardKeypad::KeyboardPause.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardPeriodandGreaterThan", KeyboardKeypad::KeyboardPeriodandGreaterThan.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardPrintScreen", KeyboardKeypad::KeyboardPrintScreen.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardPrintScreen*", KeyboardKeypad::KeyboardPrintScreen.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardQ", KeyboardKeypad::KeyboardQ.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardR", KeyboardKeypad::KeyboardR.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardReturnEnter", KeyboardKeypad::KeyboardReturnEnter.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardRightAlt", KeyboardKeypad::KeyboardRightAlt.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardRightArrow", KeyboardKeypad::KeyboardRightArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardRightBrace", KeyboardKeypad::KeyboardRightBrace.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardRightControl", KeyboardKeypad::KeyboardRightControl.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardRightGUI", KeyboardKeypad::KeyboardRightGUI.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardRightShift", KeyboardKeypad::KeyboardRightShift.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardS", KeyboardKeypad::KeyboardS.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardScrollLock", KeyboardKeypad::KeyboardScrollLock.usage_value())?;
  if any {writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardScrollLock*", KeyboardKeypad::KeyboardScrollLock.usage_value())?;}
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardSelect", KeyboardKeypad::KeyboardSelect.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardSemiColonandColon", KeyboardKeypad::KeyboardSemiColonandColon.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardSpacebar", KeyboardKeypad::KeyboardSpacebar.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardT", KeyboardKeypad::KeyboardT.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardTab", KeyboardKeypad::KeyboardTab.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardU", KeyboardKeypad::KeyboardU.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardUpArrow", KeyboardKeypad::KeyboardUpArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardV", KeyboardKeypad::KeyboardV.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardW", KeyboardKeypad::KeyboardW.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardX", KeyboardKeypad::KeyboardX.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardY", KeyboardKeypad::KeyboardY.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeyboardZ", KeyboardKeypad::KeyboardZ.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad0andInsert", KeyboardKeypad::Keypad0andInsert.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad1andEnd", KeyboardKeypad::Keypad1andEnd.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad2andDownArrow", KeyboardKeypad::Keypad2andDownArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad3andPageDn", KeyboardKeypad::Keypad3andPageDn.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad4andLeftArrow", KeyboardKeypad::Keypad4andLeftArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad5", KeyboardKeypad::Keypad5.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad6andRightArrow", KeyboardKeypad::Keypad6andRightArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad7andHome", KeyboardKeypad::Keypad7andHome.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad8andUpArrow", KeyboardKeypad::Keypad8andUpArrow.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::Keypad9andPageUp", KeyboardKeypad::Keypad9andPageUp.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeypadDash", KeyboardKeypad::KeypadDash.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeypadForwardSlash", KeyboardKeypad::KeypadForwardSlash.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeypadNumLockandClear", KeyboardKeypad::KeypadNumLockandClear.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeypadPeriodandDelete", KeyboardKeypad::KeypadPeriodandDelete.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeypadPlus", KeyboardKeypad::KeypadPlus.usage_value())?;
          writeln!(w, "{:52}, 0x{:X}", "KeyboardKeypad::KeypadStar", KeyboardKeypad::KeypadStar.usage_value())?;
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  if any { writeln!(w, "{:52}, {}", "None", "n!()")?; }
  Ok(())
}
