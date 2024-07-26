use crate::{
    effects::{Buff, Debuff},
    relics::Relic,
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
            for relic in &mut self.relics.list {
                combat._start_of_combat_relic(relic);
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
    fn _start_of_combat_relic(&mut self, relic: &mut Relic) {
        match relic {
            Relic::PaperPhrog => self.add_relic(Relic::PaperPhrog),
            Relic::PaperKrane => self.add_relic(Relic::PaperKrane),
            Relic::Ginger => self.add_relic(Relic::Ginger),
            Relic::Turnip => self.add_relic(Relic::Turnip),
            Relic::RunicPyramid => self.add_relic(Relic::RunicPyramid),
            Relic::CrackedCore => todo!(),    // Channel 1 lightning
            Relic::SymbioticVirus => todo!(), // Channel 1 dark
            Relic::NuclearBattery => todo!(), // Channel 1 plasma
            Relic::PureWater => todo!(),      // Add miracle to hand
            Relic::HolyWater => todo!(),      // Add 3 miracles to hand
            Relic::RingOfSnake => todo!(),    // Draw 2 cards
            Relic::BagOfPrep => todo!(),      // Draw 2 cards
            Relic::BagOfMarbles => {
                let debuff = Debuff::Vulnerable(Number(1));
                for enemy in &mut self.enemies {
                    enemy.effects.apply_debuff(debuff);
                }
            }
            Relic::RedMask => {
                let debuff = Debuff::Weak(Number(1));
                for enemy in &mut self.enemies {
                    enemy.effects.apply_debuff(debuff);
                }
            }
            Relic::DataDisk => self.self_effects.apply_buff(Buff::Focus(Number(1))),
            Relic::Anchor => self.self_block += Number(10),
            Relic::AncientTeaSet(rested) => {
                if *rested {
                    self.current_energy += 2;
                    *rested = false;
                }
            }
            Relic::BronzeScales => self.self_effects.apply_buff(Buff::Thorns(Number(3))),
            Relic::Lantern => self.current_energy += 1,
            Relic::SmoothStone => self.self_effects.apply_buff(Buff::Dexterity(Number(1))),
            Relic::NinjaScroll => todo!(),    // Add 3 shivs to hand
            Relic::RunicCapacitor => todo!(), // Add 3 orb slots
            Relic::Girya(amt) => self.self_effects.apply_buff(Buff::Strength(Number(*amt as i16))),
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
