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
pub struct WildStrike(pub bool);

impl Card for WildStrike {
    fn name(&self) -> CardName {
        CardName::WildStrike
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
    fn is_a_strike(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        let wound = make_card(CardName::Wound, false).unwrap();
        vec![
            CardActions::Damage((Number(17), Targets::One)),
            CardActions::ShuffleCardToPile((wound, Pile::Draw)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        let wound = make_card(CardName::Wound, false).unwrap();
        vec![
            CardActions::Damage((Number(12), Targets::One)),
            CardActions::ShuffleCardToPile((wound, Pile::Draw)),
        ]
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn get_cost(&self) -> u8 {
        1
    }
}
