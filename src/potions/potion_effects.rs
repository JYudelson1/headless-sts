use crate::{
    effects::{Buff, IntensityBuffOrDebuff, IntensityBuffs},
    enemies::EnemyIndex,
    state::State,
    utils::Number,
};

use super::Potion;

impl State {
    pub fn use_targeted_potion(&mut self, potion: Potion, _target: EnemyIndex) {
        match potion {
            _ => panic!(),
        }
    }

    pub fn use_untargeted_potion(&mut self, potion: Potion) {
        let mut combat = None;
        if self.is_in_combat() {
            combat = Some(self.get_combat())
        }
        match potion {
            Potion::BlockPotion => combat.unwrap().self_block += Number(12),
            Potion::StrengthPotion => combat
                .unwrap()
                .self_effects
                .apply_buff(Buff::Basic((IntensityBuffOrDebuff::Strength, Number(2)))),
            Potion::LiquidBronze => combat
                .unwrap()
                .self_effects
                .apply_buff(Buff::Intensity((IntensityBuffs::Thorns, Number(3)))),
            Potion::FruitJuice => self.increase_max_hp(5),
        }
    }
}
