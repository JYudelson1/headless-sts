use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    utils::Number,
};

pub struct Bash;

impl Card for Bash {
    fn name(&self) -> CardName {
        CardName::Bash
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
    }

    fn play(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(8), Targets::One)),
            CardActions::ApplyVulnerable((Number(2), Targets::One)),
        ]
    }
}
