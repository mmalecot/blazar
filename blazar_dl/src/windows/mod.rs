//! Windows implementation.

pub use blazar_winapi_sys as winapi_sys;

/// Returns a library filename.
#[doc(hidden)]
#[macro_export]
macro_rules! _library_filename {
    ($name:literal) => {
        concat!($name, ".dll")
    };
    ($name:literal, $version:literal) => {
        concat!($name, "-", $version, ".dll")
    };
}

/// Returns the handle's type of a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _handle_type {
    () => {
        blazar_dl::winapi_sys::HMODULE
    };
}

/// Loads the specified library.
#[doc(hidden)]
#[macro_export]
macro_rules! _load_library {
    ($filename:expr) => {{
        let filename = std::ffi::CString::new($filename).unwrap();
        blazar_dl::winapi_sys::LoadLibraryA(filename.as_ptr())
    }};
}

/// Loads a function of a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _load_function {
    ($handle:expr, $fn_name:ident) => {{
        let $fn_name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
        blazar_dl::winapi_sys::GetProcAddress($handle, $fn_name.as_ptr())
    }};
}

/// Unloads a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _unload_library {
    ($handle:expr) => {
        blazar_dl::winapi_sys::FreeLibrary($handle);
    };
}
