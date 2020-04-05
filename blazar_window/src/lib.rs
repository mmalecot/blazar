//! Multi-platform windowing API.

/// Kinds of window creation errors.
#[derive(Debug)]
pub enum CreateWindowError {
    ContextCreationFailed(String),
    WindowCreationFailed,
}

/// Convenient result type consisting of a return type and a `CreateWindowError`.
pub type Result<T = ()> = std::result::Result<T, CreateWindowError>;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
