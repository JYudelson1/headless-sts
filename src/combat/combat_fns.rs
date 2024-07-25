use crate::{effects::Effects, relics::Relic, screens::VisibleStates, state::State, utils::Number};

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
