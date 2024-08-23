use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct Havoc(pub bool);

impl Card for Havoc {
    fn name(&self) -> CardName {
        CardName::Havoc
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

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Havoc]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Havoc]
    }

    fn get_cost(&self) -> u8 {
        if self.0 {
            0
        } else {
            1
        }
    }
}
