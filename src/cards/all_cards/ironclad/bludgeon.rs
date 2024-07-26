use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    utils::Number,
};

pub struct Bludgeon(pub bool);

impl Card for Bludgeon {
    fn name(&self) -> CardName {
        CardName::Bludgeon
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
        vec![CardActions::Damage((Number(42), Targets::One))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Damage((Number(32), Targets::One))]
    }

    fn get_cost(&self) -> u8 {
        3
    }
}
