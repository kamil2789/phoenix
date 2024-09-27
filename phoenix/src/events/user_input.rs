use crate::window::Window;
use std::{collections::HashMap, ffi::c_int, rc::Rc, sync::Mutex};

use glfw_sys::glfw_bindings::{
    glfwGetKey, glfwSetCursorPosCallback, glfwSetFramebufferSizeCallback, glfwSetKeyCallback,
    glfwSetScrollCallback, GLFWwindow, GLFW_PRESS,
};

use super::{action::Action, UserInput};

static SCROLL_INPUT: Mutex<Option<f32>> = Mutex::new(None);
static CURSOR_POS_INPUT: Mutex<Option<(f32, f32)>> = Mutex::new(None);

pub struct GlfwUserInputHandler {
    pub window: Rc<Window>,
}

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
    key_binding: Vec<KeyEvent>,
    mouse_binding: HashMap<MouseInput, Action>,
    user_input_handler: Box<dyn UserInput>,
}

struct KeyEvent {
    pub keyboard_input: KeyboardInput,
    pub action: Action,
}

impl UserInput for GlfwUserInputHandler {
    fn is_key_pressed(&self, key: i32) -> bool {
        unsafe { glfwGetKey(self.window.get_raw_mut_window(), key) == GLFW_PRESS }
    }

    fn set_callbacks(&self) {
        unsafe {
            glfwSetFramebufferSizeCallback(
                self.window.get_raw_mut_window(),
                Some(framebuffer_size_callback),
            );
            glfwSetCursorPosCallback(self.window.get_raw_mut_window(), Some(cursor_pos_callback));
            glfwSetScrollCallback(self.window.get_raw_mut_window(), Some(scroll_callback));
        }
    }

    fn get_key_state(&self, key: i32) -> KeyState {
        unsafe { glfwGetKey(self.window.get_raw_mut_window(), key).into() }
    }
}

impl GlfwUserInputHandler {
    fn unset_callbacks(window: &Window) {
        unsafe {
            glfwSetKeyCallback(window.get_raw_mut_window(), None);
            glfwSetFramebufferSizeCallback(window.get_raw_mut_window(), None);
            glfwSetCursorPosCallback(window.get_raw_mut_window(), None);
            glfwSetScrollCallback(window.get_raw_mut_window(), None);
        }
    }
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
    pub fn new(user_input_handler: Box<dyn UserInput>) -> Self {
        user_input_handler.set_callbacks();
        Self {
            key_binding: Vec::new(),
            mouse_binding: HashMap::new(),
            user_input_handler,
        }
    }

    pub fn collect_user_actions(&mut self) -> Vec<Action> {
        let mut result = self.collect_keyboard_actions();
        result.append(&mut self.get_mouse_input());
        result
    }

    fn collect_keyboard_actions(&self) -> Vec<Action> {
        self.key_binding
            .iter()
            .filter(|event| {
                event.keyboard_input.state
                    == self
                        .user_input_handler
                        .get_key_state(event.keyboard_input.key)
            })
            .map(|event| event.action.clone())
            .collect()
    }

