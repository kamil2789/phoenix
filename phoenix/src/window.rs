mod glfw_lib_config;
mod glfw_window;

use crate::window::glfw_lib_config::create_lib;
use thiserror::Error;
use std::rc::Rc;

pub type Result<T> = std::result::Result<T, WindowError>;
#[derive(Error, Debug)]
pub enum WindowError {
    #[error("{0}")]
    WinLibraryInitError(String),
    #[error("{0}")]
    CreateWindowError(String),
}

pub enum Library {
    GLFW,
}

pub trait WinLibConfig {
    /// # Errors
    fn create_window(&self, resolution: Resolution, name: &str) -> Result<Rc<dyn Window>>;
    /// # Errors
    fn create_default_window(&self) -> Result<Rc<dyn Window>>;
}

pub trait Window {
    fn is_running(&self) -> bool;
    fn set_current(&self);
    fn swap_buffers(&self);
    fn set_close(&self);
}

pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

/// # Errors
pub fn create_window_lib_config(window_type: &Library) -> Result<Rc<dyn WinLibConfig>> {
    match window_type {
        Library::GLFW => {
            let result = create_lib()?;
            Ok(Rc::new(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_create_window_lib_config() {
        assert!(create_window_lib_config(&Library::GLFW).is_ok());
    }
}
