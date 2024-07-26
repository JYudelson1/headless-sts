use crate::{
    cards::{CardName, Targets},
    effects::{Debuff, Effects},
    enemies::EnemyIndex,
    relics::Relic,
    screens::VisibleStates,
    state::State,
    utils::Number,
};

use super::Combat;
use rand::seq::SliceRandom;

impl State {
    pub fn heal(&mut self, mut amt: u16) {
        if matches!(self.visible_screen, VisibleStates::Combat(_))
            && self.relics.contains(Relic::MagicFlower)
        {
            amt = (amt as f32 * 1.5).ceil() as u16;
        }
        if self.relics.contains(Relic::MarkOfTheBloom) {
            amt = 0;
        }

        self.current_health += amt;
        if self.current_health > self.max_health.0 as u16 {
            self.current_health = self.max_health.0 as u16
        }
    }

    pub fn increase_max_hp(&mut self, amt: u16) {
        self.max_health += Number(amt as i16);
        self.heal(amt);
    }

    pub fn lose_hp(&mut self, mut amt: u16) {
        // If you are in combat & have intangible, all hp loss goes to 1
        if let VisibleStates::Combat(combat) = &self.visible_screen {
            if combat.self_effects.is_intangible() {
                amt = 1;
            }
        }

        if amt >= self.current_health {
            // Player would die
            // TODO: Check for lizard tail
            // TODO: Check for fairy in a bottle
            // TODO: Show that you lose
            println!("Player is dead!");
            self.still_playing = false;
        } else {
            self.current_health -= amt;
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

    fn direct_damage_enemy(&mut self, enemy_index: EnemyIndex, mut amt: u16) {
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

    fn attack_damage_enemy(&mut self, enemy_index: EnemyIndex, mut amt: u16) {
        // TODO: Check for thorns

        self.direct_damage_enemy(enemy_index, amt);
    }

    pub fn damage_enemy(
        &mut self,
        damage_amt: Number,
        target_type: Targets,
        target: Option<EnemyIndex>,
    ) {
        let self_effects = &self.get_combat().self_effects.clone();
        let enemies = &self.get_combat().enemies;
        let mut damages: Vec<(EnemyIndex, u16)> = vec![];
        match target_type {
            Targets::All => {
                // Calculate damage and apply it for each enemy individually, in order
                for (i, enemy) in enemies.iter().enumerate() {
                    let total_damage = calculate_damage(self_effects, &enemy.effects, damage_amt);
                    damages.push((EnemyIndex(i), total_damage.0 as u16))
                }
            }
            Targets::One => {
                let enemy = &enemies[target.unwrap().0];
                let total_damage = calculate_damage(self_effects, &enemy.effects, damage_amt);
                damages.push((target.unwrap(), total_damage.0 as u16))
            }
            Targets::Random => todo!(),
        }

        for (enemy_index, amt) in damages {
            self.attack_damage_enemy(enemy_index, amt);
        }
    }

    fn debuff_one_enemy(&mut self, debuff: Debuff, enemy_index: EnemyIndex) {
        let combat = self.get_combat();
        let enemy = &mut combat.enemies[enemy_index.0];
        enemy.effects.apply_debuff(debuff);
    }

    pub fn debuff_enemy(
        &mut self,
        debuff: Debuff,
        target_type: Targets,
        enemy_index: Option<EnemyIndex>,
    ) {
        // TODO: Effects that alter debuffs
        // TODO: Champion's belt
        let num_enemies = self.get_combat().enemies.len();
        match target_type {
            Targets::All => {
                for i in 0..num_enemies {
                    self.debuff_one_enemy(debuff, EnemyIndex(i));
                }
            }
            Targets::One => self.debuff_one_enemy(debuff, enemy_index.unwrap()),
            Targets::Random => todo!(),
        }
    }

    fn begin_enemy_turn_effects(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.get_combat().enemies[enemy_index.0];
        // Poison
        let poison = enemy.effects.get_poison();
        if poison.0 > 0 {
            self.enemy_lose_hp(enemy_index, poison.0 as u16)
        }
    }

    pub fn begin_enemy_turn(&mut self) {
        let num_enemies = self.get_combat().num_enemies();
        for i in 0..num_enemies {
            self.begin_enemy_turn_effects(EnemyIndex(i));
        }
    }
}

pub fn calculate_damage(
    source_effects: &Effects,
    target_effects: &Effects,
    damage: Number,
) -> Number {
    let mut damage = damage.0 as f32;
    // Factor in strength
    damage += source_effects.get_strength().0 as f32;
    // Factor in vulnerability
    if target_effects.is_vulnerable() {
        match source_effects.relevant_relics.contains(&Relic::PaperPhrog) {
            true => damage *= 1.75,
            false => damage *= 1.5,
        }
    }
    // Factor in weakness
    if source_effects.is_weak() {
        match target_effects.relevant_relics.contains(&Relic::PaperKrane) {
            true => damage *= 0.6,
            false => damage *= 0.75,
        }
    }

    Number(damage.floor() as i16)
}

impl Combat {
    pub fn reshuffle(&mut self) {
        self.deck.append(&mut self.discard);
        self.discard = vec![];
        self.deck.shuffle(&mut rand::thread_rng());
    }

    fn draw_1(&mut self) {
        // Cannot draw if all cards are in hand
        if self.deck.is_empty() && self.discard.is_empty() {
            return;
        }
        // Cannot draw if hand is full
        if self.hand.len() >= 10 {
            return;
        }
        // If draw pile is empty, reshuffle
        if self.deck.is_empty() {
            self.reshuffle()
        }
        // Take the top card from deck and move to hand
        let top_card = self.deck.remove(0);
        let name = top_card.card().name();
        self.hand.push(top_card);

        // On Draw Effects
        // TODO: Deus Ex
        // Void
        if name == CardName::Void {
            if self.current_energy > 0 {
                self.current_energy -= 1;
            }
        }
    }

    pub fn draw(&mut self, amt: u8) {
        for _ in 0..amt {
            self.draw_1();
        }
    }

    pub fn gain_block(&mut self, amt: Number) {
        self.self_block += amt;
        // TODO: Block effects
        // TODO: Juggernaut
    }

    pub fn discard_hand_end_of_turn(&mut self) {
        let hand_size = self.hand.len();

        for i in (0..hand_size).rev() {
            // If the card is ethereal, exhaust it
            if self.hand[i].card().is_ethereal() {
                self.exhaust.push(self.hand.remove(i));
            } else if !self.hand[i].card().retains() {
                // Else discard if not retained
                self.discard.push(self.hand.remove(i));
            } {
                
            }

            
        }
    }

    pub fn block_goes_away(&mut self) {
        // Barricade
        if self.self_effects.barricade {
            return;
        }

        // Calipers
        if self.has_relic(&Relic::Calipers) && self.self_block.0 > 15 {
            self.self_block -= Number(15)
        } else {
            self.self_block = Number(0)
        }
    }

    fn enemy_loses_block(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.enemies[enemy_index.0];
        if enemy.effects.barricade {
            return;
        }
        enemy.current_block = Number(0)
    }

    pub fn enemies_lose_block(&mut self) {
        for i in 0..self.num_enemies() {
            self.enemy_loses_block(EnemyIndex(i));
        }
    }

    fn enemy_end_of_turn(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.enemies[enemy_index.0];
        // Metallicize
        let metal = enemy.effects.get_metallicize();
        enemy.current_block += metal;
    }

    pub fn end_enemies_turn(&mut self) {
        for i in 0..self.num_enemies() {
            self.enemy_end_of_turn(EnemyIndex(i))
        }
    }
}
