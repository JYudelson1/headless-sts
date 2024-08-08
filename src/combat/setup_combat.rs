use crate::{cardrewardrng::CombatType, enemies::EnemyType, utils::Act};

use rand::prelude::SliceRandom;

pub fn get_enemies(act: &Act, combat_type: CombatType) -> Vec<EnemyType> {
    // TODO: Is there something weird about elite enemies? Like they dont happen twice?
    match act {
        Act::Act1 => {
            match combat_type {
                CombatType::Normal => {
                    // TODO: Only call this for first three encounters
                    act_1_easy_pool()
                }
                CombatType::Elite => todo!(),
                CombatType::Boss => todo!(),
            }
        }
        Act::Act2 => match combat_type {
            CombatType::Normal => todo!(),
            CombatType::Elite => todo!(),
            CombatType::Boss => todo!(),
        },
        Act::Act3 => match combat_type {
            CombatType::Normal => todo!(),
            CombatType::Elite => todo!(),
            CombatType::Boss => todo!(),
        },
    }
}

fn act_1_easy_pool() -> Vec<EnemyType> {
    let slime_m = vec![EnemyType::AcidSlimeM, EnemyType::SpikeSlimeM]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone();
    let slime_s = vec![EnemyType::AcidSlimeS, EnemyType::SpikeSlimeS]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone();
    let lice = vec![EnemyType::GreenLouse, EnemyType::RedLouse];
    let louse_1 = lice.choose(&mut rand::thread_rng()).unwrap().clone();
    let louse_2 = lice.choose(&mut rand::thread_rng()).unwrap().clone();

    let possible_fights = vec![
        vec![EnemyType::JawWorm],
        vec![EnemyType::Cultist],
        vec![slime_m, slime_s],
        vec![louse_1, louse_2],
    ];

    possible_fights.choose(&mut rand::thread_rng()).unwrap().clone()
}
