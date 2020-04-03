//! Multi-platform dynamic loading API.

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

/// Creates a structure that wraps the functions of a library.
#[macro_export]
macro_rules! dynamic_loading {
    {
        pub enum $error_name:ident {
            $load_library_error:ident,
            $load_function_error:ident,
        }
        #[load(name = $lib_name:literal $(,version = $lib_version:literal)?)]
        pub struct $struct_name:ident {
            $(fn $fn_name:ident($($param_name:ident: $param_type:ty),*) -> $ret_type:ty;)*
        }
    } => {
        pub mod dl {
            use super::*;

            /// Kinds of dynamic loading errors.
            #[derive(Debug)]
            pub enum $error_name {
                $load_library_error,
                $load_function_error,
            }

            /// Convenient result type.
            pub type Result<T = ()> = std::result::Result<T, $error_name>;

            /// Library wrapper.
            pub struct $struct_name {
                handle: blazar_dl::_handle_type!(),
                $(
                    $fn_name: unsafe extern "C" fn($($param_type),*) -> $ret_type,
                )*
            }

            impl $struct_name {
                /// Loads the library.
                pub fn load() -> crate::dl::Result<$struct_name> {
                    unsafe {
                        let handle = blazar_dl::_load_library!(blazar_dl::_library_filename!($lib_name $(,$lib_version)?));
                        if handle.is_null() {
                            Err(crate::dl::$error_name::$load_library_error)
                        }
                        else {
                            $(
                                let $fn_name = blazar_dl::_load_function!(handle, $fn_name);
                                if $fn_name.is_null() {
                                    blazar_dl::_unload_library!(handle);
                                    return Err(crate::dl::$error_name::$load_function_error);
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
                    #[inline]
                    pub unsafe fn $fn_name(&self, $($param_name: $param_type),*) -> $ret_type {
                        (self.$fn_name)($($param_name),*)
                    }
                )*
            }

            impl Drop for $struct_name {
                fn drop(&mut self) {
                    unsafe {
                        blazar_dl::_unload_library!(self.handle);
                    }
                }
            }
        }
    };
}
