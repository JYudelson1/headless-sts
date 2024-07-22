mod actions;
mod card;
mod cardnames;
mod combat;
mod effects;
mod enemies;
mod map;
mod potions;
mod relics;
mod screens;
mod state;
mod utils;

fn main() {
    let mut state = state::State::new(utils::Character::Ironclad, 0);
}
