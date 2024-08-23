use crate::{cards::CardName, state::State};

use super::event_effects::EventAction;

pub fn actions(state: &State) -> Vec<EventAction> {
    let gold = if state.ascension >= 15 { 150 } else { 175 };
    vec![
        EventAction::Leave,
        EventAction::Multiple(vec![
            EventAction::GainGold(gold),
            EventAction::GainBlankCard(CardName::Doubt),
        ]),
    ]
}
