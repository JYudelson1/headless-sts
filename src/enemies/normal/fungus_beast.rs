use crate::{
    effects::{Buff, Effects, IntensityBuffOrDebuff, IntensityBuffs},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug)]
pub struct FungusBeast {
    intent: EnemyIntent,
    intent_history: [Option<FungusBeastAttacks>; 2],
}

impl Enemy for FungusBeast {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = FungusBeastAttacks::new_attack(&self.intent_history, ascension);
        self.intent_history[1] = self.intent_history[0].clone();
        self.intent_history[0] = Some(new_intent.clone());

        self.intent = new_intent.to_intent();
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }
}

#[derive(Clone, Debug)]
enum FungusBeastAttacks {
    Bite(Number),
    Grow(Number),
}

impl FungusBeastAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            FungusBeastAttacks::Bite(attack) => EnemyIntent::Damage(*attack),
            FungusBeastAttacks::Grow(amt) => {
                EnemyIntent::Buff(Buff::Basic((IntensityBuffOrDebuff::Strength, *amt)))
            }
        }
    }

    fn bite() -> Self {
        Self::Bite(Number(6))
    }

    fn grow(ascension: u8) -> Self {
        if ascension >= 17 {
            Self::Grow(Number(5))
        } else if ascension >= 2 {
            Self::Grow(Number(4))
        } else {
            Self::Grow(Number(3))
        }
    }

    pub fn new_attack(intent_history: &[Option<Self>; 2], ascension: u8) -> Self {
        loop {
            let x = rand::random::<f32>();

            if x < 0.6 {
                if !matches!(
                    intent_history,
                    [
                        Some(FungusBeastAttacks::Bite(_)),
                        Some(FungusBeastAttacks::Bite(_))
                    ]
                ) {
                    return FungusBeastAttacks::bite();
                }
            } else {
                if !matches!(intent_history[0], Some(FungusBeastAttacks::Grow(_))) {
                    return FungusBeastAttacks::grow(ascension);
                }
            }
        }
    }
}

impl FungusBeast {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(24, 28)
        } else {
            number_between(22, 28)
        };

        let first_attack = FungusBeastAttacks::new_attack(&[None, None], ascension);

        let beast = FungusBeast {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
        };

        let mut effects = Effects::new();
        effects.apply_buff(Buff::Intensity((IntensityBuffs::SporeCloud, Number(2))));

        ConcreteEnemy {
            effects,
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(beast),
            enemy_type: EnemyType::FungusBeast,
            ascension: ascension,
        }
    }
}
