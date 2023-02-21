use crate::window::Resolution;
use crate::window::WinLibConfig;
use crate::window::Window;
use crate::window::WindowError;

use glfw_sys::glfw_bindings;

use std::ffi::CString;
use std::rc::Rc;

pub fn create_glfw_lib_config() -> Result<GlfwLibConfig, WindowError> {
    let result = GlfwLibConfig {};
    unsafe {
        GlfwLibConfig::init_lib()?;
        GlfwLibConfig::init_hints();
    }
    Ok(result)
}

pub struct GlfwLibConfig {}

pub struct GlfwWindow {
    name: String,
    resolution: Resolution,
    window: *mut glfw_bindings::GLFWwindow,
}

impl GlfwLibConfig {
    fn new() -> Self {
        GlfwLibConfig {}
    }

    unsafe fn init_lib() -> Result<(), WindowError> {
        if glfw_bindings::glfwInit() == glfw_bindings::GLFW_TRUE {
            Ok(())
        } else {
            Err(WindowError::WinLibraryInitError(String::from(
                "Initialization of glfw failed",
            )))
        }
    }

    unsafe fn init_hints() {
        glfw_bindings::glfwWindowHint(glfw_bindings::GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfw_bindings::glfwWindowHint(glfw_bindings::GLFW_CONTEXT_VERSION_MINOR, 3);
        glfw_bindings::glfwWindowHint(
            glfw_bindings::GLFW_OPENGL_PROFILE,
            glfw_bindings::GLFW_OPENGL_CORE_PROFILE,
        );
    }
}

impl WinLibConfig for GlfwLibConfig {
    fn create_window(
        &self,
        resolution: Resolution,
        name: &str,
    ) -> Result<Rc<dyn Window>, WindowError> {
        let result = GlfwWindow::new(resolution, name)?;
        Ok(Rc::new(result))
    }
}

impl Drop for GlfwLibConfig {
    fn drop(&mut self) {
        unsafe {
            glfw_bindings::glfwTerminate();
        }
    }
}

impl GlfwWindow {
    fn new(resolution: Resolution, name: &str) -> Result<GlfwWindow, WindowError> {
        unsafe {
            let window = create_raw_window(&resolution, name)?;
            Ok(GlfwWindow {
                name: name.to_string(),
                resolution,
                window,
            })
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
}

impl Drop for GlfwWindow {
    fn drop(&mut self) {
        unsafe {
            glfw_bindings::glfwDestroyWindow(self.window);
        }
    }
}

unsafe fn create_raw_window(
    resolution: &Resolution,
    name: &str,
) -> Result<*mut glfw_bindings::GLFWwindow, WindowError> {
    let name_cstr = CString::new(name).expect("CString::new failed");
    let result = glfw_bindings::glfwCreateWindow(
        resolution.width.into(),
        resolution.height.into(),
        name_cstr.as_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    if result == std::ptr::null_mut() {
        Err(WindowError::CreateWindowError(String::from(
            "Error during function call glfwCreateWindow",
        )))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_glfw_lib_config() {
        println!("START");
        assert!(create_glfw_lib_config().is_ok());
    }
}
