use crate::{
    effects::{Buff, Effects, IntensityBuffs},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug)]
pub struct Cultist {
    intent: EnemyIntent,
}

impl Enemy for Cultist {
    fn next_intent(&mut self, _: u8) {
        self.intent = EnemyIntent::Damage(Number(6));
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }
}

impl Cultist {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(50, 56)
        } else {
            number_between(48, 54)
        };

        let ritual_amt = if ascension >= 17 {
            5
        } else if ascension >= 2 {
            4
        } else {
            3
        };
        let starting_buff = EnemyIntent::Buff(Buff::Intensity((
            IntensityBuffs::Ritual,
            Number(ritual_amt),
        )));

        let cultist = Cultist {
            intent: starting_buff,
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(cultist),
            enemy_type: EnemyType::Cultist,
            ascension: ascension,
        }
    }
}
