use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    combat::CardInHandPurpose,
    utils::Number,
};

#[derive(Debug)]
pub struct TrueGrit(pub bool);

impl Card for TrueGrit {
    fn name(&self) -> CardName {
        CardName::TrueGrit
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
            CardActions::Block(Number(9)),
            CardActions::ChooseNCards((CardInHandPurpose::Exhaust, 1, None)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Block(Number(7)),
            CardActions::ExhaustRandomCard,
        ]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
