use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

pub struct Armaments(pub bool);

impl Card for Armaments {
    fn name(&self) -> CardName {
        CardName::Armaments
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

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Block(Number(5)),
            CardActions::UpgradeAllCardsInHand,
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Block(Number(5)),
            CardActions::UpgradeACardInHand,
        ]
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
