mod neow;
mod rewards;

pub use neow::*;
pub use rewards::{CardReward, RewardsScreen};

use crate::{actions::{Action, CardRewardChoice, RewardChoice}, combat::Combat, map::Map};

pub enum VisibleStates {
    Reward(RewardsScreen),
    CardReward(Vec<CardReward>),
    Neow([NeowsBlessing; 4]),
    Map(Map),
    Combat(Combat),
}

impl VisibleStates {
    pub fn new() -> Self {
        Self::Neow(get_neow_blessings())
    }

    pub fn get_actions(&self) -> Vec<Action> {
        let mut actions = vec![];
        match self {
            VisibleStates::Reward(rewards) => {
                actions.push(Action::CollectReward(RewardChoice::Skip));
                for i in 0..rewards.0.len() {
                    actions.push(Action::CollectReward(RewardChoice::RewardIndex(i)));
                }
            }
            VisibleStates::CardReward(card_rewards) => {
                actions.push(Action::MakeCardChoice(CardRewardChoice::Skip));
                for i in 0..card_rewards.len() {
                    actions.push(Action::MakeCardChoice(CardRewardChoice::CardRewardIndex(i)));
                }
            }
            VisibleStates::Neow(_) => {
                // TODO: Support for only one choice
                for i in 0..4 {
                    actions.push(Action::MakeNeowChoice(i));
                }
            }
            VisibleStates::Map(map) => {
                for node in map.next_rooms() {
                    actions.push(Action::TraverseMap(node));
                }
            },
            VisibleStates::Combat(_) => todo!(),
        }

        actions
    }
}
