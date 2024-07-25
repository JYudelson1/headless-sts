pub use bash::Bash;
pub use defend::Defend;
pub use strike::Strike;

use super::{card_trait::{Card, MasterCard}, CardName};

mod bash;
mod defend;
mod strike;

pub fn make_card(name: CardName) -> MasterCard {
    let card: Box<dyn Card> = match name {
        CardName::Strike => Box::new(Strike),
        CardName::Defend => Box::new(Defend),
        CardName::Bash => Box::new(Bash),
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
    }
}
