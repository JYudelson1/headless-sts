use crate::{cards::CardName, state::State};

use super::event_effects::EventAction;

pub fn actions(state: &State) -> Vec<EventAction> {
    let one_third_hp = (state.current_health as f32 / 3.0).floor() as u16;
    vec![
        EventAction::Heal(one_third_hp),
        EventAction::GainMaxHp(5),
        EventAction::Multiple(vec![
            EventAction::RandomRelic,
            EventAction::GainBlankCard(CardName::Regret),
        ]),
    ]
}
