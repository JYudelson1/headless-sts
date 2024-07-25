use std::rc::Rc;

use super::{card::CardType, card_actions::CardActions, CardName};

pub trait Card {
    // TODO: Figure out upgrading
    fn name(&self) -> CardName;
    fn get_type(&self) -> CardType;
    fn reset(&mut self) {}

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
    fn play(&mut self) -> Vec<CardActions>;
}

#[derive(Clone)]
pub struct MasterCard {
    pub card: Rc<dyn Card>,
    pub id: uuid::Uuid,
}
