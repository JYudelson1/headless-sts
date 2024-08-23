use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    utils::Number,
};

#[derive(Debug)]
pub struct PerfectedStrike(pub bool);

impl Card for PerfectedStrike {
    fn name(&self) -> CardName {
        CardName::PerfectedStrike
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
        vec![CardActions::PerfectedStrike(Number(3))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::PerfectedStrike(Number(2))]
    }

    fn get_cost(&self) -> u8 {
        2
    }
}
