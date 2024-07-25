use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

pub struct Defend;

impl Card for Defend {
    fn name(&self) -> CardName {
        CardName::Defend
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
    }

    fn targets(&self) -> bool {
        false
    }

    fn play(&mut self) -> Vec<CardActions> {
        vec![CardActions::Block(Number(5))]
    }
}
