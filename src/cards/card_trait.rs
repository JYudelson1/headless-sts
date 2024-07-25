use super::{card::CardType, card_actions::CardActions, CardName};

pub trait Card {
    // TODO: Figure out upgrading
    fn name(&self) -> CardName;
    fn get_type(&self) -> CardType;
    fn reset(&mut self) {}

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

pub struct MasterCard {
    pub card: Box<dyn Card>,
    pub id: uuid::Uuid,
}
