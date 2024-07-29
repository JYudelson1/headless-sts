use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

#[derive(Debug)]
pub struct SeeingRed(pub bool);

impl Card for SeeingRed {
    fn name(&self) -> CardName {
        CardName::SeeingRed
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
        vec![CardActions::GainEnergy(2)]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::GainEnergy(2)]
    }

    fn get_cost(&self) -> u8 {
        if self.0 {
            0
        } else {
            1
        }
    }
}
