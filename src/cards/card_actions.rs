use crate::{combat::CardInHandPurpose, effects::{Buff, Debuff}, utils::Number};

use super::{CardName, MasterCard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Targets {
    All,
    One,
    Random,
}

#[derive(Debug)]
pub enum CardActions {
    Damage((Number, Targets)),
    ApplyVulnerable((Number, Targets)),
    ApplyWeak((Number, Targets)),
    DebuffEnemy((Debuff, Targets)),
    Block(Number),
    Draw(u8),
    LoseHealth(u16),
    UpgradeACardInHand,
    UpgradeAllCardsInHand,
    BodySlam,
    ApplyBuff(Buff),
    ApplyDebuff(Debuff),
    ExhaustRandomCard,
    ShuffleCardToPile((MasterCard, Pile)),
    AddCardToHand(MasterCard),
    AddFreshCardToHand((CardName, bool)),
    GainEnergy(u8),
    IncreaseMaxEnergy,
    DoubleBlock,
    Havoc,
    PerfectedStrike(Number),
    HeavyBlade(Number),
    ChooseNCards((CardInHandPurpose, usize, Option<Vec<CardActions>>)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pile {
    Draw,
    Discard,
}
