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

pub struct Rampage {
    upgraded: bool,
    extra_damage: Number,
}

impl Card for Rampage {
    fn name(&self) -> CardName {
        CardName::Rampage
    }

    fn reset(&mut self) {
        self.extra_damage = Number(0)
    }

    fn get_type(&self) -> CardType {
        CardType::Attack
    }

    fn targets(&self) -> bool {
        true
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.upgraded = to_set;
    }

    fn can_be_upgraded(&self) -> bool {
        !self.upgraded
    }

    fn is_upgraded(&self) -> bool {
        self.upgraded
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        let current_damage = Number(8) + self.extra_damage;

        self.extra_damage += Number(8);

        vec![CardActions::Damage((current_damage, Targets::One))]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        let current_damage = Number(8) + self.extra_damage;

        self.extra_damage += Number(5);

        vec![CardActions::Damage((current_damage, Targets::One))]
    }

    fn get_cost(&self) -> u8 {
        1
    }

    fn duplicate(&self) -> MasterCard {
        let inner = Self {
            upgraded: self.upgraded,
            extra_damage: self.extra_damage,
        };
        let inner = Rc::new(RefCell::new(inner));

        MasterCard {
            card: inner,
            id: uuid::Uuid::new_v4(),
            upgraded: if self.upgraded { 1 } else { 0 },
        }
    }
}

impl Rampage {
    pub fn new(upgraded: bool) -> Self {
        Self {
            upgraded,
            extra_damage: Number(0),
        }
    }
}
