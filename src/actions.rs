use crate::{card::CardIndex, enemies::EnemyIndex};

pub enum Action {
    PlayUntargetedCard(CardIndex),
    PlayTargetedCard((CardIndex, EnemyIndex)),
    CollectReward(RewardChoice),
    MakeCardChoice(CardRewardChoice),
    EndTurn,
    TraverseMap, // TODO: ???
    MakeNeowChoice(usize),
}

pub enum RewardChoice {
    Skip,
    RewardIndex(usize),
}

pub enum CardRewardChoice {
    Skip,
    CardRewardIndex(usize),
}
