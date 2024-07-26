use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct Void;

impl Card for Void {
    fn name(&self) -> CardName {
        CardName::Void
    }

    fn get_type(&self) -> CardType {
        CardType::Status
    }

    fn is_playable(&self) -> bool {
        false
    }

    fn play(&mut self) -> Vec<CardActions> {
        vec![]
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
