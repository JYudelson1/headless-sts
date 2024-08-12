use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    effects::{Debuff, DurationDebuffs},
    utils::Number,
};

#[derive(Debug)]
pub struct Berserk(pub bool);

impl Card for Berserk {
    fn name(&self) -> CardName {
        CardName::Berserk
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
        vec![
            CardActions::ApplyDebuff(Debuff::Duration((DurationDebuffs::Vulnerable, Number(1)))),
            CardActions::IncreaseMaxEnergy,
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::ApplyDebuff(Debuff::Duration((DurationDebuffs::Vulnerable, Number(2)))),
            CardActions::IncreaseMaxEnergy,
        ]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
