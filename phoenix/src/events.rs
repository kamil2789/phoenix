pub mod action;
pub mod condition;
pub mod keys_binding;
pub mod manager;
pub mod user_input;

use action::Action;
use condition::Condition;
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

pub struct Event {
    lifetime: EventLifetime,
    condition: Condition,
    action: Action,
}
