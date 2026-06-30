use std::{cell::{Ref, RefCell, RefMut}, fmt::Debug, rc::Rc};

use uuid::Uuid;

use super::{card::CardType, card_actions::CardActions, make_card, CardName};

pub trait Card: Debug {
    fn name(&self) -> CardName;
    fn get_type(&self) -> CardType;
    fn reset(&mut self) {}
    fn get_cost(&self) -> u8; 

    fn set_upgraded(&mut self, to_set: bool);
    fn upgrade(&mut self) {
        self.set_upgraded(true)
    }
    fn set_upgraded_amt(&mut self, amt: u16) {
        if amt == 0 {
            self.set_upgraded(false)
        }
        if amt == 1 {
            self.set_upgraded(true)
        }
    }
    fn can_be_upgraded(&self) -> bool;
    fn is_upgraded(&self) -> bool;

    fn is_playable(&self, _hand: &Vec<MasterCard>) -> bool {
        true
    }
    fn targets(&self) -> bool {
        false
    }
    fn exhausts(&self) -> bool {
        false
    }
    fn is_ethereal(&self) -> bool {
        false
    }
    fn retains(&self) -> bool {
        false
    }
    fn can_be_removed(&self) -> bool {
        true
    }
    fn is_a_strike(&self) -> bool {
        false
    }

    fn play_upgraded(&mut self) -> Vec<CardActions>;
    fn play_unupgraded(&mut self) -> Vec<CardActions>;

    fn play(&mut self) -> Vec<CardActions> {
        if self.is_upgraded() {
            self.play_upgraded()
        } else {
            self.play_unupgraded()
        }
    }

    fn duplicate(&self) -> MasterCard {
        // TODO: Temp changes, like cost change
        make_card(self.name(), self.is_upgraded()).expect("Card must be implemented to suplicate")
    }

    fn clone(&self, id: Uuid) -> MasterCard {
        let mut card = self.duplicate();
        card.id = id;
        card
    }
}

#[derive(Debug)]
pub struct MasterCard {
    pub card: Rc<RefCell<dyn Card>>,
    pub id: uuid::Uuid,
    pub upgraded: u16,
}

impl MasterCard {
    pub fn reset_end_combat(&mut self) {
        self.card.as_ref().borrow_mut().reset();
        self.card
            .as_ref()
            .borrow_mut()
            .set_upgraded_amt(self.upgraded);
    }

    pub fn card<'a>(&'a self) -> Ref<'a, dyn Card + 'a> {
        self.card.as_ref().borrow()
    }

    pub fn card_mut<'a>(&'a mut self) -> RefMut<'a, dyn Card + 'a> {
        self.card.as_ref().borrow_mut()
    }

    pub fn upgrade(&mut self) {
        self.upgraded += 1;
        self.card_mut().upgrade();
    }
}

impl Clone for MasterCard {
    fn clone(&self) -> Self {
        self.card().clone(self.id)
    }
}
