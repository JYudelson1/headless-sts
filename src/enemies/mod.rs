pub use enemy_trait::Enemy;
use rand::Rng;

use crate::{
    effects::{Buff, Effects},
    utils::Number,
};
mod enemy_trait;

pub mod jawworm;

#[derive(Debug)]
pub struct ConcreteEnemy {
    pub effects: Effects,
    pub max_hp: u16,
    pub current_hp: u16,
    pub current_block: Number,
    inner: Box<dyn Enemy>,
    pub enemy_type: EnemyType,
}

impl ConcreteEnemy {
    pub fn is_dead(&self) -> bool {
        self.current_hp == 0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnemyIndex(pub usize);

#[derive(Copy, Clone, Debug)]
pub enum EnemyIntent {
    Damage(Number),
    MultiAttack((Number, u16)),
    Block(Number),
    Buff(Buff),
    Stun,
    Sleep,
    AttackAndBlock(Number, Number),
    BuffAndBlock(Buff, Number),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    JawWorm,
    Louse,
}
