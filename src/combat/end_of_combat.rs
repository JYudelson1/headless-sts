use crate::{enemies::EnemyIndex, relics::Relic, screens::VisibleStates, state::State, utils::Number};

use super::Combat;

impl Combat {
    pub fn check_if_over(&self) -> bool {
        for enemy in &self.enemies {
            if !enemy.is_dead() {
                return false;
            }
        }
        return true;
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

    pub fn maybe_end_combat(&mut self) {
        if let VisibleStates::Combat(combat) = &self.visible_screen {
            if combat.check_if_over() {
                // Game effects
                self.end_of_combat_effects();
                // Undo temporary card effects
                for card in &mut self.main_deck {
                    card.reset_end_combat();
                }
                // Move to rewards screen
                self.combat_finished()
            }
        } else {
            panic!("Should not try to end combat outside of combat!")
        }
    }
}
