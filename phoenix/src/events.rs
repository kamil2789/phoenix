pub mod action;
pub mod condition;
pub mod keys_binding;
pub mod user_input;
mod world_events;

use crate::window::Window;
use action::Action;
use condition::Condition;
use std::rc::Rc;
use thiserror::Error;
use user_input::{ControlBinding, KeyboardInput, MouseInput};
use world_events::WorldEvents;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Key binding error {0}")]
    KeyBindingError(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventLifetime {
    Once,
    //Until,
    PerFrame,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    lifetime: EventLifetime,
    condition: Condition,
    action: Action,
}

pub struct Manager {
    control_binding: ControlBinding,
    world_events: WorldEvents,
}

impl Manager {
    #[must_use]
    pub fn new(window: Rc<Window>) -> Self {
        Self {
            control_binding: ControlBinding::new(window),
            world_events: WorldEvents::default(),
        }
    }

    pub fn bind_key(&mut self, keyboard_input: KeyboardInput, action: Action) {
        self.control_binding.bind_key(keyboard_input, action);
    }

    pub fn bind_mouse(&mut self, mouse_input: MouseInput, action: Action) {
        self.control_binding.bind_mouse(mouse_input, action);
    }

    pub fn add_event(&mut self, event: Event) {
        self.world_events.add(event);
    }

    pub fn add_high_priority_event(&mut self, event: Event) {
        self.world_events.add(event);
    }

    pub fn process_events(&mut self) -> Vec<Action> {
        let mut actions = self.process_user_input_callbacks();
        self.process_world_events(&mut actions)
    }

    fn process_user_input_callbacks(&mut self) -> Vec<Action> {
        self.control_binding.process_callbacks()
    }

    fn process_world_events(&mut self, actions: &mut Vec<Action>) -> Vec<Action> {
        self.world_events.process_events(actions)
    }
}

impl Event {
    #[must_use]
    pub fn new(lifetime: EventLifetime, condition: Condition, action: Action) -> Self {
        Self {
            lifetime,
            condition,
            action,
        }
    }
}

#[cfg(test)]
mod tests {

    
}

