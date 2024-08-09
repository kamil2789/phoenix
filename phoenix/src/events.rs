pub mod action;
pub mod condition;
pub mod keys_binding;
pub mod user_input;

use std::rc::Rc;
use action::Action;
use condition::Condition;
use thiserror::Error;
use user_input::{KeyBinding, UserInput};
use crate::window::Window;

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

    pub fn bind_key(&mut self, user_input: UserInput, action: Action) {
        self.key_binding.bind_key(user_input, action);
    }

    pub fn process_key_callbacks(&mut self) -> Option<Vec<Action>> {
        self.key_binding.get_callback_actions()
    }
}
