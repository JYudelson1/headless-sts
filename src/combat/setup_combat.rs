use std::vec;

use crate::{cardrewardrng::CombatType, enemies::EnemyType, utils::{number_between, Act}};

use rand::distributions::Distribution;
use rand::prelude::SliceRandom;

pub fn get_enemies(
    act: &Act,
    combat_type: CombatType,
    last_elite: Option<Elites>,
    fights_this_act: u8,
) -> Vec<EnemyType> {
    match act {
        Act::Act1 => {
            match combat_type {
                CombatType::Normal => {
                    // Easy fights for first three encounters
                    if fights_this_act <= 3 {
                        act_1_easy_pool()
                    } else {
                        act_1_hard_pool()
                    }
                }
                CombatType::Elite => act_1_elite(last_elite),
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

fn act_1_hard_pool() -> Vec<EnemyType> {
    let lice = vec![EnemyType::GreenLouse, EnemyType::RedLouse];

    let mut exo_thugs = vec![[
        EnemyType::GreenLouse,
        EnemyType::RedLouse,
        EnemyType::AcidSlimeM,
        EnemyType::SpikeSlimeM,
    ]
    .choose(&mut rand::thread_rng())
    .unwrap()
    .clone()];
    exo_thugs.push(
        [
            EnemyType::SlaverBlue,
            EnemyType::SlaverRed,
            EnemyType::Looter,
            EnemyType::Cultist,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone(),
    );

    let mut exo_wildlife = vec![[EnemyType::FungusBeast, EnemyType::JawWorm]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()];
    exo_wildlife.push(
        [
            EnemyType::SlaverBlue,
            EnemyType::SlaverRed,
            EnemyType::Looter,
            EnemyType::Cultist,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone(),
    );

    let possible_fights: Vec<Vec<EnemyType>> = vec![
        // Gremlin Gang
        vec![
            EnemyType::GremlinMad,
            EnemyType::GremlinMad,
            EnemyType::GremlinSneaky,
            EnemyType::GremlinSneaky,
            EnemyType::GremlinFat,
            EnemyType::GremlinFat,
            EnemyType::GremlinWizard,
            EnemyType::GremlinShield,
        ]
        .choose_multiple(&mut rand::thread_rng(), 4)
        .cloned()
        .collect(),
        // Large Slime
        vec![EnemyType::AcidSlimeL, EnemyType::SpikeSlimeL]
            .choose_multiple(&mut rand::thread_rng(), 1)
            .cloned()
            .collect(),
        // Lots of slimes
        vec![
            EnemyType::SpikeSlimeS,
            EnemyType::SpikeSlimeS,
            EnemyType::SpikeSlimeS,
            EnemyType::AcidSlimeS,
            EnemyType::AcidSlimeS,
        ],
        // Blue Slaver
        vec![EnemyType::SlaverBlue],
        // Red Slaver
        vec![EnemyType::SlaverRed],
        // 3 Louses
        vec![
            lice.choose(&mut rand::thread_rng()).unwrap().clone(),
            lice.choose(&mut rand::thread_rng()).unwrap().clone(),
            lice.choose(&mut rand::thread_rng()).unwrap().clone(),
        ],
        // 2 Fungi
        vec![EnemyType::FungusBeast, EnemyType::FungusBeast],
        // Exordium Thugs
        exo_thugs,
        // Exordium Wildlife
        exo_wildlife,
        // Looter
        vec![EnemyType::Looter],
    ];
    let weights = vec![625, 1225, 625, 1225, 625, 1225, 1225, 9375, 9375, 1225];
    let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();
    possible_fights[dist.sample(&mut rand::thread_rng())].clone()
}

fn act_1_elite(last_elite: Option<Elites>) -> Vec<EnemyType> {
    let elites = vec![Elites::Lagavulin, Elites::Sentries, Elites::GremlinNob];

    // Elites cannot happen twice in a row——we solve this by tracking the last elite.
    let elite = loop {
        let i = number_between(0, 2);
        let e = elites[i];
        if Some(e) != last_elite {
            break e;
        }
    };

    match elite {
        Elites::Lagavulin => vec![EnemyType::Lagavulin],
        Elites::Sentries => vec![EnemyType::SentryA, EnemyType::SentryB, EnemyType::SentryA],
        Elites::GremlinNob => vec![EnemyType::GremlinNob],
    }
}

fn act_1_boss() -> Vec<EnemyType> {
    let bosses = [
        EnemyType::Hexaghost,
        EnemyType::TheGuardian,
        EnemyType::SlimeBoss,
    ];
    let boss = bosses[number_between(0, 2)];
    vec![boss]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elites {
    Lagavulin,
    Sentries,
    GremlinNob,
    // TODO: Add acts 2 & 3 here
}
