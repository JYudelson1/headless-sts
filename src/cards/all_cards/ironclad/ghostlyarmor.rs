use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

pub struct GhostlyArmor(pub bool);

impl Card for GhostlyArmor {
    fn name(&self) -> CardName {
        CardName::GhostlyArmor
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
    }

    fn targets(&self) -> bool {
        false
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn is_ethereal(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Block(Number(13))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Block(Number(10))]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
