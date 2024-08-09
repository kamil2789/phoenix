use std::rc::Rc;

use super::user_input::{KeyBinding, UserInput};
use super::Result;
use crate::window::Window;

pub struct GlfwEvent {}

pub struct EventManager {
    window: Rc<Window>,
    key_binding: KeyBinding,
}

impl EventManager {
    #[must_use]
    pub fn new(window: Rc<Window>) -> Self {
        let result = EventManager {
            window: window.clone(),
            key_binding: KeyBinding::new(window),
        };
        result
    }

    pub fn bind_key(&mut self, user_input: UserInput) {
        self.key_binding.bind_key(user_input);
    }

    pub fn process_key_callbacks(&mut self) -> Result<()> {
        self.key_binding.process_callbacks()
    }
}
