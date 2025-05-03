# VKey
[![Crates.io Version](https://img.shields.io/crates/v/vkey)](https://crates.io/crates/vkey)
[![docsrs](https://img.shields.io/docsrs/vkey)](https://docs.rs/vkey/latest/vkey/)
[![Actions](https://github.com/clouds56/vkey-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/clouds56/vkey-rs/actions/workflows/rust.yml)

This crates only contains various types of virtual keycodes, and conversions between them.

This crate is under heavy development, and subject to breaking changes.

- [x] [`winput::Vk`](https://docs.rs/winput/latest/winput/enum.Vk.html) from [`winput`](https://github.com/gymore-io/winput)
- [x] [`VIRTUAL_KEY`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/struct.VIRTUAL_KEY.html) from [`windows`](https://github.com/microsoft/windows-rs) (or [mirrored](https://docs.rs/vkey/latest/vkey/mirror/winput/enum.Vk.html))
- [x] Wrapped [`Make1Scan`](https://docs.rs/vkey/latest/vkey/mirror/make1/struct.Make1Code.html) for [`MapVirtualKeyW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapvirtualkeyw)
- [x] [`enigo::Key`](https://docs.rs/enigo/latest/enigo/enum.Key.html) from [`enigo`](https://github.com/enigo-rs/enigo)
- [x] [`core_graphics::event::KeyCode`](https://docs.rs/core-graphics/0.24.0/core_graphics/event/struct.KeyCode.html) from [macos](https://github.com/servo/core-foundation-rs) (or [wrapped](https://docs.rs/vkey/latest/vkey/mirror/macos_ext/struct.CGKeyCode.html))
- [x] [`Usage`](https://docs.rs/hut/latest/hut/enum.Usage.html) from [`hut`](https://github.com/hidutils/hut) (both hut_03 and hut_04 supported)
- [ ] [`winit::KeyCode`](https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html) from [`winit`](https://github.com/rust-windowing/winit) (only definitions)
    * TODO: support for `winit::NamedKey`, `winit::NativeKey`, `winit::NativeKeyCode` and `winit::PhysicalKey`
- [x] [`xkeysym::Keysym`](https://docs.rs/xkeysym/0.2.1/xkeysym/struct.Keysym.html) from [`xkeysym`](https://github.com/rust-windowing/xkeysym)
    * TODO: support for `xkeysym::KeyCode`

Following conditions are considered as BUG:
* If any type cannot be accessed from any platform.
* if any type could not be converted to another type.
* if any "mirror" type is not synced with "dep" type.

# Limitations
we would also like to add conversion between "mirror" types and "dep" types.
