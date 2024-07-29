use crate::cards::make_card;
use crate::cards::Pile;
use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    utils::Number,
};

#[derive(Debug)]
pub struct Anger(pub bool);

impl Card for Anger {
    fn name(&self) -> CardName {
        CardName::Anger
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
            CardActions::Damage((Number(8), Targets::One)),
            CardActions::ShuffleCardToPile((
                make_card(self.name(), self.is_upgraded()),
                Pile::Discard,
            )),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(6), Targets::One)),
            CardActions::ShuffleCardToPile((
                make_card(self.name(), self.is_upgraded()),
                Pile::Discard,
            )),
        ]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
