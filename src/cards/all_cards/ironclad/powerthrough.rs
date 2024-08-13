use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

#[derive(Debug)]
pub struct PowerThrough(pub bool);

impl Card for PowerThrough {
    fn name(&self) -> CardName {
        CardName::PowerThrough
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
    }

    fn targets(&self) -> bool {
        false
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Block(Number(20)),
            CardActions::AddFreshCardToHand((CardName::Wound, false)),
            CardActions::AddFreshCardToHand((CardName::Wound, false)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Block(Number(15)),
            CardActions::AddFreshCardToHand((CardName::Wound, false)),
            CardActions::AddFreshCardToHand((CardName::Wound, false)),
        ]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
