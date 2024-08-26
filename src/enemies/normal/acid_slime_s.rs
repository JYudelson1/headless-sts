use crate::{
    effects::{Debuff, DurationDebuffs, Effects},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct AcidSlimeS {
    intent: AcidSlimeSAttacks,
}

impl Enemy for AcidSlimeS {
    fn next_intent(&mut self, ascension: u8) {
        // Alternate attacks
        let new_intent = match self.intent {
            AcidSlimeSAttacks::Lick => AcidSlimeSAttacks::tackle(ascension),
            AcidSlimeSAttacks::Tackle(_) => AcidSlimeSAttacks::lick(),
        };

        self.intent = new_intent;
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.to_intent().clone()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
enum AcidSlimeSAttacks {
    Lick,
    Tackle(Number),
}

impl AcidSlimeSAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            AcidSlimeSAttacks::Lick => {
                EnemyIntent::Debuff(Debuff::Duration((DurationDebuffs::Weak, Number(1))))
            }
            AcidSlimeSAttacks::Tackle(damage) => EnemyIntent::Damage(*damage),
        }
    }

    fn tackle(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::Tackle(Number(4))
        } else {
            Self::Tackle(Number(3))
        }
    }

    fn lick() -> Self {
        Self::Lick
    }
}

impl AcidSlimeS {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(9, 13)
        } else {
            number_between(8, 12)
        };

        let first_attack = if ascension >= 17 {
            AcidSlimeSAttacks::lick()
        } else {
            let possible = vec![
                AcidSlimeSAttacks::tackle(ascension),
                AcidSlimeSAttacks::lick(),
            ];
            possible[number_between(0, 1)].clone()
        };

        let slime = AcidSlimeS {
            intent: first_attack,
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(slime),
            enemy_type: EnemyType::AcidSlimeS,
            ascension: ascension,
        }
    }
}
