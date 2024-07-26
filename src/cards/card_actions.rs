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
    AddAngerToDiscard(bool), // bool: upgraded or not
    UpgradeACardInHand,
    UpgradeAllCardsInHand,
    BodySlam,
    GainTempStrength(Number),
    ApplyBuff(Buff),
    ExhaustRandomCard,
    ExhaustSelectedCard,
    ShuffleWoundIntoDraw,
    GainEnergy(u8), // TODO: Other things cards can do
}
