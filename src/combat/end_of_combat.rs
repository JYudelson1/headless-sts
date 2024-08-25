use crate::{relics::Relic, state::State, utils::NotImplemented};

use super::{combat_fns::CombatOver, Combat};

impl Combat {
    pub fn check_if_over(&self) -> CombatOver {
        for enemy in &self.enemies {
            if !enemy.is_dead() {
                return CombatOver::No;
            }
        }
        return CombatOver::Yes;
    }
}

impl State {
    pub fn end_of_combat_effects(&mut self) {
        // Burning blood
        if self.relics.contains(Relic::BurningBlood) {
            self.heal(6);
        }
        if self.relics.contains(Relic::BlackBlood) {
            self.heal(12);
        }
    }

    pub fn end_combat(&mut self) -> Result<(), NotImplemented> {
        if self.is_in_combat() {
            // Game effects
                self.end_of_combat_effects();
                // Undo temporary card effects
                for card in &mut self.main_deck {
                    card.reset_end_combat();
                }
                // Move to rewards screen
                self.combat_finished()?
        } else {
            panic!("Should not try to end combat outside of combat!")
        }

        Ok(())
    }
}
