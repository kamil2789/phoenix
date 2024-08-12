pub mod action;
pub mod condition;
pub mod keys_binding;
pub mod user_input;

use crate::window::Window;
use action::Action;
use condition::Condition;
use std::rc::Rc;
use thiserror::Error;
use user_input::{ControlBinding, KeyboardInput, MouseInput};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Key binding error {0}")]
    KeyBindingError(String),
}

pub enum EventLifetime {
    Once,
    Until,
    PerFrame,
}

#[allow(dead_code)]
pub struct Event {
    lifetime: EventLifetime,
    condition: Condition,
    action: Action,
}

pub struct Manager {
    control_binding: ControlBinding,
}

impl Manager {
    #[must_use]
    pub fn new(window: Rc<Window>) -> Self {
        Self {
            control_binding: ControlBinding::new(window),
        }
    }

    pub fn bind_key(&mut self, keyboard_input: KeyboardInput, action: Action) {
        self.control_binding.bind_key(keyboard_input, action);
    }

    pub fn process_user_input_callbacks(&mut self) -> Vec<Action> {
        self.control_binding.process_callbacks()
    }

    pub fn bind_mouse(&mut self, mouse_input: MouseInput, action: Action) {
        self.control_binding.bind_mouse(mouse_input, action);
    }
}
