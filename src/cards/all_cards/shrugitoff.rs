use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

pub struct ShrugItOff;

impl Card for ShrugItOff {
    fn name(&self) -> CardName {
        CardName::ShrugItOff
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
    }

    fn targets(&self) -> bool {
        false
    }

    fn play(&mut self) -> Vec<CardActions> {
        vec![CardActions::Block(Number(8)), CardActions::Draw(1)]
    }
}
