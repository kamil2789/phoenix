use super::action::Action;

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    OnAction(Action),
}
