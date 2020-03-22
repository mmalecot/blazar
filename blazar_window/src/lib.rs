//! Provides a windowing API.

/// Kinds of window errors.
#[derive(Debug)]
pub enum WindowError {
    CreateWindowError,
}

/// Convenient result type consisting of a return type and a `WindowError`.
pub type Result<T = ()> = std::result::Result<T, WindowError>;

#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "windows")]
pub use windows::*;

#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "unix")]
pub use unix::*;
