use crate::window::glfw_window::GlfwWindow;
use crate::window::Error;
use crate::window::Resolution;
use crate::window::WinLibConfig;
use crate::window::Window;

use glfw_sys::glfw_bindings;

use std::ffi::CString;
use std::rc::Rc;

pub struct GlfwLibConfig {}

pub fn create_lib() -> Result<GlfwLibConfig, Error> {
    let result = GlfwLibConfig {};
    unsafe {
        GlfwLibConfig::init_lib()?;
        GlfwLibConfig::init_hints();
    }
    Ok(result)
}

impl GlfwLibConfig {
    unsafe fn init_lib() -> Result<(), Error> {
        if glfw_bindings::glfwInit() == glfw_bindings::GLFW_TRUE {
            Ok(())
        } else {
            Err(Error::WinLibraryInitError(String::from(
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
    fn create_window(&self, resolution: Resolution, name: &str) -> Result<Rc<dyn Window>, Error> {
        unsafe {
            let window = create_raw_window(&resolution, name)?;
            let result = GlfwWindow::new(name.to_string(), resolution, window);
            Ok(Rc::new(result))
        }
    }

    fn create_default_window(&self) -> Result<Rc<dyn Window>, Error> {
        unsafe {
            let window_name = "Default Window";
            let resolution = Resolution {
                width: 800,
                height: 600,
            };
            let window = create_raw_window(&resolution, window_name)?;
            let result = GlfwWindow::new(window_name.to_string(), resolution, window);
            Ok(Rc::new(result))
        }
    }
}

impl Drop for GlfwLibConfig {
    fn drop(&mut self) {
        unsafe {
            glfw_bindings::glfwTerminate();
        }
    }
}

unsafe fn create_raw_window(
    resolution: &Resolution,
    name: &str,
) -> Result<*mut glfw_bindings::GLFWwindow, Error> {
    let name_cstr = CString::new(name).expect("CString::new failed");
    let result = glfw_bindings::glfwCreateWindow(
        resolution.width.into(),
        resolution.height.into(),
        name_cstr.as_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    if result.is_null() {
        Err(Error::CreateWindowError(String::from(
            "Error during function call glfwCreateWindow",
        )))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_create_glfw_lib_config() {
        assert!(create_lib().is_ok());
    }

    #[test]
    #[serial]
    fn test_create_window() {
        if let Ok(config) = create_lib() {
            let window = config.create_window(
                Resolution {
                    width: 800,
                    height: 600,
                },
                "Hello window",
            );
            assert!(window.is_ok());
        } else {
            assert!(false);
        }
    }

    #[test]
    #[serial]
    fn test_create_default_window() {
        if let Ok(config) = create_lib() {
            let window = config.create_default_window();
            assert!(window.is_ok());
        } else {
            assert!(false);
        }
    }
}
