use crate::events::Result;
use crate::window::Window;
use std::{ffi::c_int, rc::Rc, sync::Mutex};

use glfw_sys::glfw_bindings::{
    glfwSetCursorPosCallback, glfwSetFramebufferSizeCallback, glfwSetKeyCallback,
    glfwSetScrollCallback, GLFWwindow,
};

use super::Error;

static KEY_CALLBACK_QUEUE: Mutex<Vec<UserInput>> = Mutex::new(Vec::new());

#[derive(Clone, Debug, PartialEq)]
pub struct UserInput {
    key: i32,
    action: UserInputAction,
    mods: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserInputAction {
    Release = 0,
    Press = 1,
    Repeat = 2,
}

pub(super) struct KeyBinding {
    window: Rc<Window>,
    key_binding: Vec<UserInput>,
}

impl UserInput {
    pub fn new(key: i32, action: UserInputAction, mods: i32) -> Self {
        Self { key, action, mods }
    }

    pub fn new_key_press(key: i32) -> Self {
        Self::new(key, UserInputAction::Press, 0)
    }
}

impl Default for UserInput {
    fn default() -> Self {
        Self::new(0, UserInputAction::Press, 0)
    }
}

impl From<c_int> for UserInputAction {
    fn from(action: c_int) -> Self {
        match action {
            0 => Self::Release,
            1 => Self::Press,
            2 => Self::Repeat,
            _ => Self::Press,
        }
    }
}

impl KeyBinding {
    pub fn new(window: Rc<Window>) -> Self {
        KeyBinding::set_callbacks(&window);
        let result = Self {
            window,
            key_binding: Vec::new(),
        };
        result
    }

    //change bool to Result
    pub fn process_callbacks(&mut self) -> Result<()> {
        if let Ok(mut queue) = KEY_CALLBACK_QUEUE.lock() {
            while let Some(user_input) = queue.pop() {
                if self.key_binding.contains(&user_input) {
                    println!("TEST ME HELO {}", user_input.key);
                }
            }
        } else {
            return Err(Error::KeyBindingError(String::from(
                "Failed to process user inputs",
            )));
        }

        Ok(())
    }

    pub fn bind_key(&mut self, user_input: UserInput) {
        self.key_binding.push(user_input);
    }

    fn set_callbacks(window: &Window) {
        unsafe {
            glfwSetKeyCallback(window.get_raw_mut_window(), Some(key_callback));
            glfwSetFramebufferSizeCallback(
                window.get_raw_mut_window(),
                Some(framebuffer_size_callback),
            );
            glfwSetCursorPosCallback(window.get_raw_mut_window(), Some(cursor_pos_callback));
            glfwSetScrollCallback(window.get_raw_mut_window(), Some(scroll_callback));
        }
    }

    fn unset_callbacks(window: &Window) {
        unsafe {
            glfwSetKeyCallback(window.get_raw_mut_window(), None);
            glfwSetFramebufferSizeCallback(window.get_raw_mut_window(), None);
            glfwSetCursorPosCallback(window.get_raw_mut_window(), None);
            glfwSetScrollCallback(window.get_raw_mut_window(), None);
        }
    }
}

impl Drop for KeyBinding {
    fn drop(&mut self) {
        KeyBinding::unset_callbacks(&self.window);
    }
}

extern "C" fn key_callback(
    _window: *mut GLFWwindow,
    key: c_int,
    _scancode: c_int,
    action: c_int,
    mods: c_int,
) {
    let action = UserInput::new(key, action.into(), mods);
    if let Ok(mut queue) = KEY_CALLBACK_QUEUE.try_lock() {
        queue.push(action);
    }
}

//One callback is handled at a time

//Shouldn't change also resolution in window?
extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

extern "C" fn cursor_pos_callback(_window: *mut GLFWwindow, _xpos: f64, _ypos: f64) {
    //nothing right now
}

extern "C" fn scroll_callback(_window: *mut GLFWwindow, _xoffset: f64, _yoffset: f64) {
    //nothing right now
}
