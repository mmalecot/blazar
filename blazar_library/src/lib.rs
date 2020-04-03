//! Provides a dynamic loading API.

/// Kinds of library errors.
#[derive(Debug)]
pub enum LibraryError {
    LoadLibraryError,
    LoadFunctionError,
}

/// Convenient result type consisting of a return type and a `LibraryError`.
pub type Result<T = ()> = std::result::Result<T, LibraryError>;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
