pub mod action;
pub mod condition;
pub mod keyboard_input;
pub mod keys_binding;

use crate::window::Window;
use action::Action;
use condition::Condition;
use keyboard_input::{KeyBinding, KeyboardInput};
use std::rc::Rc;
use thiserror::Error;

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
    key_binding: KeyBinding,
}

impl Manager {
    #[must_use]
    pub fn new(window: Rc<Window>) -> Self {
        Self {
            key_binding: KeyBinding::new(window),
        }
    }

    pub fn bind_key(&mut self, keyboard_input: KeyboardInput, action: Action) {
        self.key_binding.bind_key(keyboard_input, action);
    }

    pub fn process_key_callbacks(&mut self) -> Vec<Action> {
        self.key_binding.process_key_status()
    }
}
