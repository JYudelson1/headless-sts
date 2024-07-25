use crate::{cards::CardIndex, enemies::EnemyIndex, map::RoomNode};

#[derive(PartialEq, Eq, Clone)]
pub enum Action {
    PlayUntargetedCard(CardIndex),
    PlayTargetedCard((CardIndex, EnemyIndex)),
    CollectReward(RewardChoice),
    MakeCardChoice(CardRewardChoice),
    EndTurn,
    TraverseMap(u8),
    MakeNeowChoice(usize),
    MakeRestChoice(RestChoice),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RestChoice {
    Skip,
    Smith,
    Rest,
    Toke,
    TakeRubyKey,
    Lift,
    Dig,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RewardChoice {
    Skip,
    RewardIndex(usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CardRewardChoice {
    Skip,
    CardRewardIndex(usize),
}
