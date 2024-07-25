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
            // E.G. If symbiiotic virus was obtained before nuclear battery, you should
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
}

impl Combat {
    fn _start_of_combat_relic(&mut self, relic: &mut Relic) {
        match relic {
            Relic::PaperPhrog => self.add_relic(Relic::PaperPhrog),
            Relic::PaperKrane => self.add_relic(Relic::PaperKrane),
            Relic::Ginger => self.add_relic(Relic::Ginger),
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
                    enemy.get_effects().apply_debuff(debuff);
                }
            }
            Relic::RedMask => {
                let debuff = Debuff::Weak(Number(1));
                for enemy in &mut self.enemies {
                    enemy.get_effects().apply_debuff(debuff);
                }
            }
            Relic::DataDisk => self.self_effects.apply_buff(Buff::Focus(Number(1))),
            Relic::Anchor => self.self_block += Number(10),
            Relic::AncientTeaSet(rested) => {
                if *rested {
                    self.current_energy += 2;
                }
            }
            Relic::BronzeScales => self.self_effects.apply_buff(Buff::Thorns(Number(3))),
            Relic::Lantern => self.current_energy += 1,
            Relic::SmoothStone => self.self_effects.apply_buff(Buff::Dexterity(Number(1))),
            Relic::NinjaScroll => todo!(),    // Add 3 shivs to hand
            Relic::RunicCapacitor => todo!(), // Add 3 orb slots
            _ => (),
        }
    }
}
