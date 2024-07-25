use crate::effects::Effects;

use super::{jawworm::JawWorm, ConcreteEnemy, EnemyIntent, EnemyType};

pub trait Enemy {
    fn next_intent(&mut self, ascension: u8);

    fn get_current_intent(&self) -> EnemyIntent;
}

impl EnemyType {
    pub fn new(&self, ascension: u8) -> ConcreteEnemy {
        match self {
            EnemyType::JawWorm => JawWorm::new(ascension),
            EnemyType::Louse => todo!(),
        }
    }
}
