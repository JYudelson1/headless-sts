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
        let actions = state.get_actions();
        match actions {
            Ok(mut actions) => {
                actions.shuffle(&mut rand::thread_rng());
                //println!("Actions: {actions:?}");
                let random_action = &actions[0];
                //println!("Action: {random_action:?}");
                state.apply_action(random_action.clone());
            },
            Err(err) => return StillPlaying::NotImplementedError(err),
        }
        
    }
    state.still_playing
}

fn main() {
    let mut results = HashMap::new();
    for _ in lazy_pbar::pbar(0..100_000) {
        let res = play_one_game();
        *results.entry(res).or_insert(0) += 1;
    }

    let mut res_vec: Vec<_> = results.iter().collect();
    res_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{res_vec:#?}")
}
