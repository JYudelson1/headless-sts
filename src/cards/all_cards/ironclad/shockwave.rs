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
pub struct Shockwave(pub bool);

impl Card for Shockwave {
    fn name(&self) -> CardName {
        CardName::Shockwave
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

    fn exhausts(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::ApplyWeak((Number(5), Targets::All)),
            CardActions::ApplyVulnerable((Number(5), Targets::All)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::ApplyWeak((Number(3), Targets::All)),
            CardActions::ApplyVulnerable((Number(3), Targets::All)),
        ]
    }

    fn get_cost(&self) -> u8 {
        2
    }
}
