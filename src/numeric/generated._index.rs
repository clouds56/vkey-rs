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
    use crate::mirror::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.WinVk.rs");
  }
  #[cfg(dep_windows_vk)]
  mod dep_1 {
    use crate::deps::windows::{self as keys, VIRTUAL_KEY as Vk};
    include!("generated.WinVk.rs");
  }
}

