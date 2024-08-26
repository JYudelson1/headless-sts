use crate::{cards::CardName, relics::Relic, screens::CardReward, state::State, utils::{Act, Character}};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CombatType {
    Normal,
    Elite,
    Boss,
}

#[derive(Debug, Clone)]
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

    fn get_one_reward(
        &mut self,
        combat_type: CombatType,
        act: &Act,
        character: Character,
    ) -> CardReward {
        if matches!(combat_type, CombatType::Boss) {
            self.reset();
            return CardReward {
                card: CardName::random_rare(character),
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
            CardName::random_common(character)
        } else if rng < common + uncommon {
            CardName::random_uncommon(character)
        } else {
            self.reset();
            CardName::random_rare(character)
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
        character: Character,
    ) -> Vec<CardReward> {
        let mut cards: Vec<CardReward> = vec![];

        for _ in 0..num_cards {
            // Check to make sure there are no duplicate cards
            let card = loop {
                let maybe_card = self.get_one_reward(combat_type, act, character);
                let mut usable = true;
                for card in cards.iter() {
                    if card.card == maybe_card.card {
                        usable = false;
                        break;
                    }
                }
                if usable {
                    break maybe_card;
                }
            };
            cards.push(card);
        }

        cards
    }

    pub fn get_noncombat_card_choice(
        &mut self,
        num_cards: usize,
        character: Character,
    ) -> Vec<CardReward> {
        // This doesn't take the offset into account
        let inner = self.0;

        let mut cards: Vec<CardReward> = vec![];

        for _ in 0..num_cards {
            self.0 = 0.0;
            // This ensures we don't get dupes
            let new_card = loop {
                let  maybe_card = self.get_one_reward(CombatType::Normal, &Act::Act1, character);
                let mut usable = true;
                for card in cards.iter() {
                    if card.card == maybe_card.card {
                        usable = false;
                        break;
                    }
                }
                if usable {
                    break maybe_card;
                }
            };
            cards.push(new_card);
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
        self.card_rng.get_rewards(num_cards, combat_type, &self.act, self.character)
    }
}
