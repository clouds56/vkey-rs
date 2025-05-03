#[allow(unused_imports)]
mod generated_hut {
  #[cfg(dep_hut_04)]
  mod dep_1 {
    use crate::deps::hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.HUT.rs");
  }

  #[cfg(dep_hut_03)]
  mod dep_2 {
    use crate::deps::hut_03::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.HUT.rs");
  }
}

#[allow(unused_imports)]
mod generated_winput {
  #[cfg(mirror_winput_vk)]
  mod mirror_1 {
    use crate::mirror::winput::Vk;
    include!("generated.Winput.rs");
  }
}

#[allow(unused_imports)]
mod generated_winvk {
  #[cfg(mirror_windows_vk)]
  mod mirror_1 {
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY};
    include!("generated.WinVk.rs");
  }
  #[cfg(dep_windows_vk)]
  mod dep_1 {
    use crate::deps::windows::{self as keys, VIRTUAL_KEY};
    include!("generated.WinVk.rs");
  }
}

#[allow(unused_imports)]
mod generated_winscan {
  #[cfg(dep_make1)]
  mod dep_1 {
    use crate::mirror::make1::Make1Code;
    include!("generated.WinScan.rs");
  }
}

#[allow(unused_imports)]
mod generated_keysym {
  #[cfg(dep_xkeysym)]
  mod dep_1 {
    use crate::deps::xkeysym::Keysym;
    include!("generated.Keysym.rs");
  }
}

#[allow(unused_imports)]
mod generated_cg {
  #[cfg(any(dep_macos, mirror_macos))]
  mod mirror_1 {
    #[cfg(dep_macos)]
    use crate::deps::macos::KeyCode;
    #[cfg(not(dep_macos))]
    #[cfg(mirror_macos)]
    use crate::mirror::macos::KeyCode;
    use crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};
    include!("generated.CG.rs");
  }
}

