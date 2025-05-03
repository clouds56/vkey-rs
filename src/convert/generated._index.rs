mod generated_winput_to_hut {
  #[cfg(all(mirror_winput_vk, dep_hut_04))]
  mod mirror_to_dep_1_1 {
    use crate::mirror::winput::Vk;
    use crate::deps::hut_04::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.Winput_to_HUT.rs");
  }

  #[cfg(all(mirror_winput_vk, dep_hut_03))]
  mod mirror_to_dep_1_2 {
    use crate::mirror::winput::Vk;
    use crate::deps::hut_03::{AsUsage, Button, Consumer, GenericDesktop, KeyboardKeypad, Usage};
    include!("generated.Winput_to_HUT.rs");
  }
}

mod generated_winput_to_enigomirror {
  #[cfg(all(mirror_winput_vk, mirror_enigo))]
  mod mirror_to_mirror_1_1 {
    use crate::mirror::winput::Vk;
    use crate::mirror::enigo::Key as Enigo;
    include!("generated.Winput_to_EnigoMirror.rs");
  }
}

mod generated_winput_to_enigodep {
  #[cfg(all(mirror_winput_vk, dep_enigo))]
  mod mirror_to_dep_1_1 {
    use crate::mirror::winput::Vk;
    use crate::deps::enigo::Key as Enigo;
    include!("generated.Winput_to_EnigoDep.rs");
  }
}

mod generated_enigomirror_to_winput {
  #[cfg(all(mirror_enigo, mirror_winput_vk))]
  mod mirror_to_mirror_1_1 {
    use crate::mirror::enigo::Key as Enigo;
    use crate::mirror::winput::Vk;
    include!("generated.EnigoMirror_to_Winput.rs");
  }
}

mod generated_enigodep_to_winput {
  #[cfg(all(dep_enigo, mirror_winput_vk))]
  mod dep_to_mirror_1_1 {
    use crate::deps::enigo::Key as Enigo;
    use crate::mirror::winput::Vk;
    include!("generated.EnigoDep_to_Winput.rs");
  }
}

mod generated_enigomirror_to_winvk {
  #[cfg(all(mirror_enigo, mirror_windows_vk))]
  mod mirror_to_mirror_1_1 {
    use crate::mirror::enigo::Key as Enigo;
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY};
    include!("generated.EnigoMirror_to_WinVk.rs");
  }
  #[cfg(all(mirror_enigo, dep_windows_vk))]
  mod mirror_to_dep_1_1 {
    use crate::mirror::enigo::Key as Enigo;
    use crate::deps::windows::{self as keys, VIRTUAL_KEY};
    include!("generated.EnigoMirror_to_WinVk.rs");
  }
}

mod generated_enigodep_to_winvk {
  #[cfg(all(dep_enigo, mirror_windows_vk))]
  mod dep_to_mirror_1_1 {
    use crate::deps::enigo::Key as Enigo;
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY};
    include!("generated.EnigoDep_to_WinVk.rs");
  }
  #[cfg(all(dep_enigo, dep_windows_vk))]
  mod dep_to_dep_1_1 {
    use crate::deps::enigo::Key as Enigo;
    use crate::deps::windows::{self as keys, VIRTUAL_KEY};
    include!("generated.EnigoDep_to_WinVk.rs");
  }
}

mod generated_enigomirror_to_keysym {
  #[cfg(all(mirror_enigo, dep_xkeysym))]
  mod mirror_to_dep_1_1 {
    use crate::mirror::enigo::Key as Enigo;
    use crate::deps::xkeysym::Keysym;
    include!("generated.EnigoMirror_to_Keysym.rs");
  }
}

mod generated_enigodep_to_keysym {
  #[cfg(all(dep_enigo, dep_xkeysym))]
  mod dep_to_dep_1_1 {
    use crate::deps::enigo::Key as Enigo;
    use crate::deps::xkeysym::Keysym;
    include!("generated.EnigoDep_to_Keysym.rs");
  }
}

mod generated_enigomirror_to_cg {
  #[cfg(all(mirror_enigo, any(dep_macos, mirror_macos)))]
  mod mirror_to_mirror_1_1 {
    use crate::mirror::enigo::Key as Enigo;
    #[cfg(dep_macos)]
    use crate::deps::macos::KeyCode;
    #[cfg(not(dep_macos))]
    #[cfg(mirror_macos)]
    use crate::mirror::macos::KeyCode;
    use crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};
    include!("generated.EnigoMirror_to_CG.rs");
  }
}

mod generated_enigodep_to_cg {
  #[cfg(all(dep_enigo, any(dep_macos, mirror_macos)))]
  mod dep_to_mirror_1_1 {
    use crate::deps::enigo::Key as Enigo;
    #[cfg(dep_macos)]
    use crate::deps::macos::KeyCode;
    #[cfg(not(dep_macos))]
    #[cfg(mirror_macos)]
    use crate::mirror::macos::KeyCode;
    use crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};
    include!("generated.EnigoDep_to_CG.rs");
  }
}

mod generated_winput_to_cg {
  #[cfg(all(mirror_winput_vk, any(dep_macos, mirror_macos)))]
  mod mirror_to_mirror_1_1 {
    use crate::mirror::winput::Vk;
    #[cfg(dep_macos)]
    use crate::deps::macos::KeyCode;
    #[cfg(not(dep_macos))]
    #[cfg(mirror_macos)]
    use crate::mirror::macos::KeyCode;
    use crate::mirror::macos_ext::{CGKeyCode, KeyCodeExt};
    include!("generated.Winput_to_CG.rs");
  }
}

mod generated_winput_to_keysym {
  #[cfg(all(mirror_winput_vk, dep_xkeysym))]
  mod mirror_to_dep_1_1 {
    use crate::mirror::winput::Vk;
    use crate::deps::xkeysym::Keysym;
    include!("generated.Winput_to_Keysym.rs");
  }
}

mod generated_winput_to_winscan {
  #[cfg(all(mirror_winput_vk, dep_make1))]
  mod mirror_to_dep_1_1 {
    use crate::mirror::winput::Vk;
    use crate::mirror::make1::Make1Code;
    include!("generated.Winput_to_WinScan.rs");
  }
}

