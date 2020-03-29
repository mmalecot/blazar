//! Windows implementation.

pub use blazar_win32 as win32;

/// Returns a library filename.
#[macro_export]
macro_rules! filename {
    ($lib_name:literal) => {
        concat!($lib_name, ".dll")
    };
    ($lib_name:literal, $lib_version:literal) => {
        concat!($lib_name, "-", $lib_version, ".dll")
    };
}

/// Creates a structure that wraps the functions of a library.
#[macro_export]
macro_rules! library {
    {
        #[load(name = $lib_name:literal $(,version = $lib_version:literal)?)]
        struct $struct_name:ident {
            $(fn $fn_name:ident($($param_name:ident: $param_type:ty),*) -> $ret_type:ty;)*
        }
    } => {
        /// Library wrapper.
        pub struct $struct_name {
            handle: win32::HMODULE,
            $(
                $fn_name: unsafe extern "C" fn($($param_type),*) -> $ret_type,
            )*
        }

        impl $struct_name {
            /// Loads the library.
            pub fn load() -> blazar_library::Result<$struct_name> {
                unsafe {
                    let filename = std::ffi::CString::new(blazar_library::filename!($lib_name $(,$lib_version)?)).unwrap();
                    let handle = win32::LoadLibraryA(filename.as_ptr());
                    if handle.is_null() {
                        Err(blazar_library::LibraryError::LoadLibraryError)
                    }
                    else {
                        $(
                            let $fn_name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
                            let $fn_name = win32::GetProcAddress(handle, $fn_name.as_ptr());
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
                #[inline]
                pub unsafe fn $fn_name(&self, $($param_name: $param_type),*) -> $ret_type {
                    (self.$fn_name)($($param_name),*)
                }
            )*
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                unsafe {
                    win32::FreeLibrary(self.handle);
                }
            }
        }
    };
}
