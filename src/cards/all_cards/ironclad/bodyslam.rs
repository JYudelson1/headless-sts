use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

pub struct BodySlam(pub bool);

impl Card for BodySlam {
    fn name(&self) -> CardName {
        CardName::BodySlam
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
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
        vec![CardActions::BodySlam]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::BodySlam]
    }

    fn get_cost(&self) -> u8 {
        if self.0 {
            0
        } else {
            1
        }
    }
}
