use crate::window::Resolution;
use crate::window::Window;

use glfw_sys::glfw_bindings;

#[allow(dead_code)]
pub struct GlfwWindow {
    name: String,
    resolution: Resolution,
    window: *mut glfw_bindings::GLFWwindow,
}

impl GlfwWindow {
    pub fn new(
        name: String,
        resolution: Resolution,
        window: *mut glfw_bindings::GLFWwindow,
    ) -> Self {
        GlfwWindow {
            name,
            resolution,
            window,
        }
    }
}

impl Window for GlfwWindow {
    fn is_running(&self) -> bool {
        unsafe { glfw_bindings::glfwWindowShouldClose(self.window) == 0 }
    }

    fn set_current(&self) {
        unsafe {
            glfw_bindings::glfwMakeContextCurrent(self.window);
            glfw_bindings::glfwSetFramebufferSizeCallback(self.window, std::ptr::null_mut());
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            glfw_bindings::glfwSwapBuffers(self.window);
            //TODO find in the future a better place for poll events
            glfw_bindings::glfwPollEvents();
        }
    }

    fn set_close(&self) {
        unsafe {
            glfw_bindings::glfwSetWindowShouldClose(self.window, 1);
        }
    }
}

impl Drop for GlfwWindow {
    fn drop(&mut self) {
        unsafe {
            glfw_bindings::glfwDestroyWindow(self.window);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::create_lib;
    use crate::window::WinLibConfig;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_glfw_window_open_close() {
        let config = create_lib().unwrap();
        let window = config
            .create_window(
                Resolution {
                    width: 800,
                    height: 600,
                },
                "Hello window",
            )
            .unwrap();
        assert!(window.is_running());
        window.set_close();
        assert!(!window.is_running());
    }
}
