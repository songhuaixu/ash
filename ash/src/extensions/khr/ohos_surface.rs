use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use core::mem;

impl crate::khr::ohos_surface::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_ohos_surface(
        &self,
        create_info: &vk::SurfaceCreateInfoOHOS<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_ohos_surface)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }
}
