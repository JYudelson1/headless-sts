use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

pub struct Void;

impl Card for Void {
    fn name(&self) -> CardName {
        CardName::Void
    }

    fn get_type(&self) -> CardType {
        CardType::Curse
    }

    fn is_playable(&self) -> bool {
        false
    }

    fn play(&mut self) -> Vec<CardActions> {
        vec![]
    }
}
