use std::fmt::Debug;

use super::{
    elites::sentries::{SentryA, SentryB},
    normal::{
        acid_slime_m::AcidSlimeM, acid_slime_s::AcidSlimeS, cultist::Cultist, jawworm::JawWorm,
        spike_slime_s::SpikeSlimeS,
    },
    ConcreteEnemy, EnemyIntent, EnemyType,
};

pub trait Enemy: Debug {
    fn next_intent(&mut self, ascension: u8);

    fn get_current_intent(&self) -> EnemyIntent;
}

impl EnemyType {
    pub fn new(&self, ascension: u8) -> ConcreteEnemy {
        match self {
            EnemyType::JawWorm => JawWorm::new(ascension),
            EnemyType::Cultist => Cultist::new(ascension),
            EnemyType::GreenLouse => todo!(),
            EnemyType::RedLouse => todo!(),
            EnemyType::AcidSlimeM => AcidSlimeM::new(ascension),
            EnemyType::SpikeSlimeM => todo!(),
            EnemyType::AcidSlimeS => AcidSlimeS::new(ascension),
            EnemyType::SpikeSlimeS => SpikeSlimeS::new(ascension),
            EnemyType::Lagavulin => todo!(),
            EnemyType::AcidSlimeL => todo!(),
            EnemyType::SpikeSlimeL => todo!(),
            EnemyType::Looter => todo!(),
            EnemyType::FungusBeast => todo!(),
            EnemyType::GremlinFat => todo!(),
            EnemyType::GremlinSneaky => todo!(),
            EnemyType::GremlinMad => todo!(),
            EnemyType::GremlinShield => todo!(),
            EnemyType::GremlinWizard => todo!(),
            EnemyType::SlaverBlue => todo!(),
            EnemyType::SlaverRed => todo!(),
            EnemyType::GremlinNob => todo!(),
            EnemyType::SentryA => SentryA::new(ascension),
            EnemyType::SentryB => SentryB::new(ascension),
            EnemyType::Hexaghost => todo!(),
            EnemyType::TheGuardian => todo!(),
            EnemyType::SlimeBoss => todo!(),
        }
    }
}
