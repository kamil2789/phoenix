mod glfw_window;

use crate::window::glfw_window::create_glfw_lib_config;

use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum WindowError {
    WinLibraryInitError(String),
    CreateWindowError(String),
}

pub enum WindowLibrary {
    GLFW,
}

pub trait WinLibConfig {
    fn create_window(
        &self,
        resolution: Resolution,
        name: &str,
    ) -> Result<Rc<dyn Window>, WindowError>;
}

pub trait Window {
    fn is_running(&self) -> bool;
    fn set_current(&self);
    fn swap_buffers(&self);
}

pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

pub fn create_window_lib_config(
    window_type: WindowLibrary,
) -> Result<Rc<dyn WinLibConfig>, WindowError> {
    match window_type {
        WindowLibrary::GLFW => {
            let result = create_glfw_lib_config()?;
            Ok(Rc::new(result))
        }
    }
}

impl Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowError::WinLibraryInitError(message) => write!(f, "{message}"),
            WindowError::CreateWindowError(message) => write!(f, "{message}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_window_lib_config() {
        let result = create_window_lib_config(WindowLibrary::GLFW);
        assert!(create_window_lib_config(WindowLibrary::GLFW).is_ok());
    }
}
