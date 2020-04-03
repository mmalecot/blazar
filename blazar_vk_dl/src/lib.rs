//! Vulkan dynamic loading.

#![allow(non_snake_case)]

use blazar_dl::dynamic_loading;
use blazar_vk::*;
use std::os::raw::*;

dynamic_loading! {
    pub enum VulkanDynamicLoadingError {
        LoadLibraryError,
        LoadFunctionError,
    }

    #[load(name = "vulkan", version = 1)]
    pub struct VulkanLibrary {
        fn vkGetInstanceProcAddr(
            instance: VkInstance,
            pName: *const c_char
        ) -> PFN_vkVoidFunction;
    }
}
