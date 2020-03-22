//! Provides a dynamic loading API.

/// Kinds of library errors.
#[derive(Debug)]
pub enum LibraryError {
    LoadLibraryError,
    LoadFunctionError,
}

/// Convenient result type consisting of a return type and a `LibraryError`.
pub type Result<T = ()> = std::result::Result<T, LibraryError>;

#[doc(hidden)]
#[cfg(target_family = "unix")]
pub use blazar_dl as dl;

#[doc(hidden)]
#[cfg(target_family = "windows")]
pub use blazar_win32 as win32;

/// Returns the filename of a library.
#[doc(hidden)]
#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! library_filename {
    ($name:literal) => {
        concat!("lib", $name, ".so");
    };
}

/// Returns the filename of a library.
#[doc(hidden)]
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! library_filename {
    ($name:literal) => {
        concat!($name, ".dll");
    };
}

/// Loads the specified library.
#[doc(hidden)]
#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! load_library {
    ($filename:expr) => {{
        let filename = std::ffi::CString::new($filename).unwrap();
        blazar_library::dl::dlopen(filename.as_ptr(), blazar_library::dl::RTLD_LAZY)
    }};
}

/// Loads the specified library.
#[doc(hidden)]
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! load_library {
    ($filename:expr) => {{
        let filename = std::ffi::CString::new($filename).unwrap();
        win32::LoadLibraryA(filename.as_ptr())
    }};
}

/// Unloads a library.
#[doc(hidden)]
#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! unload_library {
    ($handle:expr) => {{
        blazar_library::dl::dlclose($handle);
    }};
}

/// Unloads a library.
#[doc(hidden)]
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! unload_library {
    ($handle:expr) => {{
        win32::FreeLibrary($handle);
    }};
}

/// Loads a function of a library.
#[doc(hidden)]
#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! load_function {
    ($handle:expr, $fn_name:ident) => {{
        let $fn_name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
        blazar_library::dl::dlsym($handle, $fn_name.as_ptr())
    }};
}

/// Loads a function of a library.
#[doc(hidden)]
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! load_function {
    ($handle:expr, $fn_name:ident) => {{
        let $fn_name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
        win32::GetProcAddress($handle, $fn_name.as_ptr())
    }};
}

/// Returns the handle's type of a library.
#[doc(hidden)]
#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! handle_type {
    () => {
        *mut std::os::raw::c_void
    };
}

/// Returns the handle's type of a library.
#[doc(hidden)]
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! handle_type {
    () => {
        win32::HMODULE
    };
}

/// Creates a structure that wraps the functions of a library.
#[macro_export]
macro_rules! library {
    {
        #[load(name = $lib_name:literal)]
        struct $struct_name:ident {
            $(fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty;)*
        }
    } => {
        /// Library wrapper.
        pub struct $struct_name {
            handle: blazar_library::handle_type!(),
            $(
                $fn_name: unsafe extern "C" fn($($param_type),*) -> $ret_type,
            )*
        }

        impl $struct_name {
            /// Loads the library.
            pub fn load() -> blazar_library::Result<$struct_name> {
                unsafe {
                    let handle = blazar_library::load_library!(blazar_library::library_filename!($lib_name));
                    if handle.is_null() {
                        Err(blazar_library::LibraryError::LoadLibraryError)
                    }
                    else {
                        $(
                            let $fn_name = blazar_library::load_function!(handle, $fn_name);
                            if $fn_name.is_null() {
                                return Err(blazar_library::LibraryError::LoadFunctionError);
                            }
                        )*
                         Ok($struct_name {
                            handle,
                            $(
                                $fn_name: std::mem::transmute($fn_name),
                            )*
                        })
                    }
                }
            }

            $(
                pub unsafe fn $fn_name(&self, $($param_name: $param_type),*) -> $ret_type {
                    (self.$fn_name)($($param_name),*)
                }
            )*
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                unsafe {
                    blazar_library::unload_library!(self.handle);
                }
            }
        }
    };
}
