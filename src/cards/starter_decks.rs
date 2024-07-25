use crate::{cards::{all_cards::make_card, CardName}, utils::Character};

use super::MasterCard;

fn starter_ironclad() -> Vec<MasterCard> {
    vec![
        make_card(CardName::Strike, false),
        make_card(CardName::Strike, false),
        make_card(CardName::Strike, false),
        make_card(CardName::Strike, false),
        make_card(CardName::Strike, false),
        make_card(CardName::Defend, false),
        make_card(CardName::Defend, false),
        make_card(CardName::Defend, false),
        make_card(CardName::Defend, false),
        make_card(CardName::Bash, false),
    ]
}
fn starter_silent() -> Vec<MasterCard> {
    todo!()
}

fn starter_defect() -> Vec<MasterCard> {
    todo!()
}

fn starter_watcher() -> Vec<MasterCard> {
    todo!()
}

pub fn make_starter_deck(character: Character) -> Vec<MasterCard> {
    match character {
        Character::Ironclad => starter_ironclad(),
        Character::Silent => starter_silent(),
        Character::Defect => starter_defect(),
        Character::Watcher => starter_watcher(),
    }
}
