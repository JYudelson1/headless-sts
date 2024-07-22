use crate::cardnames::CardName;

pub struct CardIndex(pub u8);
pub struct CardInHand {
    card: CardName,
    index: CardIndex,
}
