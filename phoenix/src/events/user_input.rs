use crate::window::Window;
use std::{collections::HashMap, ffi::c_int, rc::Rc, sync::Mutex};

use glfw_sys::glfw_bindings::{
    glfwGetKey, glfwSetCursorPosCallback, glfwSetFramebufferSizeCallback, glfwSetKeyCallback,
    glfwSetScrollCallback, GLFWwindow, GLFW_PRESS,
};

use super::action::Action;

static SCROLL_INPUT: Mutex<Option<f32>> = Mutex::new(None);
static CURSOR_POS_INPUT: Mutex<Option<(f32, f32)>> = Mutex::new(None);

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

#[derive(Hash, Eq, PartialEq)]
pub enum MouseInput {
    Scroll,
    CursorPos,
}

pub(super) struct ControlBinding {
    window: Rc<Window>,
    key_binding: Vec<KeyEvent>,
    mouse_binding: HashMap<MouseInput, Action>,
}

struct KeyEvent {
    pub keyboard_input: KeyboardInput,
    pub action: Action,
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

impl ControlBinding {
    pub fn new(window: Rc<Window>) -> Self {
        ControlBinding::set_callbacks(&window);
        Self {
            window,
            key_binding: Vec::new(),
            mouse_binding: HashMap::new(),
        }
    }

    pub fn process_callbacks(&mut self) -> Vec<Action> {
        let mut result = self.get_keyboard_input();
        result.append(&mut self.get_mouse_input());
        result
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

    pub fn get_mouse_input(&self) -> Vec<Action> {
        let mut result = vec![];
        if let Some(action) = self.get_scroll_data() {
            result.push(action);
        }
        if let Some(action) = self.get_cursor_pos_data() {
            result.push(action);
        }

        result
    }

    pub fn bind_key(&mut self, keyboard_input: KeyboardInput, action: Action) {
        self.key_binding.push(KeyEvent {
            keyboard_input,
            action,
        });
    }

    pub fn bind_mouse(&mut self, mouse_input: MouseInput, action: Action) {
        self.mouse_binding.insert(mouse_input, action);
    }

    fn get_scroll_data(&self) -> Option<Action> {
        if self.mouse_binding.contains_key(&MouseInput::Scroll) {
            if let Ok(mut data) = SCROLL_INPUT.try_lock() {
                if let Some(value) = data.take() {
                    return Some(Action::CameraFov(value));
                }
            }
        }
        None
    }

    fn get_cursor_pos_data(&self) -> Option<Action> {
        if self.mouse_binding.contains_key(&MouseInput::CursorPos) {
            if let Ok(mut data) = CURSOR_POS_INPUT.try_lock() {
                if let Some(value) = data.take() {
                    return Some(Action::CameraOrientation(value.0, value.1));
                }
            }
        }
        None
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

impl Drop for ControlBinding {
    fn drop(&mut self) {
        ControlBinding::unset_callbacks(&self.window);
    }
}

//One callback is handled at a time

//Shouldn't change also resolution in window?
extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

#[allow(clippy::cast_possible_truncation)]
extern "C" fn cursor_pos_callback(_window: *mut GLFWwindow, xpos: f64, ypos: f64) {
    if let Ok(mut data) = CURSOR_POS_INPUT.lock() {
        *data = Some((xpos as f32, ypos as f32));
    }
}

#[allow(clippy::cast_possible_truncation)]
extern "C" fn scroll_callback(_window: *mut GLFWwindow, _xoffset: f64, yoffset: f64) {
    if let Ok(mut data) = SCROLL_INPUT.lock() {
        *data = Some(yoffset as f32);
    }
}
