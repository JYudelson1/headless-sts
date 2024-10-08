use crate::{
    cards::Targets,
    combat::CombatOver,
    effects::{Buff, Debuff, DurationDebuffs, IntensityBuffOrDebuff, IntensityBuffs},
    enemies::EnemyIndex,
    state::State,
    utils::{NotImplemented, Number},
};

use super::Potion;

impl State {
    pub fn use_targeted_potion(
        &mut self,
        potion: Potion,
        target: EnemyIndex,
    ) -> Result<CombatOver, NotImplemented> {
        let relics = self.relics.clone();
        match potion {
            Potion::Weak => self.debuff_enemy(Debuff::Duration((DurationDebuffs::Weak, Number(3))), Targets::One, Some(target)),
            Potion::Fear => self.debuff_enemy(Debuff::Duration((DurationDebuffs::Vulnerable, Number(3))), Targets::One, Some(target)),
            Potion::Fire => {
                let (_, over) = self.get_combat().direct_damage_enemy(target, 20, &relics)?;
                if over == CombatOver::Yes {
                    return Ok(CombatOver::Yes)
                }
            },
            _ => panic!("Should not be using {potion:?} here!")
        }

        Ok(CombatOver::No)
    }

    pub fn use_untargeted_potion(&mut self, potion: Potion) -> Result<CombatOver, NotImplemented> {
        let mut combat = None;
        let relics = self.relics.clone();
        if self.is_in_combat() {
            combat = Some(self.get_combat())
        }
        match potion {
            Potion::Block => combat.unwrap().self_block += Number(12),
            Potion::Strength => combat
                .unwrap()
                .self_effects
                .apply_buff(Buff::Basic((IntensityBuffOrDebuff::Strength, Number(2)))),
            Potion::LiquidBronze => combat
                .unwrap()
                .self_effects
                .apply_buff(Buff::Intensity((IntensityBuffs::Thorns, Number(3)))),
            Potion::FruitJuice => self.increase_max_hp(5),
            Potion::Ancient => Err(NotImplemented::Potion(potion))?,
            Potion::Attack => Err(NotImplemented::Potion(potion))?,
            Potion::BlessingOfTheForge => Err(NotImplemented::Potion(potion))?,
            Potion::Blood => Err(NotImplemented::Potion(potion))?,
            Potion::Colorless => Err(NotImplemented::Potion(potion))?,
            Potion::Cultist => Err(NotImplemented::Potion(potion))?,
            Potion::Dex => Err(NotImplemented::Potion(potion))?,
            Potion::DistilledChaos => Err(NotImplemented::Potion(potion))?,
            Potion::Duplication => Err(NotImplemented::Potion(potion))?,
            Potion::Elixer => Err(NotImplemented::Potion(potion))?,
            Potion::Energy => Err(NotImplemented::Potion(potion))?,
            Potion::EntropicBrew => Err(NotImplemented::Potion(potion))?,
            Potion::EssenceOfSteel => Err(NotImplemented::Potion(potion))?,
            Potion::Explosive => {
                let over = self.get_combat().direct_damage_all_enemies(10, &relics)?;
                if over == CombatOver::Yes {
                    return Ok(over)
                }
            },
            Potion::FairyInABottle => Err(NotImplemented::Potion(potion))?,
            Potion::Flex => Err(NotImplemented::Potion(potion))?,
            Potion::GamblersBrew => Err(NotImplemented::Potion(potion))?,
            Potion::HeartOfIron => Err(NotImplemented::Potion(potion))?,
            Potion::LiquidMemories => Err(NotImplemented::Potion(potion))?,
            Potion::Power => Err(NotImplemented::Potion(potion))?,
            Potion::Regen => Err(NotImplemented::Potion(potion))?,
            Potion::Skill => Err(NotImplemented::Potion(potion))?,
            Potion::SmokeBomb => Err(NotImplemented::Potion(potion))?,
            Potion::SneckoOil => Err(NotImplemented::Potion(potion))?,
            Potion::Speed => Err(NotImplemented::Potion(potion))?,
            Potion::Swift => Err(NotImplemented::Potion(potion))?,
            _ => panic!("Should not be using {potion:?} here!")
        }

        Ok(CombatOver::No)
    }
}
