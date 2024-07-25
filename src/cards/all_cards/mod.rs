pub use bash::Bash;
pub use defend::Defend;
pub use strike::Strike;

use super::{card_trait::MasterCard, CardName};

mod bash;
mod defend;
mod strike;

pub fn make_card(name: CardName) -> MasterCard {
    let card = match name {
        CardName::Strike => Box::new(Strike),
        CardName::Defend => todo!(),
        CardName::Bash => todo!(),
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
    }
}
