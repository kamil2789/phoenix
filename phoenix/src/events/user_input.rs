use crate::window::Window;
use std::{ffi::c_int, rc::Rc, sync::Mutex};

use glfw_sys::glfw_bindings::{
    glfwSetCursorPosCallback, glfwSetFramebufferSizeCallback, glfwSetKeyCallback,
    glfwSetScrollCallback, GLFWwindow,
};

use super::action::Action;

static KEY_CALLBACK_QUEUE: Mutex<Vec<UserInput>> = Mutex::new(Vec::new());

#[derive(Clone, Debug, PartialEq)]
pub struct UserInput {
    key: i32,
    action: KeyState,
    mods: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyState {
    Release = 0,
    Press = 1,
    Repeat = 2,
    Hold = 3,
}

struct KeyEvent {
    pub user_input: UserInput,
    pub action: Action,
}

pub(super) struct KeyBinding {
    window: Rc<Window>,
    key_binding: Vec<KeyEvent>,
}

impl UserInput {
    #[must_use]
    pub fn new(key: i32, action: KeyState, mods: i32) -> Self {
        Self { key, action, mods }
    }

    #[must_use]
    pub fn new_key_press(key: i32) -> Self {
        Self::new(key, KeyState::Press, 0)
    }

    #[must_use]
    pub fn new_key_hold(key: i32) -> Self {
        Self::new(key, KeyState::Hold, 0)
    }
}

impl Default for UserInput {
    fn default() -> Self {
        Self::new(0, KeyState::Press, 0)
    }
}

impl From<c_int> for KeyState {
    fn from(action: c_int) -> Self {
        match action {
            0 => Self::Release,
            2 => Self::Repeat,
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

    pub fn get_callback_actions(&mut self) -> Option<Vec<Action>> {
        if let Ok(mut queue) = KEY_CALLBACK_QUEUE.lock() {
            let mut result = Vec::new();
            while let Some(user_input) = queue.pop() {
                if let Some(action) = self.get_action_from_user_input(&user_input) {
                    result.push(action);
                }
            }

            Some(result)
        } else {
            None
        }
    }

    pub fn bind_key(&mut self, user_input: UserInput, action: Action) {
        self.key_binding.push(KeyEvent { user_input, action });
    }

    fn get_action_from_user_input(&self, user_input: &UserInput) -> Option<Action> {
        for key_event in &self.key_binding {
            if key_event.user_input == *user_input {
                return Some(key_event.action.clone());
            }
        }

        None
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
