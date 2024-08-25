use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::Card,
        CardName,
    },
    combat::CardInHandPurpose,
    utils::Number,
};

#[derive(Clone, Debug)]

pub struct WarCry(pub bool);

impl Card for WarCry {
    fn name(&self) -> CardName {
        CardName::WarCry
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
            CardActions::Draw(2),
            CardActions::ChooseNCards((CardInHandPurpose::PutOnTopOfDeck, 1, None)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Damage((Number(5), Targets::One)),
            CardActions::ChooseNCards((CardInHandPurpose::PutOnTopOfDeck, 1, None)),
        ]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
