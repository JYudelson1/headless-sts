use super::EventAction;

pub fn actions(max_hp: i16, ascension: u8) -> Vec<EventAction> {
    let gold = if ascension >= 15 { 50 } else { 75 };
    let hp_amt = (max_hp as f32 / 10.0).ceil() as u16;
    let first = EventAction::Multiple(vec![
        EventAction::GainGold(gold),
        EventAction::LoseHp(hp_amt),
    ]);

    vec![first, EventAction::GainRandomMask]
}
