use crate::{
    effects::{Buff, Debuff, Effects, IntensityBuffOrDebuff, IntensityBuffs},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug)]
pub struct Lagavulin {
    intent: EnemyIntent,
    turn_num: u8,
    intent_cycle: u8,
    awoken: bool,
}

impl Enemy for Lagavulin {
    fn next_intent(&mut self, ascension: u8) {
        self.turn_num += 1;
        if !self.awoken && (self.turn_num == 3 || self.intent == EnemyIntent::Stun) {
            self.intent = Lagavulin::attack(ascension);
            self.intent_cycle = 1;
            self.awoken = true;
            return;
        }
        self.intent_cycle += 1;

        if self.intent_cycle == 3 {
            self.intent = Lagavulin::siphon_soul(ascension);
            self.intent_cycle = 0;
        }
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }

    fn lost_hp(&mut self) {
        self.intent = EnemyIntent::Stun;
    }
}

impl Lagavulin {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 8 {
            number_between(112, 115)
        } else {
            number_between(109, 111)
        };

        let lagavulin = Lagavulin {
            intent: EnemyIntent::Sleep,
            turn_num: 0,
            intent_cycle: 0,
            awoken: false,
        };

        let mut effects = Effects::new();
        effects.apply_buff(Buff::Intensity((IntensityBuffs::Metallicize, Number(8))));

        ConcreteEnemy {
            effects: effects,
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(lagavulin),
            enemy_type: EnemyType::Lagavulin,
            ascension: ascension,
        }
    }

    fn attack(ascension: u8) -> EnemyIntent {
        if ascension >= 3 {
            EnemyIntent::Damage(Number(20))
        } else {
            EnemyIntent::Damage(Number(18))
        }
    }

    fn siphon_soul(ascension: u8) -> EnemyIntent {
        let amt = if ascension >= 18 { -2 } else { -1 };
        EnemyIntent::Multiple(vec![
            EnemyIntent::Debuff(Debuff::Basic((
                IntensityBuffOrDebuff::Strength,
                Number(amt),
            ))),
            EnemyIntent::Debuff(Debuff::Basic((
                IntensityBuffOrDebuff::Dexterity,
                Number(amt),
            ))),
        ])
    }
}
