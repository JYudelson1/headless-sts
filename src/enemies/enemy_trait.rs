use rand::Rng;

use crate::effects::Effects;

use super::{jawworm::JawWormConstructor, EnemyIntent, EnemyType};

pub trait Enemy {
    fn next_intent(&mut self, ascension: u8);

    fn get_enemy_type(&self) -> EnemyType;
    fn get_effects(&mut self) -> &mut Effects;
    fn get_current_intent(&self) -> EnemyIntent;
    fn get_max_hp(&self) -> u16;
    fn get_current_hp(&self) -> u16;
    fn get_current_block(&self) -> u16;
}

pub trait EnemyConstructor<E: Enemy> {
    fn new(ascension: u8) -> E;
}

impl EnemyType {
    pub fn new(&self, ascension: u8) -> Box<dyn Enemy> {
        match self {
            EnemyType::JawWorm => Box::new(JawWormConstructor::new(ascension)),
            EnemyType::Louse => todo!(),
        }
    }
}
