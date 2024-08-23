use glfw_sys::glfw_bindings;
use glfw_sys::glfw_bindings::GLFW_CURSOR_DISABLED;
use glfw_sys::glfw_bindings::GLFW_CURSOR_NORMAL;
use std::ffi::c_int;
use std::ffi::CString;
use std::ffi::NulError;

pub type Result<T> = std::result::Result<T, WinError>;

#[derive(thiserror::Error, Debug)]
pub enum WinError {
    #[error("{0}")]
    WinLibraryInitError(String),
    #[error("{0}")]
    CreateWinError(String),
    #[error("{0}")]
    CStringError(#[from] NulError),
    #[error("{0}")]
    RuntimeError(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

pub struct GlfwConfig {}

pub struct Window {
    window: *mut glfw_bindings::GLFWwindow,
    name: String,
    resolution: Resolution,
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            width: 800,
            height: 600,
        }
    }
}

impl GlfwConfig {
    /// # Errors
    ///
    /// Will return `Err` if glfw library does not initialize properly.
    /// There should be only one instance of `GlfwConfig`.
    pub fn create() -> Result<GlfwConfig> {
        GlfwConfig::init()?;
        GlfwConfig::init_hints();
        Ok(GlfwConfig {})
    }

    /// # Errors
    ///
    /// Will return `Err` in the following cases:
    /// GLFW library does not create a window properly.
    /// Height or width in resolution is zero.
    /// Invalid `CString` format, see `CString::new`.
    pub fn create_window(&self, name: &str, resolution: Resolution) -> Result<Window> {
        let name_cstr = CString::new(name)?;
        if resolution.height == 0 || resolution.width == 0 {
            return Err(WinError::CreateWinError(String::from(
                "No resolution dimension can be zero",
            )));
        }

        let window = unsafe {
            glfw_bindings::glfwCreateWindow(
                resolution.width.into(),
                resolution.height.into(),
                name_cstr.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if window.is_null() {
            return Err(WinError::CreateWinError(String::from(
                "Error during function call glfwCreateWindow",
            )));
        }

        Ok(Window {
            window,
            name: name.to_string(),
            resolution,
        })
    }

    fn init() -> Result<()> {
        if unsafe { glfw_bindings::glfwInit() } == glfw_bindings::GLFW_TRUE {
            Ok(())
        } else {
            Err(WinError::WinLibraryInitError(String::from(
                "Initialization of glfw failed",
            )))
        }
    }

    fn init_hints() {
        unsafe {
            glfw_bindings::glfwWindowHint(glfw_bindings::GLFW_CONTEXT_VERSION_MAJOR, 4);
            glfw_bindings::glfwWindowHint(glfw_bindings::GLFW_CONTEXT_VERSION_MINOR, 0);
            glfw_bindings::glfwWindowHint(
                glfw_bindings::GLFW_OPENGL_PROFILE,
                glfw_bindings::GLFW_OPENGL_CORE_PROFILE,
            );
        }
    }
}

impl Drop for GlfwConfig {
    fn drop(&mut self) {
        unsafe {
            glfw_bindings::glfwTerminate();
        }
    }
}

impl Window {
    #[must_use]
    pub fn is_running(&self) -> bool {
        unsafe { glfw_bindings::glfwWindowShouldClose(self.window) == 0 }
    }

    pub fn set_current(&self) {
        unsafe {
            glfw_bindings::glfwMakeContextCurrent(self.window);
        }
    }

    #[must_use]
    pub fn is_current(&self) -> bool {
        unsafe { glfw_bindings::glfwGetCurrentContext() == self.window }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            glfw_bindings::glfwSwapBuffers(self.window);
        }
    }

    pub fn close(&self) {
        unsafe {
            glfw_bindings::glfwSetWindowShouldClose(self.window, 1);
        }
    }

    pub fn poll_events() {
        unsafe {
            glfw_bindings::glfwPollEvents();
        }
    }

    #[must_use]
    pub fn get_resolution(&self) -> Resolution {
        self.resolution.clone()
    }

    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        unsafe { glfw_bindings::glfwGetFramebufferSize(self.window, &mut width, &mut height) };
        (width, height)
    }

    #[must_use]
    pub fn get_raw_mut_window(&self) -> *mut glfw_bindings::GLFWwindow {
        self.window
    }

    pub fn set_capture_mouse(&self, capture: bool) {
        unsafe {
            if capture {
                glfw_bindings::glfwSetInputMode(
                    self.window,
                    glfw_bindings::GLFW_CURSOR,
                    GLFW_CURSOR_DISABLED,
                );
            } else {
                glfw_bindings::glfwSetInputMode(
                    self.window,
                    glfw_bindings::GLFW_CURSOR,
                    GLFW_CURSOR_NORMAL,
                );
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfw_bindings::glfwDestroyWindow(self.window);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_resolution_default() {
        let resolution = Resolution::default();
        assert_eq!(resolution.width, 800);
        assert_eq!(resolution.height, 600);
    }

    #[test]
    #[serial]
    fn test_create_glfw_config() {
        assert!(GlfwConfig::create().is_ok());
    }

    #[test]
    #[serial]
    fn test_create_many_glfw_config_no_panic() {
        let first = GlfwConfig::create();
        assert!(first.is_ok());
        let second = GlfwConfig::create();
        assert!(second.is_ok());
    }

    #[test]
    #[serial]
    fn test_run_window_glfw() {
        let config = GlfwConfig::create().unwrap();
        let resolution = Resolution {
            width: 800,
            height: 600,
        };
        let res_clone = resolution.clone();
        let window = config.create_window("test_win_opengl", resolution).unwrap();
        assert_eq!(window.get_resolution(), res_clone);
        assert_eq!(window.get_name(), "test_win_opengl");
    }

    #[test]
    #[serial]
    fn test_run_window_glfw_swap_bufers() {
        let config = GlfwConfig::create().unwrap();
        let resolution = Resolution {
            width: 800,
            height: 600,
        };
        let window = config.create_window("test_win_opengl", resolution).unwrap();
        window.set_current();
        window.swap_buffers();
        assert!(window.is_running());
    }

    #[test]
    #[serial]
    fn test_window_create_error_glfw_not_initialized() {
        let config = GlfwConfig {}; //library not initialized
        let resolution = Resolution {
            width: 800,
            height: 600,
        };
        let window = config.create_window("test_win_opengl", resolution);
        assert!(window.is_err());
    }

    #[test]
    #[serial]
    fn test_window_create_error_invalid_name() {
        let config = GlfwConfig::create().unwrap();
        let resolution = Resolution {
            width: 800,
            height: 600,
        };
        let window = config.create_window("test_w\0in_opengl", resolution);
        assert!(window.is_err());
    }

    #[test]
    #[serial]
    fn test_window_create_error_zero_resolution() {
        let config = GlfwConfig::create().unwrap();
        let resolution = Resolution {
            width: 0,
            height: 0,
        };
        let window = config.create_window("test_win_opengl", resolution);
        assert!(window.is_err());
    }

    #[test]
    #[serial]
    fn test_window_get_framebuffer_size() {
        let config = GlfwConfig::create().unwrap();
        let resolution = Resolution {
            width: 900,
            height: 600,
        };
        let window = config.create_window("test_win_opengl", resolution).unwrap();
        let (x, y) = window.get_framebuffer_size();
        assert_eq!(x, 900);
        assert_eq!(y, 600);
    }
}
