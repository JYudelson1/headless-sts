use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct Dazed;

impl Card for Dazed {
    fn name(&self) -> CardName {
        CardName::Dazed
    }

    fn get_type(&self) -> CardType {
        CardType::Status
    }

    fn is_playable(&self) -> bool {
        false
    }

    fn exhausts(&self) -> bool {
        true
    }

    fn is_ethereal(&self) -> bool {
        true
    }

    fn set_upgraded(&mut self, _: bool) {}

    fn can_be_upgraded(&self) -> bool {
        false
    }

    fn is_upgraded(&self) -> bool {
        false
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
