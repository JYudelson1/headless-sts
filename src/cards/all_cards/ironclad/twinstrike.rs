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
pub struct TwinStrike(pub bool);

impl Card for TwinStrike {
    fn name(&self) -> CardName {
        CardName::TwinStrike
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(7), Targets::One)),
            CardActions::Damage((Number(7), Targets::One)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(5), Targets::One)),
            CardActions::Damage((Number(5), Targets::One)),
        ]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
