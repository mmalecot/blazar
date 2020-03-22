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
- Multiplatform support (Windows and Linux)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
blazar = "0.1"
```

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [Xlib documentation](https://www.x.org/releases/current/doc/libX11/libX11/libX11.html)
* [Windows API Index](https://docs.microsoft.com/en-us/windows/win32/apiindex/windows-api-list)
* [Vulkan overview](https://www.khronos.org/vulkan/)

## Examples

### Window

Opens an empty window and prints events to stdout.

```sh
cargo run --example window
```

## Workspace

The workspace is composed of the following members:
- `blazar_dl`: libdl FFI (Unix).
- `blazar_event`: Provides several types of events.
- `blazar_library`: Provides a dynamic loading API.
- `blazar_win32`: Win32 FFI (Windows).
- `blazar_window`: Provides a multiplatform windowing API.
- `blazar_x11`: libX11 FFI (Unix).

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
