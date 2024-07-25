use std::{cell::{Ref, RefCell}, rc::Rc};

use super::{card::CardType, card_actions::CardActions, CardName};

pub trait Card {
    fn name(&self) -> CardName;
    fn get_type(&self) -> CardType;
    fn reset(&mut self) {}

    fn set_upgraded(&mut self, to_set: bool);
    fn upgrade(&mut self) {self.set_upgraded(true)}
    fn can_be_upgraded(&self) -> bool;
    fn is_upgraded(&self) -> bool;

    fn is_playable(&self) -> bool {
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

    fn play_upgraded(&mut self) -> Vec<CardActions>;
    fn play_unupgraded(&mut self) -> Vec<CardActions>;

    fn play(&mut self) -> Vec<CardActions> {
        if self.is_upgraded() {
            self.play_upgraded()
        } else {
            self.play_unupgraded()
        }
    }
}

#[derive(Clone)]
pub struct MasterCard {
    pub card: Rc<RefCell<dyn Card>>,
    pub id: uuid::Uuid,
    pub upgraded: bool,
}

impl MasterCard {
    pub fn reset_end_combat(&mut self) {
        self.card.as_ref().borrow_mut().reset();
        self.card.as_ref().borrow_mut().set_upgraded(self.upgraded);
    }

    pub fn card(&self) -> Ref<dyn Card> {
        self.card.as_ref().borrow()
    }
}
