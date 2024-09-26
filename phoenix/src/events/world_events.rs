use crate::{common::IdGarbageCollector, events::EventLifetime};
use std::collections::HashMap;

use super::{action::Action, condition::Condition, Event};

type ID = u32;

#[derive(Default)]
pub struct WorldEvents {
    events: HashMap<ID, Event>,
    high_priority_events: HashMap<ID, Event>,
    id_gc: IdGarbageCollector,
}

impl WorldEvents {
    pub fn add(&mut self, event: Event) {
        self.events.insert(self.id_gc.create_id(), event);
    }

    pub fn add_high_priority(&mut self, event: Event) {
        self.high_priority_events
            .insert(self.id_gc.create_id(), event);
    }

    pub fn process_events(&mut self, input_actions: &mut Vec<Action>) -> Vec<Action> {
        let mut result = Vec::new();
        let mut once_events: Vec<ID> = Vec::new();

        for (id, event) in &self.events {
            if WorldEvents::is_condition_met(event, input_actions) {
                result.push(event.action.clone());
                if event.lifetime == EventLifetime::Once {
                    once_events.push(*id);
                }
            }
        }

        for (id, event) in &self.high_priority_events {
            if WorldEvents::is_condition_met(event, input_actions) {
                result.push(event.action.clone());
                if event.lifetime == EventLifetime::Once {
                    once_events.push(*id);
                }
            }
        }

        result.append(input_actions);

        once_events.iter().for_each(|id| self.id_gc.remove_id(*id));
        once_events.into_iter().for_each(|id| self.remove_event(id));

        result
    }

    fn is_condition_met(event: &Event, input_actions: &[Action]) -> bool {
        match &event.condition {
            Condition::OnAction(action) => input_actions.contains(action),
        }
    }

    fn remove_event(&mut self, id: ID) {
        self.events.remove(&id);
    }
}
