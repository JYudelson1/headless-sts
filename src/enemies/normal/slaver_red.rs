use crate::{
    effects::{Debuff, DurationDebuffs, Effects, OneTurnBoolDebuffs},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct SlaverRed {
    intent: EnemyIntent,
    intent_history: [Option<SlaverRedAttacks>; 2],
    used_entangled: bool,
}

impl Enemy for SlaverRed {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent =
            SlaverRedAttacks::new_attack(&self.intent_history, ascension, &mut self.used_entangled);
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
enum SlaverRedAttacks {
    Scrape((Number, Number)),
    Stab(Number),
    Entangle,
}

impl SlaverRedAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            SlaverRedAttacks::Stab(damage) => EnemyIntent::Damage(*damage),
            SlaverRedAttacks::Scrape((damage, weak_amt)) => EnemyIntent::Multiple(vec![
                EnemyIntent::Damage(*damage),
                EnemyIntent::Debuff(Debuff::Duration((DurationDebuffs::Vulnerable, *weak_amt))),
            ]),
            SlaverRedAttacks::Entangle => {
                EnemyIntent::Debuff(Debuff::OneTurnBool(OneTurnBoolDebuffs::Entangled))
            }
        }
    }

    fn stab(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::Stab(Number(14))
        } else {
            Self::Stab(Number(13))
        }
    }

    fn scrape(ascension: u8) -> Self {
        if ascension >= 17 {
            Self::Scrape((Number(9), Number(2)))
        } else if ascension >= 2 {
            Self::Scrape((Number(9), Number(1)))
        } else {
            Self::Scrape((Number(8), Number(1)))
        }
    }

    pub fn new_attack(
        intent_history: &[Option<Self>; 2],
        ascension: u8,
        used_entangled: &mut bool,
    ) -> Self {
        if !*used_entangled {
            let x = rand::random::<f32>();
            if x < 0.25 {
                *used_entangled = true;
                return Self::Entangle;
            }
            if let Some(prev_attack) = &intent_history[0] {
                if matches!(prev_attack, Self::Stab(_)) {
                    return Self::scrape(ascension);
                } else {
                    if ascension >= 17 {
                        return Self::stab(ascension);
                    }
                    if let Some(prev_prev_attack) = &intent_history[1] {
                        return match prev_prev_attack {
                            SlaverRedAttacks::Scrape(_) => Self::stab(ascension),
                            SlaverRedAttacks::Stab(_) => Self::scrape(ascension),
                            SlaverRedAttacks::Entangle => unreachable!(),
                        };
                    }
                }
            }
        }

        loop {
            let x = rand::random::<f32>();

            if x < 0.45 {
                if !matches!(
                    intent_history,
                    [
                        Some(SlaverRedAttacks::Stab(_)),
                        Some(SlaverRedAttacks::Stab(_))
                    ]
                ) {
                    return SlaverRedAttacks::stab(ascension);
                }
            } else {
                if (ascension >= 17
                    && !matches!(intent_history[0], Some(SlaverRedAttacks::Scrape(_))))
                    || (!matches!(
                        intent_history,
                        [
                            Some(SlaverRedAttacks::Scrape(_)),
                            Some(SlaverRedAttacks::Scrape(_))
                        ]
                    ))
                {
                    return SlaverRedAttacks::scrape(ascension);
                }
            }
        }
    }
}

impl SlaverRed {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(48, 52)
        } else {
            number_between(46, 50)
        };

        let first_attack = SlaverRedAttacks::stab(ascension);

        let slaver = SlaverRed {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
            used_entangled: false,
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(slaver),
            enemy_type: EnemyType::SlaverRed,
            ascension: ascension,
        }
    }
}
