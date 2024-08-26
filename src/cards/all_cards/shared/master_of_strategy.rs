use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct MasterOfStrategy(pub bool);

impl Card for MasterOfStrategy {
    fn name(&self) -> CardName {
        CardName::MasterOfStrategy
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

    fn exhausts(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Draw(4)]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Draw(3)]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
