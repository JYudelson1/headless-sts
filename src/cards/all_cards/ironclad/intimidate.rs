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

pub struct Intimidate(pub bool);

impl Card for Intimidate {
    fn name(&self) -> CardName {
        CardName::Intimidate
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

    fn exhausts(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ApplyWeak((Number(2), Targets::All))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ApplyWeak((Number(1), Targets::All))]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
