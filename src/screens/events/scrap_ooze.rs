use crate::state::State;

use super::event_effects::EventAction;

pub fn actions(state: &State, amt: usize) -> Vec<EventAction> {
    let prob = 25 + amt * 10;
    let mut hp_loss = if state.ascension >= 15 { 5 } else { 3 };
    hp_loss += amt;

    let relic_or_again = EventAction::ChanceForAction((
        Box::new(EventAction::RandomRelic),
        prob as u8,
        Some(Box::new(EventAction::GoToScrapOoze(amt + 1))),
    ));
    let action = EventAction::Multiple(vec![EventAction::LoseHp(hp_loss as u16), relic_or_again]);

    vec![EventAction::Leave, action]
}
