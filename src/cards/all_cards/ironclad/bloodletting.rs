use crate::cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName};

#[derive(Debug)]
pub struct Bloodletting(pub bool);

impl Card for Bloodletting {
    fn name(&self) -> CardName {
        CardName::Bloodletting
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
        vec![CardActions::LoseHealth(3), CardActions::GainEnergy(3)]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::LoseHealth(3), CardActions::GainEnergy(2)]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
