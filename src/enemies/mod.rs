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

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConcreteEnemy {
    pub effects: Effects,
    pub max_hp: u16,
    pub current_hp: u16,
    pub current_block: Number,
    inner: InnerEnemy,
    pub enemy_type: EnemyType,
    ascension: u8,
}

impl ConcreteEnemy {
    pub fn is_dead(&self) -> bool {
        self.current_hp == 0
    }

    pub fn get_intent(&self) -> EnemyIntent {
        self.inner.inner().get_current_intent()
    }

    pub fn next_intent(&mut self) {
        self.inner.inner_mut().next_intent(self.ascension)
    }

    pub fn lost_hp(&mut self) {
        // This is ONLY for changing intents based on enemies losing HP
        // Maybe only effects Lagavulin?
        self.inner.inner_mut().lost_hp();
    }
}

impl Clone for ConcreteEnemy {
    fn clone(&self) -> Self {
        Self {
            effects: self.effects.clone(),
            max_hp: self.max_hp.clone(),
            current_hp: self.current_hp.clone(),
            current_block: self.current_block.clone(),
            inner: self.inner.clone(),
            enemy_type: self.enemy_type.clone(),
            ascension: self.ascension.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct EnemyIndex(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    UpgradeAllBurns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    TheGuardian,
    SlimeBoss,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InnerEnemy {
    JawWorm(normal::jawworm::JawWorm)    ,
    GreenLouse(normal::greenlouse::GreenLouse),
    RedLouse(normal::redlouse::RedLouse),
    Cultist(normal::cultist::Cultist),
    //AcidSlimeL(normal::acid_slime_l::AcidSlimeL),
   // SpikeSlimeL(normal::spike_slime_l::SpikeSlimeL),
    AcidSlimeM(normal::acid_slime_m::AcidSlimeM),
    SpikeSlimeM(normal::spike_slime_m::SpikeSlimeM),
    AcidSlimeS(normal::acid_slime_s::AcidSlimeS),
    SpikeSlimeS(normal::spike_slime_s::SpikeSlimeS),
    //Looter(normal::looter::Looter),
    FungusBeast(normal::fungus_beast::FungusBeast),
    GremlinFat(normal::gremlin_fat::GremlinFat),
    GremlinSneaky(normal::gremlin_sneaky::GremlinSneaky),
    //GremlinMad(normal::gremlin_mad::GremlinMad),
    //GremlinShield(normal::gremlin_shield::GremlinShield),
    //GremlinWizard(normal::gremlin_wizard::GremlinWizard),
    SlaverBlue(normal::slaver_blue::SlaverBlue),
    SlaverRed(normal::slaver_red::SlaverRed),
    Lagavulin(elites::lagavulin::Lagavulin),
    //GremlinNob(elites::gremlin_nob::GremlinNob),
    SentryA(elites::sentries::SentryA),
    SentryB(elites::sentries::SentryB),
    Hexaghost(bosses::hexaghost::Hexaghost),
    //TheGuardian(bosses::the_guardian::TheGuardian),
    //SlimeBoss(bosses::slime_boss::SlimeBoss),
}

impl InnerEnemy {
    pub fn inner(&self) -> &dyn Enemy {
        match self {
            InnerEnemy::JawWorm(enemy) => enemy,
            InnerEnemy::GreenLouse(enemy) => enemy,
            InnerEnemy::RedLouse(enemy) => enemy,
            InnerEnemy::Cultist(enemy) => enemy,
            //InnerEnemy::AcidSlimeL(enemy) => enemy,
            //InnerEnemy::SpikeSlimeL(enemy) => enemy,
            InnerEnemy::AcidSlimeM(enemy) => enemy,
            InnerEnemy::SpikeSlimeM(enemy) => enemy,
            InnerEnemy::AcidSlimeS(enemy) => enemy,
            InnerEnemy::SpikeSlimeS(enemy) => enemy,
            //InnerEnemy::Looter(enemy) => enemy,
            InnerEnemy::FungusBeast(enemy) => enemy,
            InnerEnemy::GremlinFat(enemy) => enemy,
            InnerEnemy::GremlinSneaky(enemy) => enemy,
            //InnerEnemy::GremlinMad(enemy) => enemy,
            //InnerEnemy::GremlinShield(enemy) => enemy,
            //InnerEnemy::GremlinWizard(enemy) => enemy,
            InnerEnemy::SlaverBlue(enemy) => enemy,
            InnerEnemy::SlaverRed(enemy) => enemy,
            InnerEnemy::Lagavulin(enemy) => enemy,
            //InnerEnemy::GremlinNob(enemy) => enemy,
            InnerEnemy::SentryA(enemy) => enemy,
            InnerEnemy::SentryB(enemy) => enemy,
            InnerEnemy::Hexaghost(enemy) => enemy,
            //InnerEnemy::TheGuardian(enemy) => enemy,
            //InnerEnemy::SlimeBoss(enemy) => enemy,
        }
    }

    pub fn inner_mut(&mut self) -> &mut dyn Enemy {
        match self {
            InnerEnemy::JawWorm(enemy) => enemy,
            InnerEnemy::GreenLouse(enemy) => enemy,
            InnerEnemy::RedLouse(enemy) => enemy,
            InnerEnemy::Cultist(enemy) => enemy,
            //InnerEnemy::AcidSlimeL(enemy) => enemy,
            //InnerEnemy::SpikeSlimeL(enemy) => enemy,
            InnerEnemy::AcidSlimeM(enemy) => enemy,
            InnerEnemy::SpikeSlimeM(enemy) => enemy,
            InnerEnemy::AcidSlimeS(enemy) => enemy,
            InnerEnemy::SpikeSlimeS(enemy) => enemy,
            //InnerEnemy::Looter(enemy) => enemy,
            InnerEnemy::FungusBeast(enemy) => enemy,
            InnerEnemy::GremlinFat(enemy) => enemy,
            InnerEnemy::GremlinSneaky(enemy) => enemy,
            //InnerEnemy::GremlinMad(enemy) => enemy,
            //InnerEnemy::GremlinShield(enemy) => enemy,
            //InnerEnemy::GremlinWizard(enemy) => enemy,
            InnerEnemy::SlaverBlue(enemy) => enemy,
            InnerEnemy::SlaverRed(enemy) => enemy,
            InnerEnemy::Lagavulin(enemy) => enemy,
            //InnerEnemy::GremlinNob(enemy) => enemy,
            InnerEnemy::SentryA(enemy) => enemy,
            InnerEnemy::SentryB(enemy) => enemy,
            InnerEnemy::Hexaghost(enemy) => enemy,
            //InnerEnemy::TheGuardian(enemy) => enemy,
            //InnerEnemy::SlimeBoss(enemy) => enemy,
        }
    }
}