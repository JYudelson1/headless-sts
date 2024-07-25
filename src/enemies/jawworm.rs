use crate::{
    effects::{Buff, Effects},
    utils::Number,
};

use super::{enemy_trait::{Enemy, EnemyConstructor}, get_starting_health, EnemyIntent};

pub struct JawWorm {
    intent: EnemyIntent,
    effects: Effects,
    max_hp: u16,
    current_hp: u16,
    intent_history: [Option<JawWormAttacks>; 2],
    current_block: u16,
}

impl Enemy for JawWorm {
    fn next_intent(&mut self, ascension: u8) {
        let new_intent = loop {
            let x = rand::random::<f32>();
            if x <= 0.45 {
                if !matches!(self.intent_history[0], Some(JawWormAttacks::Bellow(_, _))) {
                    break JawWormAttacks::bellow(ascension);
                }
            } else if x <= 0.75 {
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

    fn get_enemy_type(&self) -> super::EnemyType {
        super::EnemyType::JawWorm
    }

    fn get_effects(&mut self) -> &mut Effects {
        &mut self.effects
    }

    fn get_current_intent(&self) -> super::EnemyIntent {
        self.intent
    }

    fn get_max_hp(&self) -> u16 {
        self.max_hp
    }

    fn get_current_hp(&self) -> u16 {
        self.current_hp
    }

    fn get_current_block(&self) -> u16 {
        todo!()
    }
}

#[derive(Clone)]
enum JawWormAttacks {
    Bellow(Number, Number),
    Chomp(Number),
    Thrash,
}

impl JawWormAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            JawWormAttacks::Bellow(buff, block) => {
                EnemyIntent::BuffAndBlock(Buff::Strength(*buff), *block)
            }
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

pub struct JawWormConstructor;

impl EnemyConstructor<JawWorm> for JawWormConstructor {
    fn new(ascension: u8) -> JawWorm {
        let hp = if ascension >= 7 {
            get_starting_health(42, 46)
        } else {
            get_starting_health(40, 44)
        };

        let first_attack = JawWormAttacks::chomp(ascension);

        JawWorm {
            intent: first_attack.to_intent(),
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            intent_history: [Some(first_attack), None],
            current_block: 0,
        }
    }
}
