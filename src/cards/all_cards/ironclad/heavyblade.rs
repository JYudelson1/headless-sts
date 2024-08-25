use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

#[derive(Debug)]
pub struct HeavyBlade(pub bool);

impl Card for HeavyBlade {
    fn name(&self) -> CardName {
        CardName::HeavyBlade
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

    fn is_a_strike(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::HeavyBlade(Number(5))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::HeavyBlade(Number(3))]
    }

    fn get_cost(&self) -> u8 {
        2
    }
}
