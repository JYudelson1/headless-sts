pub mod potion_bag;
pub mod potion_effects;
pub mod potion_rng;

use rand::{random, seq::SliceRandom, thread_rng};

use crate::{enemies::EnemyIndex, relics::Relic, state::State};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Potion {
    BlockPotion,
    StrengthPotion,
    LiquidBronze,
    FruitJuice, // TODO: add the rest
}

impl Potion {
    pub fn is_noncombat(&self) -> bool {
        match self {
            Potion::BlockPotion => false,
            Potion::StrengthPotion => false,
            Potion::LiquidBronze => false,
            Potion::FruitJuice => true,
        }
    }

    pub fn is_combat(&self) -> bool {
        match self {
            Potion::BlockPotion => true,
            Potion::StrengthPotion => true,
            Potion::LiquidBronze => true,
            Potion::FruitJuice => true,
        }
    }

    pub fn targets(&self) -> bool {
        match self {
            Potion::BlockPotion => false,
            Potion::StrengthPotion => false,
            Potion::LiquidBronze => false,
            Potion::FruitJuice => false,
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
    pub fn use_potion(&mut self, index: usize, target: Option<EnemyIndex>) {
        let potion = self.potions.remove_potion(index);
        match target {
            Some(enemy) => self.use_targeted_potion(potion, enemy),
            None => self.use_untargeted_potion(potion),
        }

        if self.relics.contains(Relic::ToyOrnithopter) {
            self.heal(5);
        }
    }

    pub fn discard_potion(&mut self, index: usize) {
        self.potions.remove_potion(index);
    }
}

const COMMON: &[Potion] = &[Potion::BlockPotion, Potion::StrengthPotion];
const UNCOMMON: &[Potion] = &[Potion::LiquidBronze];
const RARE: &[Potion] = &[Potion::FruitJuice];
