use crate::state::State;

impl State {
    pub fn upgrade_card_in_deck(&mut self, card_id: uuid::Uuid) {
        for card in &mut self.main_deck {
            if card.id == card_id {
                card.upgraded += 1;
                card.card_mut().upgrade();
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
}