    fn get_mouse_input(&self) -> Vec<Action> {
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
}

impl Drop for GlfwUserInputHandler {
    fn drop(&mut self) {
        GlfwUserInputHandler::unset_callbacks(&self.window);
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

#[cfg(test)]
mod tests {
    use cgmath::assert_relative_eq;
    use serial_test::serial;

    use crate::{
        events::{
            action::Action,
            keys_binding::{KEY_A, KEY_D, KEY_P, KEY_W},
            user_input::{scroll_callback, KeyState, MouseInput},
            UserInput,
        },
        window::{GlfwConfig, Resolution},
    };

    use super::{
        cursor_pos_callback, ControlBinding, KeyboardInput, CURSOR_POS_INPUT, SCROLL_INPUT,
    };

    struct TestUserInput {
        pub pressed_keys: Vec<i32>,
    }

    impl UserInput for TestUserInput {
        fn is_key_pressed(&self, key: i32) -> bool {
            self.pressed_keys.contains(&key)
        }

        fn set_callbacks(&self) {}

        fn get_key_state(&self, key: i32) -> KeyState {
            if self.is_key_pressed(key) {
                KeyState::Press
            } else {
                KeyState::Release
            }
        }
    }

    fn test_scroll_input_callback() {
        if let Ok(mut data) = SCROLL_INPUT.try_lock() {
            *data = Some(5.0);
        }
    }

    fn test_cursor_pos_input_callback() {
        if let Ok(mut data) = CURSOR_POS_INPUT.try_lock() {
            *data = Some((2.0, 4.0));
        }
    }

    #[test]
    fn test_control_binding_new() {
        let user_input_handler: Box<dyn UserInput> = Box::new(TestUserInput {
            pressed_keys: vec![],
        });
        let mut control_binding = ControlBinding::new(user_input_handler);

        control_binding.bind_key(
            KeyboardInput::new_key(KEY_D.into()),
            Action::CameraUpdateRight,
        );

        let actions = control_binding.collect_user_actions();
        assert!(actions.is_empty());
    }

    #[test]
    fn test_control_binding_bind_key() {
        let user_input_handler: Box<dyn UserInput> = Box::new(TestUserInput {
            pressed_keys: vec![KEY_D.into()],
        });
        let mut control_binding = ControlBinding::new(user_input_handler);

        control_binding.bind_key(
            KeyboardInput::new_key(KEY_D.into()),
            Action::CameraUpdateRight,
        );

        let actions = control_binding.collect_user_actions();
        assert_eq!(actions, vec![Action::CameraUpdateRight]);
    }

    #[test]
    fn test_control_binding_bind_key_when_released() {
        let user_input_handler: Box<dyn UserInput> = Box::new(TestUserInput {
            pressed_keys: vec![],
        });
        let mut control_binding = ControlBinding::new(user_input_handler);

        control_binding.bind_key(
            KeyboardInput::new(KEY_D.into(), KeyState::Release),
            Action::CameraUpdateRight,
        );

        let actions = control_binding.collect_user_actions();
        assert_eq!(actions, vec![Action::CameraUpdateRight]);
    }

    #[test]
    fn test_control_binding_bind_mouse() {
        let user_input_handler: Box<dyn UserInput> = Box::new(TestUserInput {
            pressed_keys: vec![KEY_D.into(), KEY_A.into()],
        });
        let mut control_binding = ControlBinding::new(user_input_handler);

        control_binding.bind_mouse(MouseInput::Scroll, Action::CameraFov(0.0));

        let actions = control_binding.collect_user_actions();
        assert!(actions.is_empty());

        test_scroll_input_callback();

        let actions = control_binding.collect_user_actions();
        assert_eq!(actions, vec![Action::CameraFov(5.0)]);
    }

    #[test]
    fn test_control_binding_bind_keys_and_mouse() {
        let user_input_handler: Box<dyn UserInput> = Box::new(TestUserInput {
            pressed_keys: vec![KEY_D.into(), KEY_A.into(), KEY_P.into()],
        });

        let mut control_binding = ControlBinding::new(user_input_handler);

        control_binding.bind_mouse(MouseInput::CursorPos, Action::CameraOrientation(0.0, 0.0));

        control_binding.bind_key(
            KeyboardInput::new_key(KEY_D.into()),
            Action::CameraUpdateRight,
        );

        control_binding.bind_key(
            KeyboardInput::new_key(KEY_P.into()),
            Action::CameraUpdateLeft,
        );

        control_binding.bind_key(
            KeyboardInput::new_key(KEY_W.into()),
            Action::CameraUpdateForward,
        );

        test_cursor_pos_input_callback();

        let actions = control_binding.collect_user_actions();
        assert!(actions.contains(&Action::CameraOrientation(2.0, 4.0)));
        assert!(actions.contains(&Action::CameraUpdateRight));
        assert!(actions.contains(&Action::CameraUpdateLeft));
    }

    #[test]
    fn test_keyboard_input_new() {
        let keyboard_input = KeyboardInput::default();
        assert_eq!(keyboard_input.key, 0);
        assert_eq!(keyboard_input.state, 1.into());
    }

    #[test]
    #[serial]
    fn test_glfw_callbacks() {
        let config = GlfwConfig::create().unwrap();
        let resolution = Resolution {
            width: 800,
            height: 600,
        };

        let window = config.create_window("test_win_opengl", resolution).unwrap();

        {
            let cursor_pos = CURSOR_POS_INPUT.try_lock().unwrap();
            assert!(cursor_pos.is_none());

            let scroll_data = SCROLL_INPUT.try_lock().unwrap();
            assert!(scroll_data.is_none());
        }

        cursor_pos_callback(window.get_raw_mut_window(), 2.0, 4.0);
        scroll_callback(window.get_raw_mut_window(), 0.0, 5.0);

        let scroll_data = SCROLL_INPUT.try_lock().unwrap();
        let cursor_pos = CURSOR_POS_INPUT.try_lock().unwrap();

        let cursor_values = cursor_pos.unwrap();
        assert_relative_eq!(cursor_values.0, 2.0);
        assert_relative_eq!(cursor_values.1, 4.0);
        assert_relative_eq!(scroll_data.unwrap(), 5.0);
    }
}
