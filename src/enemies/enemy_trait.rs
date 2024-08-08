use std::fmt::Debug;

use super::{cultist::Cultist, jawworm::JawWorm, ConcreteEnemy, EnemyIntent, EnemyType};

pub trait Enemy: Debug {
    fn next_intent(&mut self, ascension: u8);

    fn get_current_intent(&self) -> EnemyIntent;
}

impl EnemyType {
    pub fn new(&self, ascension: u8) -> ConcreteEnemy {
        match self {
            EnemyType::JawWorm => JawWorm::new(ascension),
            EnemyType::Louse => todo!(),
            EnemyType::Cultist => Cultist::new(ascension),
        }
    }
}
