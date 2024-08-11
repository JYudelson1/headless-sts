use crate::{
    effects::{Buff, Effects, IntensityBuffOrDebuff},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug)]
pub struct JawWorm {
    intent: EnemyIntent,
    intent_history: [Option<JawWormAttacks>; 2],
}

impl Enemy for JawWorm {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = loop {
            let x = rand::random::<f32>();
            if x < 0.45 {
                if !matches!(self.intent_history[0], Some(JawWormAttacks::Bellow(_, _))) {
                    break JawWormAttacks::bellow(ascension);
                }
            } else if x < 0.75 {
                if !matches!(
                    self.intent_history,
                    [Some(JawWormAttacks::Thrash), Some(JawWormAttacks::Thrash)]
                ) {
                    break JawWormAttacks::thrash();
                }
            } else {
                if !matches!(self.intent_history[0], Some(JawWormAttacks::Chomp(_))) {
                    break JawWormAttacks::chomp(ascension);
                }
            }
        };

        self.intent_history[1] = self.intent_history[0].clone();
        self.intent_history[0] = Some(new_intent.clone());

        self.intent = new_intent.to_intent();
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }
}

#[derive(Clone, Debug)]
enum JawWormAttacks {
    Bellow(Number, Number),
    Chomp(Number),
    Thrash,
}

impl JawWormAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            JawWormAttacks::Bellow(buff, block) => EnemyIntent::BuffAndBlock(
                Buff::Basic((IntensityBuffOrDebuff::Strength, *buff)),
                *block,
            ),
            JawWormAttacks::Chomp(attack) => EnemyIntent::Damage(*attack),
            JawWormAttacks::Thrash => EnemyIntent::AttackAndBlock(Number(7), Number(5)),
        }
    }

    fn bellow(ascension: u8) -> Self {
        if ascension >= 17 {
            Self::Bellow(Number(5), Number(9))
        } else if ascension >= 2 {
            Self::Bellow(Number(4), Number(6))
        } else {
            Self::Bellow(Number(3), Number(6))
        }
    }

    fn chomp(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::Chomp(Number(12))
        } else {
            Self::Chomp(Number(11))
        }
    }

    fn thrash() -> Self {
        Self::Thrash
    }
}

impl JawWorm {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(42, 46)
        } else {
            number_between(40, 44)
        };

        let first_attack = JawWormAttacks::chomp(ascension);

        let jawworm = JawWorm {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(jawworm),
            enemy_type: EnemyType::JawWorm,
            ascension: ascension,
        }
    }
}
