use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName, MasterCard};

#[derive(Debug)]
pub struct AscendersBane;

impl Card for AscendersBane {
    fn name(&self) -> CardName {
        CardName::AscendersBane
    }

    fn get_type(&self) -> CardType {
        CardType::Curse
    }

    fn is_playable(&self, _: &Vec<MasterCard>) -> bool {
        false
    }

    fn exhausts(&self) -> bool {
        true
    }

    fn is_ethereal(&self) -> bool {
        true
    }

    fn can_be_removed(&self) -> bool {
        false
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
