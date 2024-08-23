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
pub struct Clash(pub bool);

impl Card for Clash {
    fn name(&self) -> CardName {
        CardName::Clash
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

    fn is_playable(&self, hand: &Vec<crate::cards::MasterCard>) -> bool {
        for card in hand {
            if card.card().get_type() != CardType::Attack {
                return false;
            }
        }
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Damage((Number(18), Targets::One))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Damage((Number(14), Targets::One))]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
