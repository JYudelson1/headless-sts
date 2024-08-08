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
pub struct SwordBoomerang(pub bool);

impl Card for SwordBoomerang {
    fn name(&self) -> CardName {
        CardName::SwordBoomerang
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
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
            CardActions::Damage((Number(3), Targets::Random)),
            CardActions::Damage((Number(3), Targets::Random)),
            CardActions::Damage((Number(3), Targets::Random)),
            CardActions::Damage((Number(3), Targets::Random)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(3), Targets::Random)),
            CardActions::Damage((Number(3), Targets::Random)),
            CardActions::Damage((Number(3), Targets::Random)),
        ]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
