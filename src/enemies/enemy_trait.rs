use std::fmt::Debug;

use crate::utils::NotImplemented;

use super::{
    bosses::hexaghost::Hexaghost,
    elites::{
        lagavulin::Lagavulin,
        sentries::{SentryA, SentryB},
    },
    normal::{
        acid_slime_m::AcidSlimeM, acid_slime_s::AcidSlimeS, cultist::Cultist,
        fungus_beast::FungusBeast, greenlouse::GreenLouse, gremlin_mad::GremlinFat,
        gremlin_sneaky::GremlinSneaky, jawworm::JawWorm, redlouse::RedLouse,
        slaver_blue::SlaverBlue, slaver_red::SlaverRed, spike_slime_m::SpikeSlimeM,
        spike_slime_s::SpikeSlimeS,
    },
    ConcreteEnemy, EnemyIntent, EnemyType,
};

pub trait Enemy: Debug {
    fn next_intent(&mut self, ascension: u8);

    fn get_current_intent(&self) -> EnemyIntent;

    fn lost_hp(&mut self) {}

    fn duplicate(&self) -> Box<dyn Enemy>;
}

impl EnemyType {
    pub fn new(&self, ascension: u8, player_hp: u16) -> Result<ConcreteEnemy, NotImplemented> {
        match self {
            EnemyType::JawWorm => Ok(JawWorm::new(ascension)),
            EnemyType::Cultist => Ok(Cultist::new(ascension)),
            EnemyType::GreenLouse => Ok(GreenLouse::new(ascension)),
            EnemyType::RedLouse => Ok(RedLouse::new(ascension)),
            EnemyType::AcidSlimeM => Ok(AcidSlimeM::new(ascension)),
            EnemyType::SpikeSlimeM => Ok(SpikeSlimeM::new(ascension)),
            EnemyType::AcidSlimeS => Ok(AcidSlimeS::new(ascension)),
            EnemyType::SpikeSlimeS => Ok(SpikeSlimeS::new(ascension)),
            EnemyType::Lagavulin => Ok(Lagavulin::new(ascension)),
            EnemyType::AcidSlimeL => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::SpikeSlimeL => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::Looter => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::FungusBeast => Ok(FungusBeast::new(ascension)),
            EnemyType::GremlinFat => Ok(GremlinFat::new(ascension)),
            EnemyType::GremlinSneaky => Ok(GremlinSneaky::new(ascension)),
            EnemyType::GremlinMad => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::GremlinShield => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::GremlinWizard => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::SlaverBlue => Ok(SlaverBlue::new(ascension)),
            EnemyType::SlaverRed => Ok(SlaverRed::new(ascension)),
            EnemyType::GremlinNob => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::SentryA => Ok(SentryA::new(ascension)),
            EnemyType::SentryB => Ok(SentryB::new(ascension)),
            EnemyType::Hexaghost => Ok(Hexaghost::new(ascension, player_hp)),
            EnemyType::TheGuardian => Err(NotImplemented::Enemy(self.clone())),
            EnemyType::SlimeBoss => Err(NotImplemented::Enemy(self.clone())),
        }
    }
}
