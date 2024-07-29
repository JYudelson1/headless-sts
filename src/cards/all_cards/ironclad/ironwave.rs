use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    utils::Number,
};

#[derive(Clone, Debug)]

pub struct IronWave(pub bool);

impl Card for IronWave {
    fn name(&self) -> CardName {
        CardName::IronWave
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
        vec![
            CardActions::Damage((Number(7), Targets::One)),
            CardActions::Block(Number(7)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(5), Targets::One)),
            CardActions::Block(Number(5)),
        ]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
