use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    effects::{Buff, IntensityBuffs},
    utils::Number,
};

#[derive(Debug)]
pub struct Metallicize(pub bool);

impl Card for Metallicize {
    fn name(&self) -> CardName {
        CardName::Metallicize
    }

    fn get_type(&self) -> CardType {
        CardType::Power
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
        vec![CardActions::ApplyBuff(Buff::Intensity((
            IntensityBuffs::Metallicize,
            Number(4),
        )))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ApplyBuff(Buff::Intensity((
            IntensityBuffs::Metallicize,
            Number(3),
        )))]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
