use crate::{cardrewardrng::CombatType, cards::CardName, potions::Potion, relics::Relic, state::State, utils::number_between};

use super::VisibleStates;

#[derive(Debug, Clone)]
pub struct RewardsScreen(pub Vec<Reward>);

#[derive(Debug, Clone)]
pub enum Reward {
    Gold(u32),
    Relic(Relic),
    Potion(Potion),
    CardReward(CombatType),
}

#[derive(Debug, Clone)]
pub struct CardReward {
    pub card: CardName,
    pub is_upgraded: bool,
}

impl State {
    pub fn make_rewards_screen(&mut self) -> RewardsScreen {
        let combat_type = self.get_combat().combat_type;
        let mut rewards = vec![];

        let gold = match combat_type {
            // TODO: Ascensions change this I think
            CombatType::Normal => number_between(10, 20),
            CombatType::Elite => number_between(25, 35),
            CombatType::Boss => number_between(95, 105),
        };
        rewards.push(Reward::Gold(gold));

        if combat_type == CombatType::Elite {
            rewards.push(Reward::Relic(self.relics.random_elite()));
        }

        // Potion Rng Logic
        if self.relics.contains(Relic::WhiteBeastStatue) || self.potion_rng.maybe_get_potion() {
            rewards.push(Reward::Potion(Potion::random().0));
        }

        rewards.push(Reward::CardReward(combat_type));

        if self.relics.contains(Relic::PrayerWheel) && combat_type == CombatType::Normal {
            rewards.push(Reward::CardReward(combat_type));
        }

        RewardsScreen(rewards)
    }

    pub fn take_reward(&mut self, index: usize) {
        let reward = if let VisibleStates::Reward(rewards_screen) = &mut self.visible_screen {
            rewards_screen.0.remove(index)
        } else {
            panic!("You should not call take_reward except on the rewards screen!");
        };

        match reward {
            Reward::Gold(amt) => self.gold += amt,
            Reward::Relic(relic) => self.collect_relic(relic),
            Reward::Potion(potion) => self.potions.add(potion),
            Reward::CardReward(combat_type) => {
                let rewards = self.get_card_rewards(combat_type);
                let screen = VisibleStates::CardReward(rewards);
                self.visible_screen = screen;
            }
        }
    }
}
