//! Linux implementation.

#[doc(hidden)]
pub use blazar_libc as libc;

/// Returns a library filename.
#[doc(hidden)]
#[macro_export]
macro_rules! _library_filename {
    ($name:literal) => {
        concat!("lib", $name, ".so")
    };
    ($name:literal, $version:literal) => {
        concat!("lib", $name, ".so.", $version)
    };
}

/// Returns the handle's type of a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _handle_type {
    () => {
        *mut std::os::raw::c_void
    };
}

/// Loads the specified library.
#[doc(hidden)]
#[macro_export]
macro_rules! _load_library {
    ($filename:expr) => {{
        let filename = std::ffi::CString::new($filename).unwrap();
        blazar_dl::libc::dlopen(filename.as_ptr(), blazar_dl::libc::RTLD_NOW)
    }};
}

/// Loads a function of a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _load_function {
    ($handle:expr, $fn_name:ident) => {{
        let $fn_name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
        blazar_dl::libc::dlsym($handle, $fn_name.as_ptr())
    }};
}

/// Unloads a library.
#[doc(hidden)]
#[macro_export]
macro_rules! _unload_library {
    ($handle:expr) => {
        blazar_dl::libc::dlclose($handle);
    };
}
