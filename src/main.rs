use actions::Action;

mod actions;
mod cardrewardrng;
mod cards;
mod combat;
mod effects;
mod enemies;
mod map;
mod potions;
mod relic_pickup;
mod relics;
mod screens;
mod state;
mod utils;

use rand::seq::SliceRandom;

fn main() {
    let mut state = state::State::new(utils::Character::Ironclad, 0);
    while state.still_playing {
        let mut actions = state.get_actions();
        actions.shuffle(&mut rand::thread_rng());
        let random_action = &actions[0];
        state.apply_action(random_action.clone());
    }
}
