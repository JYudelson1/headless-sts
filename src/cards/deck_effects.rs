use crate::{relics::Relic, state::State, utils::{number_between, NotImplemented}};

use super::{make_card, CardName, CardType, MasterCard};

impl State {
    pub fn upgrade_card_in_deck(&mut self, card_id: uuid::Uuid) {
        for card in &mut self.main_deck {
            if card.id == card_id {
                card.upgrade();
                return;
            }
        }
        panic!("No card with that ID exists!")
    }

    pub fn remove_card_in_deck(&mut self, card_id: uuid::Uuid) {
        let mut index = None;
        for (i, card) in self.main_deck.iter().enumerate() {
            if card.id == card_id {
                index = Some(i);
            }
        }
        match index {
            Some(i) => {
                self.main_deck.remove(i);
            }
            None => panic!("No card with that ID exists!"),
        }
    }

    pub fn duplicate_card_in_deck(&mut self, card_id: uuid::Uuid) {
        for card in &self.main_deck {
            if card.id == card_id {
                let dupe = card.card().duplicate();
                self.add_to_deck(dupe);
                return;
            }
        }
    }

    pub fn transform_card_in_deck(&mut self, card_id: uuid::Uuid) -> Result<(), NotImplemented> {
        let mut index = None;
        for (i, card) in self.main_deck.iter().enumerate() {
            if card.id == card_id {
                index = Some(i);
            }
        }
        match index {
            Some(i) => {
                let card = self.main_deck.remove(i);
                // TODO: Treat colorless cards correctly here
                let new_card = if card.card().get_type() == CardType::Curse {
                    let curses = CardName::transform_curses();
                    let i = number_between(0, curses.len() - 1);
                    curses[i]
                } else {
                    let cards = CardName::transform_cards(self.character);
                    let i = number_between(0, cards.len() - 1);
                    cards[i]
                };
                let new_card = make_card(new_card, false)?;
                self.add_to_deck(new_card);
            }
            None => panic!("No card with that ID exists!"),
        }
        Ok(())
    }

    pub fn add_to_deck(&mut self, mut card: MasterCard) {
        let card_type = card.card().get_type();
        match card_type {
            CardType::Attack => {
                if self.relics.contains(Relic::MoltenEgg) {
                    card.upgrade();
                }
            }
            CardType::Power => {
                if self.relics.contains(Relic::ToxicEgg) {
                    card.upgrade();
                }
            }
            CardType::Skill => {
                if self.relics.contains(Relic::FrozenEgg) {
                    card.upgrade();
                }
            }
            CardType::Status => (),
            CardType::Curse => {
                // Omamori
                if self.relics.try_use_omamori() {
                    return;
                }
                // Periapt
                if self.relics.contains(Relic::DarkstonePeriapt) {
                    self.increase_max_hp(6);
                }
            }
        }

        self.main_deck.push(card);

        // Ceramic Fish
        if self.relics.contains(Relic::CeramicFish) {
            self.gold += 9;
        }
    }
}
