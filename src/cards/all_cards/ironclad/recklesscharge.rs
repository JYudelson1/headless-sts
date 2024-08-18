use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        make_card, CardName, Pile,
    },
    utils::Number,
};

#[derive(Debug)]
pub struct RecklessCharge(pub bool);

impl Card for RecklessCharge {
    fn name(&self) -> CardName {
        CardName::RecklessCharge
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        let dazed = make_card(CardName::Dazed, false).unwrap();
        vec![
            CardActions::Damage((Number(10), Targets::One)),
            CardActions::ShuffleCardToPile((dazed, Pile::Draw)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        let dazed = make_card(CardName::Dazed, false).unwrap();
        vec![
            CardActions::Damage((Number(7), Targets::One)),
            CardActions::ShuffleCardToPile((dazed, Pile::Draw)),
        ]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
