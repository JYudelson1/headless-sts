use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

pub struct ShrugItOff(pub bool);

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

    fn upgrade(&mut self) {
        self.0 = true;
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Block(Number(11)), CardActions::Draw(1)]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Block(Number(8)), CardActions::Draw(1)]
    }
}
