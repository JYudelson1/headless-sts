// TODO: Figure out whether to have different blessings if you die early

use core::num;

use rand::Rng;

use crate::utils::number_between;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FirstBlessing {
    Remove,
    Transform,
    Upgrade,
    ChooseClassCard,
    ChooseUncommonColorless,
    RandomRare,
}

impl FirstBlessing {
    pub fn random() -> Self {
        let x = number_between(0, 6);
        let variants = [
            Self::Remove,
            Self::Transform,
            Self::Upgrade,
            Self::ChooseClassCard,
            Self::ChooseUncommonColorless,
            Self::RandomRare,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SecondBlessing {
    MaxHP,
    NeowsLament,
    RandomCommonRelic,
    Gold100,
    Random3Potions,
}

impl SecondBlessing {
    pub fn random() -> Self {
        let x = number_between(0, 5);
        let variants = [
            Self::MaxHP,
            Self::NeowsLament,
            Self::RandomCommonRelic,
            Self::Gold100,
            Self::Random3Potions,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThirdUpside {
    Remove2,
    Transform2,
    Gold250,
    ChooseRareClassCard,
    ChooseRareColorless,
    BigMaxHP,
}

impl ThirdUpside {
    pub fn random() -> Self {
        let x = number_between(0, 6);
        let variants = [
            Self::Remove2,
            Self::Transform2,
            Self::Gold250,
            Self::ChooseRareClassCard,
            Self::ChooseRareColorless,
            Self::BigMaxHP,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThirdDownside {
    LoseMaxHealth,
    RandomCurse,
    TakeDamage,
    LoseAllGold,
}

impl ThirdDownside {
    pub fn random() -> Self {
        let x = number_between(0, 4);
        let variants = [
            Self::LoseMaxHealth,
            Self::RandomCurse,
            Self::TakeDamage,
            Self::LoseAllGold,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThirdBlessing {
    pub upside: ThirdUpside,
    pub downside: ThirdDownside,
}

impl ThirdBlessing {
    pub fn random() -> Self {
        loop {
            let upside = ThirdUpside::random();
            let downside = ThirdDownside::random();

            // Forbidden combos
            if (upside == ThirdUpside::BigMaxHP && downside == ThirdDownside::LoseMaxHealth)
                || (upside == ThirdUpside::Gold250 && downside == ThirdDownside::LoseAllGold)
                || (upside == ThirdUpside::Remove2 && downside == ThirdDownside::RandomCurse)
            {
                continue;
            } else {
                return Self { upside, downside };
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NeowsBlessing {
    First(FirstBlessing),
    Second(SecondBlessing),
    Third(ThirdBlessing),
    RelicSwap,
}

pub fn get_neow_blessings() -> [NeowsBlessing; 4]{
    let first = NeowsBlessing::First(FirstBlessing::random());
    let second = NeowsBlessing::Second(SecondBlessing::random());
    let third = NeowsBlessing::Third(ThirdBlessing::random());

    [first, second, third, NeowsBlessing::RelicSwap]
}
