use std::{cell::RefCell, rc::Rc};

use crate::{
    cards::{
        card::CardType,
        card_actions::{CardActions, Targets},
        card_trait::{Card, MasterCard},
        CardName,
    },
    utils::Number,
};

#[derive(Clone, Debug)]

pub struct SearingBlow(pub u16);

impl Card for SearingBlow {
    fn name(&self) -> CardName {
        CardName::SearingBlow
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
    }

    fn set_upgraded(&mut self, _: bool) {
        panic!()
    }

    fn upgrade(&mut self) {
        self.0 += 1;
    }

    fn set_upgraded_amt(&mut self, amt: u16) {
        self.0 = amt
    }

    fn can_be_upgraded(&self) -> bool {
        !true
    }

    fn is_upgraded(&self) -> bool {
        true
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![CardActions::Damage((self.get_damage(), Targets::One))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        panic!()
    }

    fn get_cost(&self) -> u8 {
        2
    }

    fn duplicate(&self) -> MasterCard {
        let inner = Self(self.0);
        let inner = Rc::new(RefCell::new(inner));

        MasterCard {
            card: inner,
            id: uuid::Uuid::new_v4(),
            upgraded: self.0,
        }
    }
}

impl SearingBlow {
    pub fn get_damage(&self) -> Number {
        let damage = (self.0 * (self.0 + 7) / 2) + 12;
        Number(damage as i16)
    }
}
