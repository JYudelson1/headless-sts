mod all_cards;
mod card;
mod card_actions;
mod card_trait;
mod starter_decks;
mod deck_effects;

pub use all_cards::make_card;
pub use card::CardName;
pub use card_actions::{CardActions, Pile, Targets};
pub use card_trait::MasterCard;
pub use starter_decks::make_starter_deck;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CardIndex(pub usize);
