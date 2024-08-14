use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct Burn(pub bool);

impl Card for Burn {
    fn name(&self) -> CardName {
        CardName::Burn
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

    fn set_upgraded(&mut self, upgraded: bool) {
        self.0 = upgraded
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
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
