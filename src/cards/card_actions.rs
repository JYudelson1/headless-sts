use crate::{effects::Buff, utils::Number};

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
    AddAngerToDiscard,
    UpgradeACardInHand,
    UpgradeAllCardsInHand,
    BodySlam,
    GainTempStrength(Number),
    ApplyBuff(Buff),
    ExhaustRandomCard,
    ExhaustSelectedCard,
    ShuffleWoundIntoDraw, // TODO: Other things cards can do
}
