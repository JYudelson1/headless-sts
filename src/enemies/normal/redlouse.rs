use crate::{
    effects::{Buff, Effects, IntensityBuffOrDebuff, IntensityBuffs},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct RedLouse {
    intent: EnemyIntent,
    intent_history: [Option<RedLouseAttacks>; 2],
    d: u8,
}

impl Enemy for RedLouse {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = RedLouseAttacks::new_attack(&self.intent_history, ascension, self.d);
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
enum RedLouseAttacks {
    Grow(Number),
    Bite(Number),
}

impl RedLouseAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            RedLouseAttacks::Grow(amt) => {
                EnemyIntent::Buff(Buff::Basic((IntensityBuffOrDebuff::Strength, *amt)))
            }
            RedLouseAttacks::Bite(damage) => EnemyIntent::Damage(*damage),
        }
    }

    fn bite(ascension: u8, d: u8) -> Self {
        if ascension >= 2 {
            Self::Bite(Number((d + 1) as i16))
        } else {
            Self::Bite(Number(d as i16))
        }
    }

    fn grow(ascension: u8) -> Self {
        if ascension >= 17 {
            Self::Grow(Number(4))
        } else {
            Self::Grow(Number(3))
        }
    }

    pub fn new_attack(intent_history: &[Option<Self>; 2], ascension: u8, d: u8) -> Self {
        loop {
            let x = rand::random::<f32>();

            if x < 0.75 {
                if !matches!(
                    intent_history,
                    [
                        Some(RedLouseAttacks::Bite(_)),
                        Some(RedLouseAttacks::Bite(_))
                    ]
                ) {
                    return RedLouseAttacks::bite(ascension, d);
                }
            } else {
                if (ascension >= 17 && !matches!(intent_history[0], Some(RedLouseAttacks::Grow(_))))
                    || (!matches!(
                        intent_history,
                        [
                            Some(RedLouseAttacks::Grow(_)),
                            Some(RedLouseAttacks::Grow(_))
                        ]
                    ))
                {
                    return RedLouseAttacks::grow(ascension);
                }
            }
        }
    }
}

impl RedLouse {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(11, 16)
        } else {
            number_between(10, 15)
        };

        let d = number_between(5, 7);

        let first_attack = RedLouseAttacks::new_attack(&[None, None], ascension, d);

        let louse = RedLouse {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
            d,
        };

        let mut effects = Effects::new();

        let curl_up = if ascension >= 17 {
            number_between(9, 12)
        } else if ascension >= 7 {
            number_between(4, 8)
        } else {
            number_between(3, 7)
        };
        effects.apply_buff(Buff::Intensity((IntensityBuffs::CurlUp, Number(curl_up))));

        ConcreteEnemy {
            effects,
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(louse),
            enemy_type: EnemyType::RedLouse,
            ascension: ascension,
        }
    }
}
