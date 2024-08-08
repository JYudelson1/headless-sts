use crate::{
    cards::{CardName, Pile},
    effects::{Debuff, DurationDebuffs, Effects},
    utils::{number_between, Number},
};

use super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug)]
pub struct AcidSlimeM {
    intent: EnemyIntent,
    intent_history: [Option<AcidSlimeMAttacks>; 2],
}

impl Enemy for AcidSlimeM {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = AcidSlimeMAttacks::new_attack(&self.intent_history, ascension);
        self.intent_history[1] = self.intent_history[0].clone();
        self.intent_history[0] = Some(new_intent.clone());

        self.intent = new_intent.to_intent();
    }

    fn get_current_intent(&self) -> super::EnemyIntent {
        self.intent.clone()
    }
}

#[derive(Clone, Debug)]
enum AcidSlimeMAttacks {
    CorrosiveSpit(Number),
    Lick,
    Tackle(Number),
}

impl AcidSlimeMAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            AcidSlimeMAttacks::CorrosiveSpit(damage) => {
                let mut inner = vec![EnemyIntent::Damage(*damage)];
                inner.push(EnemyIntent::ShuffleCardToPile(
                    CardName::Slimed,
                    Pile::Discard,
                    false,
                ));
                EnemyIntent::Multiple(inner)
            }
            AcidSlimeMAttacks::Lick => {
                EnemyIntent::Debuff(Debuff::Duration((DurationDebuffs::Weak, Number(1))))
            }
            AcidSlimeMAttacks::Tackle(damage) => EnemyIntent::Damage(*damage),
        }
    }

    fn tackle(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::Tackle(Number(12))
        } else {
            Self::Tackle(Number(10))
        }
    }

    fn lick() -> Self {
        Self::Lick
    }

    fn corrosive_spit(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::CorrosiveSpit(Number(8))
        } else {
            Self::CorrosiveSpit(Number(7))
        }
    }

    pub fn new_attack(intent_history: &[Option<Self>; 2], ascension: u8) -> Self {
        loop {
            let x = rand::random::<f32>();

            let thresholds = if ascension >= 17 {
                (0.2, 0.6)
            } else {
                (0.4, 0.7)
            };

            if x < thresholds.0 {
                if !matches!(
                    intent_history,
                    [
                        Some(AcidSlimeMAttacks::Tackle(_)),
                        Some(AcidSlimeMAttacks::Tackle(_))
                    ]
                ) {
                    break AcidSlimeMAttacks::tackle(ascension);
                }
            } else if x < thresholds.1 {
                if !matches!(
                    intent_history,
                    [
                        Some(AcidSlimeMAttacks::CorrosiveSpit(_)),
                        Some(AcidSlimeMAttacks::CorrosiveSpit(_))
                    ]
                ) {
                    break AcidSlimeMAttacks::corrosive_spit(ascension);
                }
            } else {
                if !matches!(intent_history[0], Some(AcidSlimeMAttacks::Lick)) {
                    break AcidSlimeMAttacks::lick();
                }
            }
        }
    }
}

impl AcidSlimeM {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(29, 34)
        } else {
            number_between(28, 32)
        };

        let first_attack = AcidSlimeMAttacks::new_attack(&[None, None], ascension);

        let slime = AcidSlimeM {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(slime),
            enemy_type: EnemyType::AcidSlimeM,
            ascension: ascension,
        }
    }
}
