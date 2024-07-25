mod all_cards;
mod card;
mod card_actions;
mod card_trait;
mod starter_decks;

pub use card::CardName;
pub use card_actions::CardActions;
use card_trait::Card;
pub use card_trait::MasterCard;
pub use starter_decks::make_starter_deck;

#[derive(PartialEq, Eq)]
pub struct CardIndex(pub u8);

pub struct CardInHand {
    pub card: Box<dyn Card>,
    pub id: uuid::Uuid,
}
