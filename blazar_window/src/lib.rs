//! Provides a multiplatform windowing API.

/// Kinds of window errors.
#[derive(Debug)]
pub enum WindowError {
    CreateWindowError,
}

/// Convenient result type consisting of a return type and a `WindowError`.
pub type Result<T = ()> = std::result::Result<T, WindowError>;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
