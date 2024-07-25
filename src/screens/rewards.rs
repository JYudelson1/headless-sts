use crate::{cards::CardName, potions::Potion};

pub struct RewardsScreen(pub Vec<Reward>);

pub enum Reward {
    Gold(u16),
    Relic, // TODO: Relics
    Potion(Potion),
    CardReward,
}

pub struct CardReward {
    pub card: CardName,
    pub is_upgraded: bool,
}
