use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName, Targets},
    effects::{Debuff, IntensityBuffOrDebuff},
    utils::Number,
};

#[derive(Debug)]
pub struct Disarm(pub bool);

impl Card for Disarm {
    fn name(&self) -> CardName {
        CardName::Disarm
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
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

    fn exhausts(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::DebuffEnemy((Debuff::Basic((
            IntensityBuffOrDebuff::Strength,
            Number(3),
        )), Targets::One))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::DebuffEnemy((Debuff::Basic((
            IntensityBuffOrDebuff::Strength,
            Number(2),
        )), Targets::One))]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
