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

    /// Returns actions that need to be processed by the scene
    pub fn collect_world_events(&mut self, input_actions: &mut Vec<Action>) -> Vec<Action> {
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
            Condition::None => true,
        }
    }

    fn remove_event(&mut self, id: ID) {
        self.events.remove(&id);
        self.high_priority_events.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use crate::events::{action::Action, condition::Condition, Event, EventLifetime};

    use super::WorldEvents;

    #[test]
    fn test_world_events_new() {
        let mut world_events = WorldEvents::default();

        world_events.add(Event::new(
            EventLifetime::PerFrame,
            Condition::OnAction(Action::CameraUpdateRight),
            Action::CameraUpdateForward,
        ));

        let mut actions = Vec::new();
        let result = world_events.collect_world_events(&mut actions);

        assert!(result.is_empty());
    }

    #[test]
    fn test_world_events_new_action() {
        let mut world_events = WorldEvents::default();

        world_events.add(Event::new(
            EventLifetime::PerFrame,
            Condition::OnAction(Action::CameraUpdateRight),
            Action::CameraUpdateForward,
        ));

        let mut actions = vec![Action::CameraUpdateRight];
        let result = world_events.collect_world_events(&mut actions);

        assert!(result.contains(&Action::CameraUpdateRight));
        assert!(result.contains(&Action::CameraUpdateForward));
    }

    #[test]
    fn test_world_events_new_action_once() {
        let mut world_events = WorldEvents::default();

        world_events.add(Event::new(
            EventLifetime::Once,
            Condition::OnAction(Action::CameraUpdateRight),
            Action::CameraUpdateForward,
        ));

        let mut actions = vec![Action::CameraUpdateRight];
        let result = world_events.collect_world_events(&mut actions);

        assert!(result.contains(&Action::CameraUpdateRight));
        assert!(result.contains(&Action::CameraUpdateForward));

        actions = vec![Action::CameraUpdateRight];
        let result = world_events.collect_world_events(&mut actions);
        assert_eq!(result, vec![Action::CameraUpdateRight]);
    }

    #[test]
    fn test_world_events_multiple_actions() {
        let mut world_events = WorldEvents::default();

        world_events.add(Event::new(
            EventLifetime::Once,
            Condition::OnAction(Action::CameraUpdateRight),
            Action::CameraUpdateForward,
        ));

        world_events.add_high_priority(Event::new(
            EventLifetime::Once,
            Condition::OnAction(Action::CameraUpdateRight),
            Action::CameraUpdateBackward,
        ));

        world_events.add_high_priority(Event::new(
            EventLifetime::PerFrame,
            Condition::OnAction(Action::CameraUpdateForward),
            Action::CameraUpdateLeft,
        ));

        let mut actions = vec![Action::CameraUpdateRight];
        let result = world_events.collect_world_events(&mut actions);

        assert_eq!(result.len(), 3);
        // first user input
        assert_eq!(result[2], Action::CameraUpdateRight);
        // second high priority
        assert_eq!(result[1], Action::CameraUpdateBackward);
        // rest of actions
        assert_eq!(result[0], Action::CameraUpdateForward);

        actions = vec![Action::CameraUpdateForward];
        let result = world_events.collect_world_events(&mut actions);

        // first user input
        assert_eq!(result.len(), 2);
        assert_eq!(result[1], Action::CameraUpdateForward);
        assert_eq!(result[0], Action::CameraUpdateLeft);
    }
}
