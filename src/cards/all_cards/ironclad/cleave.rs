use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    utils::Number,
};

#[derive(Debug)]
pub struct Cleave(pub bool);

impl Card for Cleave {
    fn name(&self) -> CardName {
        CardName::Cleave
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
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
        vec![CardActions::Damage((Number(11), Targets::All))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Damage((Number(8), Targets::All))]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
