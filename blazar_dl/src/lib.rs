//! Multi-platform dynamic loading API.

/// Kinds of dynamic loading errors.
#[derive(Debug)]
pub enum DynamicLoadingError {
    LoadLibraryError,
    LoadFunctionError,
}

/// Convenient result type consisting of a return type and a `DynamicLoadingError`.
pub type Result<T = ()> = std::result::Result<T, DynamicLoadingError>;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
