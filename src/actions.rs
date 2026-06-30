use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{cards::CardIndex, enemies::EnemyIndex, relics::Relic, screens::{EventAction, Wares}, utils::Key};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    PlayUntargetedCard(CardIndex),
    PlayTargetedCard((CardIndex, EnemyIndex)),
    CollectReward(RewardChoice),
    MakeCardChoice(CardRewardChoice),
    EndTurn,
    TraverseMap(u8),
    MakeNeowChoice(usize),
    MakeRestChoice(RestChoice),
    Upgrade(Uuid),
    Remove(Uuid),
    Duplicate(Uuid),
    Transform(Uuid),
    Purchase(Wares),
    LeaveShop,
    EventAction(EventAction),
    TakeRelicLeave(Relic),
    TakeKeyLeave(Key),
    UsePotionNoTargets(usize),
    UsePotionTargets((usize, EnemyIndex)),
    DiscardPotion(usize),
    ChooseCardInHand(Uuid),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum RestChoice {
    Skip,
    Smith,
    Rest,
    Toke,
    TakeRubyKey,
    Lift,
    Dig,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RewardChoice {
    Skip,
    RewardIndex(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CardRewardChoice {
    Skip,
    CardRewardIndex(usize),
}
