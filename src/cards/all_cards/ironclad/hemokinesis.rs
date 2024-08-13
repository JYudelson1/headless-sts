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

pub struct HemoKinesis(pub bool);

impl Card for HemoKinesis {
    fn name(&self) -> CardName {
        CardName::HemoKinesis
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
            CardActions::LoseHealth(2),
            CardActions::Damage((Number(15), Targets::One)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::LoseHealth(2),
            CardActions::Damage((Number(10), Targets::One)),
        ]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
