use crate::{
    cardrewardrng::CombatType,
    effects::{Buff, Debuff, DurationDebuffs, IntensityBuffOrDebuff, IntensityBuffs},
    relics::{Relic, Relics},
    screens::VisibleStates,
    state::State,
    utils::Number,
};

use super::Combat;

impl State {
    pub fn start_turn_1_effects(&mut self) {
        if let VisibleStates::Combat(combat) = &mut self.visible_screen {
            // Note that we iterate throuh the relics so the order is dependant on relic order
            // E.G. If symbiotic virus was obtained before nuclear battery, you should
            // channel 1 dark and then 1 plasma. If they were obtained in the opposite order,
            // you should channel them in the opposite order
            let relics = self.relics.clone();
            for relic in &mut self.relics.list {
                combat._start_of_combat_relic(relic, &relics);
            }
            // Blood vial requires healing, which accesses state (to check for magic flower)
            if self.relics.contains(Relic::BloodVial) {
                self.heal(2);
            }
        } else {
            panic!("You should be in combat now!")
        }
    }

    pub fn start_every_turn_effects(&mut self) {
        if let VisibleStates::Combat(combat) = &mut self.visible_screen {
            // Note that we iterate throuh the relics so the order is dependant on relic order
            for relic in &mut self.relics.list {
                combat._start_of_turn_relic(relic);
            }
        } else {
            panic!("You should be in combat now!")
        }
    }
}

impl Combat {
    fn _start_of_combat_relic(&mut self, relic: &mut Relic, relics: &Relics) {
        match relic {
            Relic::CrackedCore => todo!(),    // Channel 1 lightning
            Relic::SymbioticVirus => todo!(), // Channel 1 dark
            Relic::NuclearBattery => todo!(), // Channel 1 plasma
            Relic::PureWater => todo!(),      // Add miracle to hand
            Relic::HolyWater => todo!(),      // Add 3 miracles to hand
            Relic::RingOfSnake => {
                // NOTE: literally impossible for this to end the combat
                let _ = self.draw(2, relics);
            },    // Draw 2 cards
            Relic::BagOfPrep => {
                // NOTE: literally impossible for this to end the combat
                let _ = self.draw(2, relics);
            },      // Draw 2 cards
            Relic::BagOfMarbles => {
                let debuff = Debuff::Duration((DurationDebuffs::Vulnerable, Number(1)));
                for enemy in &mut self.enemies {
                    enemy.effects.apply_debuff(debuff, relics);
                }
            }
            Relic::RedMask => {
                let debuff = Debuff::Duration((DurationDebuffs::Weak, Number(1)));
                for enemy in &mut self.enemies {
                    enemy.effects.apply_debuff(debuff, relics);
                }
            }
            Relic::DataDisk => self.self_effects.apply_buff(Buff::Basic((IntensityBuffOrDebuff::Focus, Number(1)))),
            Relic::Anchor => self.self_block += Number(10),
            Relic::AncientTeaSet(rested) => {
                if *rested {
                    self.current_energy += 2;
                    *rested = false;
                }
            }
            Relic::BronzeScales => self.self_effects.apply_buff(Buff::Intensity((IntensityBuffs::Thorns, Number(3)))),
            Relic::Lantern => self.current_energy += 1,
            Relic::SmoothStone => self.self_effects.apply_buff(Buff::Basic((IntensityBuffOrDebuff::Dexterity, Number(1)))),
            Relic::Vajra => self.self_effects.apply_buff(Buff::Basic((IntensityBuffOrDebuff::Strength, Number(1)))),
            Relic::SlingOfCourage => {
                if self.combat_type == CombatType::Elite {
                    self.self_effects.apply_buff(Buff::Basic((IntensityBuffOrDebuff::Strength, Number(2))))
                }
            },
            Relic::NinjaScroll => todo!(),    // Add 3 shivs to hand
            Relic::RunicCapacitor => todo!(), // Add 3 orb slots
            Relic::Girya(amt) => self.self_effects.apply_buff(Buff::Basic((IntensityBuffOrDebuff::Strength, Number(*amt as i16)))),
            _ => (),
        }
    }

    fn _start_of_turn_relic(&mut self, relic: &mut Relic) {
        match relic {
            Relic::ArtOfWar(is_on) => {
                if *is_on && self.turn != 1 {
                    self.current_energy += 1;
                }
            }
            _ => (),
        }
    }

    pub fn _end_of_turn_relic(&mut self, relic: &mut Relic) {
        match relic {
            Relic::FrozenCore => todo!(),
            Relic::Orichalcum => {
                if self.self_block == Number(0) {
                    self.gain_block(Number(6));
                }
            }
            _ => (),
        }
    }
}
