use std::rc::Rc;

pub use bash::Bash;
pub use defend::Defend;
pub use shrugitoff::ShrugItOff;
pub use strike::Strike;
pub use void::Void;

use super::{card_trait::{Card, MasterCard}, CardName};

mod bash;
mod defend;
mod shrugitoff;
mod strike;
mod void;

pub fn make_card(name: CardName, upgraded: bool) -> MasterCard {
    let card: Rc<dyn Card> = match name {
        CardName::Strike => Rc::new(Strike(upgraded)),
        CardName::Defend => Rc::new(Defend(upgraded)),
        CardName::Bash => Rc::new(Bash(upgraded)),
        CardName::Void => Rc::new(Void),
        CardName::ShrugItOff => Rc::new(ShrugItOff(upgraded))
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
    }
}
