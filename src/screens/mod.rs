mod neow;
mod rewards;

use neow::NeowsBlessing;
pub use rewards::{CardReward, RewardsScreen};

use crate::actions::{Action, CardRewardChoice, RewardChoice};

pub enum VisibleStates {
    Reward(RewardsScreen),
    CardReward(Vec<CardReward>),
    Neow([NeowsBlessing; 4]),
}

impl VisibleStates {
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
                for i in 0..4 {
                    actions.push(Action::MakeNeowChoice(i));
                }
            }
        }

        actions
    }
}
