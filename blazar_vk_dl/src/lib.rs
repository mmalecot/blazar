//! Vulkan dynamic loading.

#![allow(non_snake_case)]

use blazar_dl::dynamic_loading;
use blazar_vk_sys::*;
use std::os::raw::*;

dynamic_loading! {
    #[load(wrapper = VulkanLibrary, error = LoadVulkanError, name = "vulkan", version = 1)]
    extern "C" {
        pub fn vkGetInstanceProcAddr(
            instance: VkInstance,
            pName: *const c_char
        ) -> PFN_vkVoidFunction;
    }
}
