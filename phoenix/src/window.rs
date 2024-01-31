use glfw_sys::glfw_bindings;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WinError>;

#[derive(Error, Debug)]
pub enum WinError {
    #[error("{0}")]
    WinLibraryInitError(String),
    #[error("{0}")]
    CreateWinError(String),
}
pub struct GlfwConfig {}

impl GlfwConfig {

    /// # Errors
    ///
    /// Will return `Err` if glfw library does not initialize properly.
    /// There should be only one instance of `GlfwConfig`.
    pub fn create() -> Result<GlfwConfig> {
        GlfwConfig::init()?;
        GlfwConfig::init_hints();
        Ok(GlfwConfig{})
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_create_glfw_config() {
        assert!(GlfwConfig::create().is_ok());
    }

    #[test]
    #[serial]
    fn test_create_many_glfw_config_no_panic() {
        let _first = GlfwConfig{};
        let _second = GlfwConfig::create();
        let _third = GlfwConfig::create();
    }
}

