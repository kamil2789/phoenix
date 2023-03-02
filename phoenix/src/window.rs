mod glfw_lib_config;
mod glfw_window;

use crate::window::glfw_lib_config::create_lib;

use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    WinLibraryInitError(String),
    CreateWindowError(String),
}

pub enum Library {
    GLFW,
}

pub trait WinLibConfig {
    /// # Errors
    fn create_window(&self, resolution: Resolution, name: &str) -> Result<Rc<dyn Window>, Error>;
    /// # Errors
    fn create_default_window(&self) -> Result<Rc<dyn Window>, Error>;
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
pub fn create_window_lib_config(window_type: &Library) -> Result<Rc<dyn WinLibConfig>, Error> {
    match window_type {
        Library::GLFW => {
            let result = create_lib()?;
            Ok(Rc::new(result))
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WinLibraryInitError(message) | Error::CreateWindowError(message) => {
                write!(f, "{message}")
            }
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
