#![allow(dead_code)]

mod actions;
mod cardrewardrng;
mod cards;
mod combat;
mod effects;
mod enemies;
mod map;
mod potions;
mod question_rng;
mod relic_pickup;
mod relics;
mod screens;
mod state;
mod utils;

use std::collections::HashMap;

use rand::seq::SliceRandom;
use utils::StillPlaying;

fn play_one_game() -> StillPlaying {
    let mut state = state::State::new(utils::Character::Ironclad, 0);
    //println!("{}", state.map);
    while state.still_playing == StillPlaying::Playing {
        let mut actions = state.get_actions();
        actions.shuffle(&mut rand::thread_rng());
        let random_action = &actions[0];
        //println!("Action: {random_action:?}");
        state.apply_action(random_action.clone());
    }
    state.still_playing
}

fn main() {
    let mut results = HashMap::new();
    for _ in 0..1000 {
        let res = play_one_game();
        match results.get_mut(&res) {
            Some(num) => *num += 1,
            None => {
                results.insert(res, 1);
            }
        }
    }

    println!("{results:?}")
}
