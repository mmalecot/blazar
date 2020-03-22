//! libdl FFI.

use std::os::raw::*;

// Constants
pub const RTLD_LAZY: c_int = 0x0001;

// Functions
#[link(name = "dl")]
extern "C" {
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}
