use crate::{
    effects::{Debuff, DurationDebuffs, Effects},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct SlaverBlue {
    intent: EnemyIntent,
    intent_history: [Option<SlaverBlueAttacks>; 2],
}

impl Enemy for SlaverBlue {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = SlaverBlueAttacks::new_attack(&self.intent_history, ascension);
        self.intent_history[1] = self.intent_history[0].clone();
        self.intent_history[0] = Some(new_intent.clone());

        self.intent = new_intent.to_intent();
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
enum SlaverBlueAttacks {
    Rake((Number, Number)),
    Stab(Number),
}

impl SlaverBlueAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            SlaverBlueAttacks::Stab(damage) => EnemyIntent::Damage(*damage),
            SlaverBlueAttacks::Rake((damage, weak_amt)) => EnemyIntent::Multiple(vec![
                EnemyIntent::Damage(*damage),
                EnemyIntent::Debuff(Debuff::Duration((DurationDebuffs::Weak, *weak_amt))),
            ]),
        }
    }

    fn stab(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::Stab(Number(13))
        } else {
            Self::Stab(Number(12))
        }
    }

    fn rake(ascension: u8) -> Self {
        if ascension >= 17 {
            Self::Rake((Number(8), Number(2)))
        } else if ascension >= 2 {
            Self::Rake((Number(8), Number(1)))
        } else {
            Self::Rake((Number(7), Number(1)))
        }
    }

    pub fn new_attack(intent_history: &[Option<Self>; 2], ascension: u8) -> Self {
        loop {
            let x = rand::random::<f32>();

            if x < 0.4 {
                if !matches!(
                    intent_history,
                    [
                        Some(SlaverBlueAttacks::Stab(_)),
                        Some(SlaverBlueAttacks::Stab(_))
                    ]
                ) {
                    return SlaverBlueAttacks::stab(ascension);
                }
            } else {
                if (ascension >= 17
                    && !matches!(intent_history[0], Some(SlaverBlueAttacks::Rake(_))))
                    || (!matches!(
                        intent_history,
                        [
                            Some(SlaverBlueAttacks::Rake(_)),
                            Some(SlaverBlueAttacks::Rake(_))
                        ]
                    ))
                {
                    return SlaverBlueAttacks::rake(ascension);
                }
            }
        }
    }
}

impl SlaverBlue {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(48, 52)
        } else {
            number_between(46, 50)
        };

        let first_attack = SlaverBlueAttacks::new_attack(&[None, None], ascension);

        let slaver = SlaverBlue {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(slaver),
            enemy_type: EnemyType::SlaverBlue,
            ascension: ascension,
        }
    }
}
