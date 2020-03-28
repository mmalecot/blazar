//! Provides a dynamic loading API.

/// Kinds of library errors.
#[derive(Debug)]
pub enum LibraryError {
    LoadLibraryError,
    LoadFunctionError,
}

/// Convenient result type consisting of a return type and a `LibraryError`.
pub type Result<T = ()> = std::result::Result<T, LibraryError>;

#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "windows")]
pub use windows::*;

#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "unix")]
pub use unix::*;
