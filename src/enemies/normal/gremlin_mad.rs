use crate::{
    effects::{Debuff, DurationDebuffs, Effects},
    utils::{number_between, Number},
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct GremlinFat {
    intent: EnemyIntent,
}

impl Enemy for GremlinFat {
    fn next_intent(&mut self, _: u8) {}

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.clone()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}
impl GremlinFat {
    pub fn new(ascension: u8) -> ConcreteEnemy {
        let hp = if ascension >= 7 {
            number_between(14, 18)
        } else {
            number_between(13, 17)
        };

        let mut first_attack = if ascension >= 2 {
            vec![EnemyIntent::Damage(Number(5))]
        } else {
            vec![EnemyIntent::Damage(Number(4))]
        };
        first_attack.push(EnemyIntent::Debuff(Debuff::Duration((
            DurationDebuffs::Weak,
            Number(1),
        ))));
        if ascension >= 17 {
            first_attack.push(EnemyIntent::Debuff(Debuff::Duration((
                DurationDebuffs::Frail,
                Number(1),
            ))));
        }

        let gremlin = GremlinFat {
            intent: EnemyIntent::Multiple(first_attack),
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(gremlin),
            enemy_type: EnemyType::GremlinFat,
            ascension: ascension,
        }
    }
}
