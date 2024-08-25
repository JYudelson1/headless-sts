use crate::{cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName}, combat::CardInHandPurpose};

#[derive(Debug)]
pub struct BurningPact(pub bool);

impl Card for BurningPact {
    fn name(&self) -> CardName {
        CardName::BurningPact
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
        vec![CardActions::ChooseNCards((CardInHandPurpose::Exhaust, 1)), CardActions::Draw(3)]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::ChooseNCards((CardInHandPurpose::Exhaust, 1)), CardActions::Draw(2)]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
