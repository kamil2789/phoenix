macro_rules! vulkan_not_supported {
    ($render_type:expr) => {
        if $render_type == Api::Vulkan {
            return TestResult::NotSupported;
        }
    };
}

pub(crate) use vulkan_not_supported;
