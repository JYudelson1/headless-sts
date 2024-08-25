mod card_actions;
mod combat_fns;
mod enemy_actions;
mod relic_effects;
mod setup_combat;
mod end_of_combat;

pub use combat_fns::CombatOver;
pub use setup_combat::{get_enemies, Elites};

use crate::{
    cardrewardrng::CombatType,
    cards::MasterCard,
    effects::Effects,
    enemies::{ConcreteEnemy, EnemyType},
    relics::{Relic, Relics},
    screens::VisibleStates,
    state::State,
    utils::{NotImplemented, Number},
};

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Combat {
    pub self_effects: Effects,
    enemies: Vec<ConcreteEnemy>,
    turn: u16,
    pub current_energy: u8,
    max_energy: u8,
    pub self_block: Number,
    pub combat_type: CombatType,
    deck: Vec<MasterCard>,
    pub hand: Vec<MasterCard>,
    discard: Vec<MasterCard>,
    exhaust: Vec<MasterCard>,
}

impl Combat {
    pub fn new(
        enemies: Vec<EnemyType>,
        combat_type: CombatType,
        ascension: u8,
        relics: &mut Relics,
        deck: &Vec<MasterCard>,
    ) -> Result<Self, NotImplemented> {
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

        let mut concrete_enemies = vec![];
        for enemy in enemies {
            let concrete_enemy = enemy.new(ascension)?;
            concrete_enemies.push(concrete_enemy);
        }

        // Neow's lament
        if relics.trigger_neow() {
            for enemy in &mut concrete_enemies {
                enemy.current_hp = 1;
            }
        }

        let mut deck = deck.clone();
        deck.shuffle(&mut rand::thread_rng());

        let combat = Self {
            self_effects: Effects::new(),
            enemies: concrete_enemies,
            turn: 0,
            self_block: Number(0),
            max_energy,
            current_energy: max_energy,
            combat_type,
            hand: vec![],
            discard: vec![],
            exhaust: vec![],
            deck
        };

        Ok(combat)
    }

    pub fn num_enemies(&self) -> usize {
        self.enemies.len()
    }
}

impl State {
    pub fn start_combat_turn(&mut self) -> Result<CombatOver, NotImplemented> {
        let combat = self.get_combat();
        combat.turn += 1;
        let turn = combat.turn;

        // Lose all block
        // Except with calipers
        let relics = &self.relics.clone();
        let combat = self.get_combat();
        combat.block_goes_away(relics);

        // Reset relics
        self.relics.reset_start_of_turn();

        // TODO: Do effects that change strength/dex/focus here

        // TODO: Start of turn power effects

        // Set energy equal to max
        let has_ice_cream = self.relics.contains(Relic::IceCream);
        let combat = self.get_combat();
        if has_ice_cream {
            combat.current_energy += combat.max_energy;
        } else {
            combat.current_energy = combat.max_energy;
        }

        // Start of combat relics
        if turn == 1 {
            self.start_turn_1_effects();
        }
        // Other start turn relic effects
        self.start_every_turn_effects();

        // Draw 5 cards
        self.get_combat().draw(5, &relics)
    }

    pub fn get_combat(&mut self) -> &mut Combat {
        if let VisibleStates::Combat(combat) = &mut self.visible_screen {
            combat
        } else {
            panic!("Cannot access combat from outside of combat!")
        }
    }

    pub fn is_in_combat(&self) -> bool {
        matches!(self.visible_screen, VisibleStates::Combat(_))
    }

    pub fn end_turn_effects(&mut self) {
        if let VisibleStates::Combat(combat) = &mut self.visible_screen {
            for relic in &mut self.relics.list {
                combat._end_of_turn_relic(relic);
            }
        }
    }

    pub fn end_turn(&mut self) -> Result<CombatOver, NotImplemented> {
        // End of turn effects
        self.end_turn_effects();
        // Discard every card that doesn't retain
        // If you don't have Runic Pyramid
        let relics = &self.relics.clone();
        let (combat_over, hp_loss) = self.get_combat().discard_hand_end_of_turn(relics)?;
        self.lose_hp(hp_loss.0);
        if combat_over == CombatOver::Yes {return Ok(CombatOver::Yes);}

        // All timed debuffs go down
        self.get_combat().self_effects.increment_turn();

        // Beginning of opponent's turn effects (e.g. poison)
        let relics = &self.relics.clone();
        let combat_over = self.get_combat().begin_enemy_turn(relics)?;
        if combat_over == CombatOver::Yes {return Ok(CombatOver::Yes);}

        // Enemies lose all block
        self.get_combat().enemies_lose_block();
        // Apply opponent's intent
        let combat_over = self.enemy_actions();
        if combat_over == CombatOver::Yes {return Ok(CombatOver::Yes);}
        // Change opponent's intent
        self.get_combat().cycle_enemy_intents();
        // End of enemy turn effects (e.g. metallicize)
        self.get_combat().end_enemies_turn();
        // Start your next turn
        let combat_over = self.start_combat_turn()?;
        if combat_over == CombatOver::Yes {return Ok(CombatOver::Yes);}

        Ok(CombatOver::No)
    }
}
