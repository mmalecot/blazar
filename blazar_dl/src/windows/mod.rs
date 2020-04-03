//! Windows implementation.

pub use blazar_winapi as winapi;

/// Returns a library filename.
#[doc(hidden)]
#[macro_export]
macro_rules! _library_filename {
    ($lib_name:literal) => {
        concat!($lib_name, ".dll")
    };
    ($lib_name:literal, $lib_version:literal) => {
        concat!($lib_name, "-", $lib_version, ".dll")
    };
}

/// Returns the handle's type of a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _handle_type {
    () => {
        blazar_dl::winapi::HMODULE
    };
}

/// Loads the specified library.
#[doc(hidden)]
#[macro_export]
macro_rules! _load_library {
    ($filename:expr) => {{
        let filename = std::ffi::CString::new($filename).unwrap();
        blazar_dl::winapi::LoadLibraryA(filename.as_ptr())
    }};
}

/// Loads a function of a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _load_function {
    ($handle:expr, $fn_name:ident) => {{
        let $fn_name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
        blazar_dl::winapi::GetProcAddress($handle, $fn_name.as_ptr())
    }};
}

/// Unloads a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _unload_library {
    ($handle:expr) => {{
        blazar_dl::winapi::FreeLibrary($handle);
    }};
}
