use crate::{cards::CardName, potions::Potion, state::State};

use super::VisibleStates;

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

impl State {
    pub fn make_rewards_screen(&mut self) -> RewardsScreen {
        if let VisibleStates::Combat(combat) = &self.visible_screen {
            // TODO: Maybe dovetail this with opening treasure?
            todo!()
            // TODO: Prayer wheel
        } else {
            panic!("Should only make rewards after being in combat!")
        }
    }
}
