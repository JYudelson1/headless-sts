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

    pub fn enemy_lose_hp(&mut self, enemy_index: EnemyIndex, mut amt: u16) {
        let has_the_boot = self.relics.contains(Relic::TheBoot);
        let combat = self.get_combat();
        let enemy = &mut combat.enemies[enemy_index.0];
        if enemy.effects.is_intangible() {
            amt = 1;
        }
        if has_the_boot && amt < 5 {
            amt = 5;
        }

        if amt <= enemy.current_hp {
            enemy.current_hp -= amt;
        } else {
            enemy.current_hp = 0;
        }
        self.maybe_end_combat();
    }

    pub fn damage_enemy(&mut self, enemy_index: EnemyIndex, mut amt: u16) {
        let combat = self.get_combat();
        let enemy = &mut combat.enemies[enemy_index.0];

        if amt < enemy.current_block.0 as u16 {
            enemy.current_block -= Number(amt as i16);
        } else {
            amt -= enemy.current_block.0 as u16;
            enemy.current_block = Number(0);
        }

        self.enemy_lose_hp(enemy_index, amt)
    }
}
