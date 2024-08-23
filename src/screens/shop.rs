use crate::{
    cards::{make_card, CardName},
    potions::Potion,
    relics::{Relic, Relics},
    state::State,
    utils::NotImplemented,
};

use super::VisibleStates;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Wares {
    Card((CardName, u32)),
    Relic((Relic, u32)),
    Potion((Potion, u32)),
    Removal(u32),
}

impl Wares {
    pub fn new(relics: &Relics, prev_removals: u8) -> Vec<Self> {
        let mut wares = vec![];

        // Card Remove
        let removal_cost = if relics.contains(Relic::SmilingMask) {
            50
        } else {
            75 + (25 * prev_removals)
        };
        wares.push(Self::Removal(removal_cost as u32));

        // Class cards

        // Colorless Cards

        // Relics

        // Potions

        wares
    }
}

impl State {
    pub fn buy_wares(&mut self, ware: Wares) -> Result<(), NotImplemented> {
        // TODO: Break Maw Bank
        match ware {
            Wares::Card((card, cost)) => {
                let card = make_card(card, false)?;
                self.add_to_deck(card);
                self.gold -= cost;
            }
            Wares::Relic((relic, cost)) => {
                self.collect_relic(relic);
                self.gold -= cost
            }
            Wares::Potion((potion, cost)) => {
                self.potions.add(potion);
                self.gold -= cost
            }
            Wares::Removal(cost) => {
                self.gold -= cost;
                // TODO: Should be able to return after removing
                self.visible_screen = VisibleStates::RemoveCardScreen(1);
            }
        }

        Ok(())
    }
}
