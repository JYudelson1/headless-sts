use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct Entrench(pub bool);

impl Card for Entrench {
    fn name(&self) -> CardName {
        CardName::Entrench
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
        vec![CardActions::DoubleBlock]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::DoubleBlock]
    }

    fn get_cost(&self) -> u8 {
        if self.0 {
            1
        } else {
            2
        }
    }
}
