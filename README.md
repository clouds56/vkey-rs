# VKey
This crates only contains various types of virtual keycodes, and conversions between them.

This crate is under heavy development, and subject to breaking changes.

- [x] Vk from [winput](https://github.com/gymore-io/winput)
- [x] VIRTUAL_KEY from [windows](https://github.com/microsoft/windows-rs)
- [ ] enigo::Key from [enigo](https://github.com/enigo-rs/enigo) (partially done)
- [x] CGKeyCode from [macos](https://github.com/phracker/MacOSX-SDKs)
- [x] Usage from [hut](https://github.com/hidutils/hut)
- [ ] winit::KeyCode from [winit](https://github.com/rust-windowing/winit) (only definitions)
- [x] xkeysym::KeyCode from [xkeysym](https://github.com/rust-windowing/xkeysym)

Following conditions are considered as BUG:
* If any type cannot be accessed from any platform.
* if any type could not be converted to another type.
* if any "mirrored" type is not synced with "underlying" type.

# Limitations
we would also like to add conversion between "mirrored" types and "underlying" types.
