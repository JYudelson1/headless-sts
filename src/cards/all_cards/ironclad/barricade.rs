use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    effects::{Buff, PermanentBoolBuffs},
};

#[derive(Debug)]
pub struct Barricade(pub bool);

impl Card for Barricade {
    fn name(&self) -> CardName {
        CardName::Barricade
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
        vec![CardActions::ApplyBuff(Buff::PermanentBool(PermanentBoolBuffs::Barricade))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ApplyBuff(Buff::PermanentBool(PermanentBoolBuffs::Barricade))]
    }

    fn get_cost(&self) -> u8 {
        if self.0 {
            2
        } else {
            3
        }
    }
}
