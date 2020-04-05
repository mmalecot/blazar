//! Multi-platform dynamic loading API.

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

/// Creates a dynamic library wrapper.
#[macro_export]
macro_rules! dynamic_loading {
    {
        #[load(wrapper = $wrapper:ident, error = $error:ident, name = $name:literal $(,version = $version:literal)?)]
        extern $abi:literal {
            $(pub fn $fn:ident($($param_name:ident: $param_type:ty),*) -> $ret_type:ty;)*
        }
    } => {
        /// Kinds of dynamic loading errors.
        #[derive(Debug)]
        pub enum $error {
            OpenFailed,
            FunctionNotFound(String),
        }

        /// Convenient result type.
        pub type Result<T = ()> = std::result::Result<T, $error>;

        /// Library wrapper.
        pub struct $wrapper {
            handle: blazar_dl::_handle_type!(),
            $(
                $fn: unsafe extern $abi fn($($param_type),*) -> $ret_type,
            )*
        }

        impl $wrapper {
            /// Loads the library.
            pub fn load() -> crate::Result<$wrapper> {
                unsafe {
                    let handle = blazar_dl::_load_library!(blazar_dl::_library_filename!($name $(,$version)?));
                    if handle.is_null() {
                        Err(crate::$error::OpenFailed)
                    }
                    else {
                        $(
                            let $fn = blazar_dl::_load_function!(handle, $fn);
                            if $fn.is_null() {
                                blazar_dl::_unload_library!(handle);
                                return Err(crate::$error::FunctionNotFound(String::from(stringify!($fn))));
                            }
                        )*
                        Ok($wrapper {
                            handle,
                            $(
                                $fn: std::mem::transmute($fn),
                            )*
                        })
                    }
                }
            }

            $(
                #[inline]
                pub unsafe fn $fn(&self, $($param_name: $param_type),*) -> $ret_type {
                    (self.$fn)($($param_name),*)
                }
            )*
        }

        impl Drop for $wrapper {
            fn drop(&mut self) {
                unsafe {
                    blazar_dl::_unload_library!(self.handle);
                }
            }
        }
    };
}
