use crate::{
    cards::CardName, effects::Effects, relics::Relic, screens::VisibleStates, state::State,
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

    pub fn end_turn(&mut self) {
        // End of turn effects

        // Discard every card that doesn't retain
        // If you don't have Runic Pyramid

        // Beginning of opponent's turn effects (e.g. poison)

        // Apply opponent's intent
    }
}
