use crate::{effects::{Buff, Debuff}, utils::Number};

use super::MasterCard;

pub enum Targets {
    All,
    One,
    Random,
}

pub enum CardActions {
    Damage((Number, Targets)),
    ApplyVulnerable((Number, Targets)),
    ApplyWeak((Number, Targets)),
    Block(Number),
    Draw(u8),
    LoseHealth(u16),
    UpgradeACardInHand,
    UpgradeAllCardsInHand,
    BodySlam,
    GainTempStrength(Number),
    ApplyBuff(Buff),
    ApplyDebuff(Debuff),
    ExhaustRandomCard,
    ExhaustSelectedCard,
    ShuffleCardToPile((MasterCard, Pile)),
    GainEnergy(u8), // TODO: Other things cards can do
    IncreaseMaxEnergy,
    DoubleBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pile {
    Draw,
    Discard,
}
