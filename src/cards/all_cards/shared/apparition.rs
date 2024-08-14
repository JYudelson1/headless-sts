use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    effects::{Buff, DurationBuffs},
    utils::Number,
};

#[derive(Debug)]
pub struct Apparition(pub bool);

impl Card for Apparition {
    fn name(&self) -> CardName {
        CardName::Apparition
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
    }

    fn is_playable(&self) -> bool {
        true
    }

    fn exhausts(&self) -> bool {
        true
    }

    fn is_ethereal(&self) -> bool {
        !self.0
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ApplyBuff(Buff::Duration((
            DurationBuffs::Intangible,
            Number(1),
        )))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ApplyBuff(Buff::Duration((
            DurationBuffs::Intangible,
            Number(1),
        )))]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
