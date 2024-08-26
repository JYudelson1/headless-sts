use crate::{
    effects::Effects,
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct SpikeSlimeS {
    intent: EnemyIntent,
}

impl Enemy for SpikeSlimeS {
    fn next_intent(&mut self, _: u8) {}

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}
impl SpikeSlimeS {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(11, 15)
        } else {
            number_between(10, 14)
        };

        let first_attack = if ascension >= 2 {
            EnemyIntent::Damage(Number(6))
        } else {
            EnemyIntent::Damage(Number(5))
        };

        let slime = SpikeSlimeS {
            intent: first_attack,
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(slime),
            enemy_type: EnemyType::SpikeSlimeS,
            ascension: ascension,
        }
    }
}
