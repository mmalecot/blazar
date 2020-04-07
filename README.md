# Blazar

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mmalecot/blazar/CI)
[![Crates.io](https://img.shields.io/crates/v/blazar)](https://crates.io/crates/blazar)
[![Docs.rs](https://docs.rs/blazar/badge.svg)](https://docs.rs/blazar)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.42+-blueviolet.svg?logo=rust)

Simple and lite game engine focused on:
- Performance
- Minimum of dependencies
- Convenient API
- Modernity (thanks to the Vulkan graphics API)
- Multi-platform support (Windows and Linux)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
blazar = "1.0.0-dev.1"
```

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [Xlib documentation](https://www.x.org/releases/current/doc/libX11/libX11/libX11.html)
* [Windows API Index](https://docs.microsoft.com/en-us/windows/win32/apiindex/windows-api-list)
* [Vulkan overview](https://www.khronos.org/vulkan/)

## Native dependencies

**Blazar** only requires basic dependencies.

### Linux

Xlib, Vulkan ICD loader and Vulkan drivers for your graphics card are required.

#### Arch Linux

```sh
pacman -Syu libx11 vulkan-icd-loader
```

For Intel graphics card:

```sh
pacman -Syu vulkan-intel
```

For NVIDIA graphics card:

```sh
pacman -Syu nvidia-utils
```

For AMD graphics card:

```sh
pacman -Syu vulkan-radeon
```

### Windows

Only Vulkan drivers for your graphics card are required.

## Examples

### Simple window

Opens an empty window and prints events to stdout.

```sh
cargo run --example simple_window
```

## Workspace

The workspace is composed of the following members:
- `blazar_dl`: Multi-platform dynamic loading API.
- `blazar_event`: Definition of several types of events.
- `blazar_libc_sys`: libc raw FFI bindings.
- `blazar_vk_dl`: Vulkan dynamic loading.
- `blazar_vk_sys`: Vulkan raw FFI bindings.
- `blazar_winapi_sys`: Windows API raw FFI bindings.
- `blazar_window`: Multi-platform windowing API.
- `blazar_xlib_dl`: Xlib dynamic loading.
- `blazar_xlib_sys`: Xlib raw FFI bindings.

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
