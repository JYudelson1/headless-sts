use crate::{cards::CardName, relics::Relic, screens::CardReward, state::State, utils::Act};

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

    fn get_one_reward(&mut self, combat_type: CombatType, act: &Act) -> CardReward {
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
        combat_type: CombatType,
        act: &Act,
    ) -> Vec<CardReward> {
        let mut cards = vec![];

        for _ in 0..num_cards {
            cards.push(self.get_one_reward(combat_type, act));
        }

        cards
    }

    pub fn get_noncombat_choice(&mut self, num_cards: usize, act: Act) -> Vec<CardReward> {
        // This doesn't take the offset into account
        let inner = self.0;

        let mut cards = vec![];

        for _ in 0..num_cards {
            self.0 = 0.0;
            cards.push(self.get_one_reward(CombatType::Normal, &act));
        }

        self.0 = inner;

        cards
    }
}

impl State {
    pub fn get_card_rewards(&mut self, combat_type: CombatType) -> Vec<CardReward> {
        let mut num_cards = 3;
        if self.relics.contains(Relic::BrokenCrown) {
            num_cards -= 2;
        }
        if self.relics.contains(Relic::QuestionCard) {
            num_cards += 1;
        }
        self.card_rng.get_rewards(num_cards, combat_type, &self.act)
    }
}
