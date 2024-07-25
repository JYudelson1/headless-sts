mod card_actions;
mod combat_fns;
mod enemy_actions;
mod relic_effects;
mod setup_combat;

pub use setup_combat::get_enemies;

use crate::{
    cardrewardrng::CombatType,
    effects::Effects,
    enemies::{ConcreteEnemy, EnemyType},
    relics::{Relic, Relics},
    screens::VisibleStates,
    state::State,
    utils::Number,
};

pub struct Combat {
    self_effects: Effects,
    enemies: Vec<ConcreteEnemy>,
    turn: u16,
    current_energy: u8,
    max_energy: u8,
    self_block: Number,
    combat_type: CombatType,
}

impl Combat {
    pub fn new(
        enemies: Vec<EnemyType>,
        combat_type: CombatType,
        ascension: u8,
        relics: &Relics,
    ) -> Self {
        let mut max_energy = 3;

        if relics.contains(Relic::BrokenCrown) {
            max_energy += 1;
        }
        if relics.contains(Relic::CoffeeDripper) {
            max_energy += 1;
        }
        if relics.contains(Relic::FusionHammer) {
            max_energy += 1;
        }
        if relics.contains(Relic::Ectoplasm) {
            max_energy += 1;
        }
        if relics.contains(Relic::RunicDome) {
            max_energy += 1;
        }
        if relics.contains(Relic::PhilosophersStone) {
            max_energy += 1;
        }
        if relics.contains(Relic::CursedKey) {
            max_energy += 1;
        }
        if relics.contains(Relic::MarkOfPain) {
            max_energy += 1;
        }
        if relics.contains(Relic::Sozu) {
            max_energy += 1;
        }
        if relics.contains(Relic::VelvetChoker) {
            max_energy += 1;
        }
        if relics.contains(Relic::SlaversCollar) && combat_type != CombatType::Normal {
            max_energy += 1;
        }

        let enemies = enemies
            .iter()
            .map(|enemy_type| enemy_type.new(ascension))
            .collect();
        Self {
            self_effects: Effects::new(),
            enemies,
            turn: 0,
            self_block: Number(0),
            max_energy,
            current_energy: max_energy,
            combat_type,
        }
    }

    pub fn has_relic(&self, relic: &Relic) -> bool {
        self.self_effects.relevant_relics.contains(relic)
    }

    pub fn add_relic(&mut self, relic: Relic) {
        self.self_effects.relevant_relics.insert(relic);
    }
}

impl State {
    pub fn start_turn_effects(&mut self) {
        if let VisibleStates::Combat(combat) = &mut self.visible_screen {
            combat.turn += 1;
            if combat.turn == 1 {
                self.start_turn_1_effects();
            }
            // Other start turn relic effects
        } else {
            panic!("You should be in combat now!")
        }
    }
}
