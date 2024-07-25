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

fn main() {
    let mut state = state::State::new(utils::Character::Ironclad, 0);
    let action: Action = unimplemented!();
    state.apply_action(action);
}
