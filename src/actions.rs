use crate::{cards::CardIndex, enemies::EnemyIndex, map::RoomNode};

#[derive(PartialEq, Eq)]
pub enum Action {
    PlayUntargetedCard(CardIndex),
    PlayTargetedCard((CardIndex, EnemyIndex)),
    CollectReward(RewardChoice),
    MakeCardChoice(CardRewardChoice),
    EndTurn,
    TraverseMap(RoomNode),
    MakeNeowChoice(usize),
    Lift,
    Toke,
    Rest,
    Smith,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RewardChoice {
    Skip,
    RewardIndex(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CardRewardChoice {
    Skip,
    CardRewardIndex(usize),
}
