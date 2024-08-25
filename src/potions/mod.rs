pub mod potion_bag;
pub mod potion_effects;
pub mod potion_rng;

use rand::{random, seq::SliceRandom, thread_rng};

use crate::{enemies::EnemyIndex, relics::Relic, state::State, utils::NotImplemented};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Potion {
    Block,
    Strength,
    LiquidBronze,
    FruitJuice,
    Ancient,
    Attack,
    BlessingOfTheForge,
    Blood,
    Colorless,
    Cultist,
    Dex,
    DistilledChaos,
    Duplication,
    Elixer,
    Energy,
    EntropicBrew,
    EssenceOfSteel,
    Explosive,
    FairyInABottle,
    Fear,
    Fire,
    Flex,
    GamblersBrew,
    HeartOfIron,
    LiquidMemories,
    Power,
    Regen,
    Skill,
    SmokeBomb,
    SneckoOil,
    Speed,
    Swift,
    Weak,
}

impl Potion {
    pub fn is_noncombat(&self) -> bool {
        match self {
            Potion::Block => false,
            Potion::Strength => false,
            Potion::LiquidBronze => false,
            Potion::FruitJuice => true,
            Potion::Ancient => false,
            Potion::Attack => false,
            Potion::BlessingOfTheForge => false,
            Potion::Blood => true,
            Potion::Colorless => false,
            Potion::Cultist => false,
            Potion::Dex => false,
            Potion::DistilledChaos => false,
            Potion::Duplication => false,
            Potion::Elixer => false,
            Potion::Energy => false,
            Potion::EntropicBrew => true,
            Potion::EssenceOfSteel => false,
            Potion::Explosive => false,
            Potion::FairyInABottle => true,
            Potion::Fear => false,
            Potion::Fire => false,
            Potion::Flex => false,
            Potion::GamblersBrew => false,
            Potion::HeartOfIron => false,
            Potion::LiquidMemories => false,
            Potion::Power => false,
            Potion::Regen => false,
            Potion::Skill => false,
            Potion::SmokeBomb => false,
            Potion::SneckoOil => false,
            Potion::Speed => false,
            Potion::Swift => false,
            Potion::Weak => false,
        }
    }

    pub fn is_combat(&self) -> bool {
        true
    }

    pub fn targets(&self) -> bool {
        match self {
            Potion::Block => false,
            Potion::Strength => false,
            Potion::LiquidBronze => false,
            Potion::FruitJuice => false,
            Potion::Ancient => false,
            Potion::Attack => false,
            Potion::BlessingOfTheForge => false,
            Potion::Blood => false,
            Potion::Colorless => false,
            Potion::Cultist => false,
            Potion::Dex => false,
            Potion::DistilledChaos => false,
            Potion::Duplication => false,
            Potion::Elixer => false,
            Potion::Energy => false,
            Potion::EntropicBrew => false,
            Potion::EssenceOfSteel => false,
            Potion::Explosive => false,
            Potion::FairyInABottle => false,
            Potion::Fear => true,
            Potion::Fire => true,
            Potion::Flex => false,
            Potion::GamblersBrew => false,
            Potion::HeartOfIron => false,
            Potion::LiquidMemories => false,
            Potion::Power => false,
            Potion::Regen => false,
            Potion::Skill => false,
            Potion::SmokeBomb => false,
            Potion::SneckoOil => false,
            Potion::Speed => false,
            Potion::Swift => false,
            Potion::Weak => true,
        }
    }

    pub fn random() -> Self {
        let x = random::<f32>();
        let pool = if x < 0.5 {
            COMMON
        } else if x <= 0.83 {
            UNCOMMON
        } else {
            RARE
        };
        *pool.choose(&mut thread_rng()).unwrap()
    }
}
impl State {
    pub fn use_potion(
        &mut self,
        index: usize,
        target: Option<EnemyIndex>,
    ) -> Result<(), NotImplemented> {
        let potion = self.potions.remove_potion(index);
        match target {
            Some(enemy) => self.use_targeted_potion(potion, enemy)?,
            None => self.use_untargeted_potion(potion)?,
        }

        if self.relics.contains(Relic::ToyOrnithopter) {
            self.heal(5);
        }

        Ok(())
    }

    pub fn discard_potion(&mut self, index: usize) {
        self.potions.remove_potion(index);
    }
}

const COMMON: &[Potion] = &[
    Potion::Block,
    Potion::Strength,
    Potion::Attack,
    Potion::BlessingOfTheForge,
    Potion::Blood,
    Potion::Colorless,
    Potion::Dex,
    Potion::Energy,
    Potion::Explosive,
    Potion::Fear,
    Potion::Fire,
    Potion::Flex,
    Potion::Power,
    Potion::Skill,
    Potion::Speed,
    Potion::Swift,
    Potion::Weak,
];
const UNCOMMON: &[Potion] = &[
    Potion::LiquidBronze,
    Potion::Ancient,
    Potion::DistilledChaos,
    Potion::Duplication,
    Potion::Elixer,
    Potion::EssenceOfSteel,
    Potion::GamblersBrew,
    Potion::LiquidMemories,
    Potion::Regen,
];
const RARE: &[Potion] = &[
    Potion::FruitJuice,
    Potion::Cultist,
    Potion::EntropicBrew,
    Potion::FairyInABottle,
    Potion::HeartOfIron,
    Potion::SmokeBomb,
    Potion::SneckoOil,
];
