use crate::window::Window;
use std::{ffi::c_int, rc::Rc};

use glfw_sys::glfw_bindings::{
    glfwGetKey, glfwSetCursorPosCallback, glfwSetFramebufferSizeCallback, glfwSetKeyCallback,
    glfwSetScrollCallback, GLFWwindow, GLFW_PRESS,
};

use super::action::Action;

#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardInput {
    key: i32,
    state: KeyState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyState {
    Release = 0,
    Press = 1,
}

struct KeyEvent {
    pub keyboard_input: KeyboardInput,
    pub action: Action,
}

pub(super) struct KeyBinding {
    window: Rc<Window>,
    key_binding: Vec<KeyEvent>,
}

impl KeyboardInput {
    #[must_use]
    pub fn new(key: i32, state: KeyState) -> Self {
        Self { key, state }
    }

    #[must_use]
    pub fn new_key(key: i32) -> Self {
        Self::new(key, KeyState::Press)
    }
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new(0, KeyState::Press)
    }
}

impl From<c_int> for KeyState {
    fn from(action: c_int) -> Self {
        match action {
            0 => Self::Release,
            _ => Self::Press,
        }
    }
}

impl KeyBinding {
    pub fn new(window: Rc<Window>) -> Self {
        KeyBinding::set_callbacks(&window);
        Self {
            window,
            key_binding: Vec::new(),
        }
    }

    pub fn process_key_status(&mut self) -> Vec<Action> {
        self.get_keyboard_input()
    }

    pub fn get_keyboard_input(&self) -> Vec<Action> {
        let mut results = Vec::new();
        for key in &self.key_binding {
            match key.keyboard_input.state {
                KeyState::Press => {
                    if self.is_key_pressed(key.keyboard_input.key) {
                        results.push(key.action.clone());
                    }
                }
                KeyState::Release => {
                    if !self.is_key_pressed(key.keyboard_input.key) {
                        results.push(key.action.clone());
                    }
                }
            }
        }

        results
    }

    pub fn bind_key(&mut self, keyboard_input: KeyboardInput, action: Action) {
        self.key_binding.push(KeyEvent {
            keyboard_input,
            action,
        });
    }

    fn is_key_pressed(&self, key: i32) -> bool {
        unsafe { glfwGetKey(self.window.get_raw_mut_window(), key) == GLFW_PRESS }
    }

    fn set_callbacks(window: &Window) {
        unsafe {
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
