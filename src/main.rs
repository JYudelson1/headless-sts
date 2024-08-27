#![deny(unused_results)]
#![deny(unused_must_use)]
#![allow(dead_code)]

use std::collections::HashMap;

use headless_sts::{
    utils::{Character, StillPlaying},
    State,
};
use rand::{seq::SliceRandom, thread_rng};

fn play_one_game() -> StillPlaying {
    let mut state = State::new(Character::Ironclad, 0);
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

fn play_one_game_lookahead() -> (StillPlaying, usize) {
    let mut state = State::new(Character::Ironclad, 0);
    //println!("{}", state.map);
    let mut length = 0;
    while state.still_playing == StillPlaying::Playing {
        let actions = state.get_actions();
        match actions {
            Ok(mut actions) => {
                actions.shuffle(&mut thread_rng());

                let mut found_one = false;
                for action in actions.iter() {
                    let mut new_state = state.clone();
                    new_state.apply_action(action.clone());
                    if new_state.still_playing != StillPlaying::Playing {
                        continue;
                    } else {
                        found_one = true;
                        state = new_state;
                        length += 1;
                        break;
                    }
                }
                if !found_one {
                    state.apply_action(actions[0].clone());
                }
            }
            Err(err) => return (StillPlaying::NotImplementedError(err), length),
        }
    }
    (state.still_playing, length)
}

fn main() {
    let mut results = HashMap::new();
    let mut longest = 0;
    for _ in lazy_pbar::pbar(0..10_000) {
        let (res, length) = play_one_game_lookahead();
        *results.entry(res).or_insert(0) += 1;
        if length > longest {
            longest = length
        }
    }

    let mut res_vec: Vec<_> = results.iter().collect();
    res_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{res_vec:#?}");
    println!("Longest: {longest}");
}
