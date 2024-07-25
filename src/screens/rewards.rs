use crate::{cardrewardrng::CombatType, cards::CardName, potions::Potion, relics::Relic, state::State};

use super::VisibleStates;

pub struct RewardsScreen(pub Vec<Reward>);

pub enum Reward {
    Gold(u32),
    Relic(Relic),
    Potion(Potion),
    CardReward(CombatType),
}

pub struct CardReward {
    pub card: CardName,
    pub is_upgraded: bool,
}

impl State {
    pub fn make_rewards_screen(&mut self) -> RewardsScreen {
        let combat = self.get_combat();
        let mut rewards = vec![];

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
