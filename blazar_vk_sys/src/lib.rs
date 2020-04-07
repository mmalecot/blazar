//! Vulkan raw FFI bindings.

#![allow(non_camel_case_types)]

// Types
pub type PFN_vkVoidFunction = Option<unsafe extern "C" fn()>;
pub type VkInstance = *mut VkInstance_T;

// Opaque structures
pub enum VkInstance_T {}
