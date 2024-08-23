use crate::{relics::{Relic, Relics}, utils::number_between};

#[derive(Debug)]
pub struct Chest {
    pub gold: u32,
    size: ChestType,
    pub relic: ChestRelicType,
}

impl Chest {
    pub fn new_random(has_sapphire_key: bool, relics: &mut Relics) -> Self {
        let size = ChestType::new_random();
        let gold = size.get_gold();

        let rarity = size.get_relic_rarity();
        let relic = ChestRelicType::new_random(rarity, relics, has_sapphire_key);

        Self { gold, size, relic }
    }
}

#[derive(Debug)]
pub enum ChestType {
    Small,
    Medium,
    Large,
}

impl ChestType {
    pub fn new_random() -> Self {
        let x = rand::random::<f32>();

        if x < 0.50 {
            Self::Small
        } else if x < 0.83 {
            Self::Medium
        } else {
            Self::Large
        }
    }

    fn get_gold(&self) -> u32 {
        match self._has_gold() {
            true => self._gold_amt(),
            false => 0,
        }
    }

    fn _has_gold(&self) -> bool {
        let threshold = match self {
            ChestType::Small => 0.5,
            ChestType::Medium => 0.35,
            ChestType::Large => 0.5,
        };
        let x = rand::random::<f32>();
        if x < threshold {
            true
        } else {
            false
        }
    }

    fn _gold_amt(&self) -> u32 {
        match self {
            ChestType::Small => number_between(23, 27),
            ChestType::Medium => number_between(45, 55),
            ChestType::Large => number_between(68, 82),
        }
    }

    fn get_relic_rarity(&self) -> ChestRelicRarity {
        let common_chance = match self {
            ChestType::Small => 0.75,
            ChestType::Medium => 0.35,
            ChestType::Large => 0.0,
        };

        let uncommon_chance = match self {
            ChestType::Small => 0.25,
            ChestType::Medium => 0.5,
            ChestType::Large => 0.75,
        };

        let x = rand::random::<f32>();

        if x < common_chance {
            ChestRelicRarity::Common
        } else if x < common_chance + uncommon_chance {
            ChestRelicRarity::Uncommon
        } else {
            ChestRelicRarity::Rare
        }
    }
}

#[derive(Debug)]
pub enum ChestRelicType {
    None,
    Relic(Relic),
    RelicOrKey(Relic),
}

#[derive(Debug)]
enum ChestRelicRarity {
    Common,
    Uncommon,
    Rare,
}

impl ChestRelicType {
    fn new_random(
        rarity: ChestRelicRarity,
        relics: &mut Relics,
        has_sapphire_key: bool,
    ) -> Self {
        if relics.contains(Relic::NlothsHungryFace(true)) {
            return Self::None;
        }

        let relic = match rarity {
            ChestRelicRarity::Common => relics.random_common(),
            ChestRelicRarity::Uncommon => relics.random_uncommon(),
            ChestRelicRarity::Rare => relics.random_rare(),
        };

        match has_sapphire_key {
            true => Self::Relic(relic),
            false => Self::RelicOrKey(relic),
        }
    }
}
