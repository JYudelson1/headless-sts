use crate::{cards::CardName, screens::CardReward, utils::Act};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CombatType {
    Normal,
    Elite,
    Boss,
}

pub struct CardRewardRng(f32);

impl CardRewardRng {
    pub fn new() -> Self {
        Self(-0.05)
    }

    fn increase(&mut self) {
        let new = if self.0 < 0.4 { self.0 + 0.01 } else { 0.4 };
        *self = Self(new);
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn get_one_reward(&mut self, combat_type: &CombatType, act: &Act) -> CardReward {
        if matches!(combat_type, CombatType::Boss) {
            self.reset();
            return CardReward {
                card: CardName::random_rare(),
                is_upgraded: false,
            };
        }
        let rng = rand::random::<f32>();

        let (mut common, mut uncommon, rare) = if matches!(combat_type, CombatType::Normal) {
            (0.6, 0.37, 0.03)
        } else {
            (0.5, 0.4, 0.1)
        };

        if rare + self.0 < 0.0 {
            uncommon += rare + self.0;
        }
        common -= self.0;

        let card = if rng < common {
            self.increase();
            CardName::random_common()
        } else if rng < common + uncommon {
            CardName::random_uncommon()
        } else {
            self.reset();
            CardName::random_rare()
        };

        // TODO: Change upgraded card odds after A12
        let rng = rand::random::<f32>();
        let upgraded_chance = match act {
            Act::Act1 => 0.0,
            Act::Act2 => 0.25,
            Act::Act3 => 0.5,
        };
        CardReward {
            card: card,
            is_upgraded: rng < upgraded_chance,
        }
    }

    pub fn get_rewards(
        &mut self,
        num_cards: usize,
        combat_type: &CombatType,
        act: &Act,
    ) -> Vec<CardReward> {
        let mut cards = vec![];

        for _ in 0..num_cards {
            cards.push(self.get_one_reward(combat_type, act));
        }

        cards
    }
}
