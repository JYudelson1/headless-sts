use crate::{
    cards::{CardName, Pile},
    effects::{Debuff, DurationDebuffs, Effects},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct SpikeSlimeM {
    intent: EnemyIntent,
    intent_history: [Option<SpikeSlimeMAttacks>; 2],
}

impl Enemy for SpikeSlimeM {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = SpikeSlimeMAttacks::new_attack(&self.intent_history, ascension);
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
enum SpikeSlimeMAttacks {
    Lick,
    FlameTackle(Number),
}

impl SpikeSlimeMAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            SpikeSlimeMAttacks::Lick => {
                EnemyIntent::Debuff(Debuff::Duration((DurationDebuffs::Frail, Number(1))))
            }
            SpikeSlimeMAttacks::FlameTackle(damage) => EnemyIntent::Multiple(vec![
                EnemyIntent::Damage(*damage),
                EnemyIntent::ShuffleCardToPile(CardName::Slimed, Pile::Discard, false),
            ]),
        }
    }

    fn flame_tackle(ascension: u8) -> Self {
        if ascension >= 2 {
            Self::FlameTackle(Number(10))
        } else {
            Self::FlameTackle(Number(8))
        }
    }

    fn lick() -> Self {
        Self::Lick
    }

    pub fn new_attack(intent_history: &[Option<Self>; 2], ascension: u8) -> Self {
        loop {
            let x = rand::random::<f32>();

            if x < 0.3 {
                if !matches!(
                    intent_history,
                    [
                        Some(SpikeSlimeMAttacks::FlameTackle(_)),
                        Some(SpikeSlimeMAttacks::FlameTackle(_))
                    ]
                ) {
                    return SpikeSlimeMAttacks::flame_tackle(ascension);
                }
            } else {
                if (ascension >= 17 && !matches!(intent_history[0], Some(SpikeSlimeMAttacks::Lick)))
                    || (!matches!(
                        intent_history,
                        [
                            Some(SpikeSlimeMAttacks::Lick),
                            Some(SpikeSlimeMAttacks::Lick)
                        ]
                    ))
                {
                    return SpikeSlimeMAttacks::lick();
                }
            }
        }
    }
}

impl SpikeSlimeM {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(29, 34)
        } else {
            number_between(28, 32)
        };

        let first_attack = SpikeSlimeMAttacks::new_attack(&[None, None], ascension);

        let slime = SpikeSlimeM {
            intent: first_attack.to_intent(),
            intent_history: [Some(first_attack), None],
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(slime),
            enemy_type: EnemyType::SpikeSlimeM,
            ascension: ascension,
        }
    }
}
