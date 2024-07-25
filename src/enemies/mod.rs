pub use enemy_trait::{Enemy, EnemyConstructor};
use jawworm::JawWormConstructor;
use rand::Rng;

use crate::{
    effects::{Buff, Effects},
    utils::Number,
};
mod enemy_trait;

mod jawworm;

#[derive(Debug, PartialEq, Eq)]
pub struct EnemyIndex(pub usize);

#[derive(Copy, Clone)]
pub enum EnemyIntent {
    Damage(Number),
    Block(Number),
    Buff(Buff),
    Stun,
    Sleep,
    AttackAndBlock(Number, Number),
    BuffAndBlock(Buff, Number),
}

pub enum EnemyType {
    JawWorm,
    Louse,
}

fn get_starting_health(min: u16, max: u16) -> u16 {
    rand::thread_rng().gen_range(min..max + 1)
}
