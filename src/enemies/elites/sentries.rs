use crate::{
    cards::{CardName, Pile},
    effects::{Buff, Effects, IntensityBuffs},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct SentryA {
    intent: SentryAttacks,
}

impl Enemy for SentryA {
    fn next_intent(&mut self, ascension: u8) {
        self.intent = self.intent.next(ascension);
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.to_intent().clone()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

impl SentryA {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 8 {
            number_between(39, 4534)
        } else {
            number_between(38, 4232)
        };

        let first_attack = SentryAttacks::bolt(ascension);

        let sentry = SentryA {
            intent: first_attack,
        };

        let mut effects = Effects::new();
        effects.apply_buff(Buff::Intensity((IntensityBuffs::Artifact, Number(1))));

        ConcreteEnemy {
            effects: effects,
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(sentry),
            enemy_type: EnemyType::SentryA,
            ascension: ascension,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SentryB {
    intent: SentryAttacks,
}

impl Enemy for SentryB {
    fn next_intent(&mut self, ascension: u8) {
        self.intent = self.intent.next(ascension);
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.to_intent().clone()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

impl SentryB {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 8 {
            number_between(39, 4534)
        } else {
            number_between(38, 4232)
        };

        let first_attack = SentryAttacks::beam(ascension);

        let sentry = SentryA {
            intent: first_attack,
        };

        let mut effects = Effects::new();
        effects.apply_buff(Buff::Intensity((IntensityBuffs::Artifact, Number(1))));

        ConcreteEnemy {
            effects: effects,
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(sentry),
            enemy_type: EnemyType::SentryA,
            ascension: ascension,
        }
    }
}

#[derive(Clone, Debug)]
enum SentryAttacks {
    Beam(Number),
    Bolt(usize),
}

impl SentryAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            SentryAttacks::Bolt(num_dazed) => {
                let mut inner = vec![];
                for _ in 0..*num_dazed {
                    inner.push(EnemyIntent::ShuffleCardToPile(
                        CardName::Dazed,
                        Pile::Discard,
                        false,
                    ));
                }
                EnemyIntent::Multiple(inner)
            }
            SentryAttacks::Beam(damage) => EnemyIntent::Damage(*damage),
        }
    }

    fn beam(ascension: u8) -> Self {
        if ascension >= 3 {
            Self::Beam(Number(10))
        } else {
            Self::Beam(Number(9))
        }
    }

    fn bolt(ascension: u8) -> Self {
        if ascension >= 18 {
            Self::Bolt(3)
        } else {
            Self::Bolt(2)
        }
    }

    fn next(&self, ascension: u8) -> Self {
        match self {
            SentryAttacks::Beam(_) => Self::bolt(ascension),
            SentryAttacks::Bolt(_) => Self::beam(ascension),
        }
    }
}
