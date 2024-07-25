use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    utils::Number,
};

#[derive(Clone)]

pub struct Strike;

impl Card for Strike {
    fn name(&self) -> CardName {
        CardName::Strike
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
    }

    fn play(&mut self) -> Vec<CardActions> {
        vec![CardActions::Damage((Number(6), Targets::One))]
    }
}
