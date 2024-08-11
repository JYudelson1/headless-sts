pub use enemy_trait::Enemy;

use crate::{
    cards::{CardName, Pile},
    effects::{Buff, Debuff, Effects},
    utils::Number,
};
mod enemy_trait;

pub mod bosses;
pub mod elites;
pub mod normal;

#[derive(Debug)]
pub struct ConcreteEnemy {
    pub effects: Effects,
    pub max_hp: u16,
    pub current_hp: u16,
    pub current_block: Number,
    inner: Box<dyn Enemy>,
    pub enemy_type: EnemyType,
    ascension: u8,
}

impl ConcreteEnemy {
    pub fn is_dead(&self) -> bool {
        self.current_hp == 0
    }

    pub fn get_intent(&self) -> EnemyIntent {
        self.inner.get_current_intent()
    }

    pub fn next_intent(&mut self) {
        self.inner.next_intent(self.ascension)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnemyIndex(pub usize);

#[derive(Clone, Debug)]
pub enum EnemyIntent {
    Damage(Number),
    MultiAttack((Number, u16)),
    Block(Number),
    Buff(Buff),
    Debuff(Debuff),
    Stun,
    Sleep,
    AttackAndBlock(Number, Number),
    BuffAndBlock(Buff, Number),
    ShuffleCardToPile(CardName, Pile, bool),
    Multiple(Vec<EnemyIntent>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    JawWorm,
    GreenLouse,
    RedLouse,
    Cultist,
    AcidSlimeL,
    SpikeSlimeL,
    AcidSlimeM,
    SpikeSlimeM,
    AcidSlimeS,
    SpikeSlimeS,
    Looter,
    FungusBeast,
    GremlinFat,
    GremlinSneaky,
    GremlinMad,
    GremlinShield,
    GremlinWizard,
    SlaverBlue,
    SlaverRed,
    Lagavulin,
    GremlinNob,
    SentryA,
    SentryB,
    Hexaghost,
}
