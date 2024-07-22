use enemy_trait::Enemy;
use rand::Rng;

use crate::{
    effects::{Buff, Effects},
    utils::Number,
};
mod enemy_trait;

mod jawworm;

struct VisibleEnemy {
    enemy_type: EnemyType,
    effects: Effects,
    intent: VisibleIntent,
    max_hp: u16,
    current_hp: u16,
}

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

impl EnemyIntent {
    pub fn to_visible_intent(&self) -> VisibleIntent {
        match self {
            EnemyIntent::Damage(amt) => VisibleIntent::Damage(amt.0.try_into().unwrap()),
            EnemyIntent::Block(_) => VisibleIntent::Block,
            EnemyIntent::Buff(_) => VisibleIntent::BuffSelf,
            EnemyIntent::Stun => VisibleIntent::Stun,
            EnemyIntent::Sleep => VisibleIntent::Sleep,
            EnemyIntent::AttackAndBlock(attack, _) => {
                VisibleIntent::AttackAndBlock(attack.0.try_into().unwrap())
            }
            EnemyIntent::BuffAndBlock(_, _) => VisibleIntent::BlockAndBuff,
        }
    }
}

pub enum VisibleIntent {
    Damage(u16),
    Block,
    BuffSelf,
    Stun,
    Sleep,
    SmallDebuff,
    LargeDebuff,
    AttackAndDebuff(u16),
    AttackAndBlock(u16),
    BlockAndBuff,
    RunAway,
    Unknown,
}

pub enum EnemyType {
    JawWorm,
    Louse,
}

fn get_starting_health(min: u16, max: u16) -> u16 {
    rand::thread_rng().gen_range(min..max + 1)
}
