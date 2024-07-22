use rand::Rng;

use crate::effects::Effects;

use super::{EnemyIntent, EnemyType, VisibleEnemy};

pub trait Enemy
where
    Self: Sized,
{
    fn new(ascension: u8) -> Self;

    fn next_intent(&mut self, ascension: u8);

    fn get_enemy_type(&self) -> EnemyType;
    fn get_effects(&self) -> Effects;
    fn get_current_intent(&self) -> EnemyIntent;
    fn get_max_hp(&self) -> u16;
    fn get_current_hp(&self) -> u16;

    fn to_visible(&self) -> VisibleEnemy {
        VisibleEnemy {
            enemy_type: self.get_enemy_type(),
            effects: self.get_effects(),
            intent: self.get_current_intent().to_visible_intent(),
            max_hp: self.get_max_hp(),
            current_hp: self.get_current_hp(),
        }
    }
}
