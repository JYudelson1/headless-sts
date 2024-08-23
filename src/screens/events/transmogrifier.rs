use super::event_effects::EventAction;

pub fn actions() -> Vec<EventAction> {
    vec![EventAction::Leave, EventAction::Transform]
}
