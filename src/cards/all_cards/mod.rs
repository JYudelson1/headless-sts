mod bash;
mod carnage;
mod cleave;
mod clothesline;
mod defend;
mod shrugitoff;
mod strike;
mod void;

use std::{cell::RefCell, rc::Rc};

pub use bash::Bash;
pub use carnage::Carnage;
pub use cleave::Cleave;
pub use clothesline::Clothesline;
pub use defend::Defend;
pub use shrugitoff::ShrugItOff;
pub use strike::Strike;
pub use void::Void;

use super::{
    card_trait::{Card, MasterCard},
    CardName,
};

pub fn make_card(name: CardName, upgraded: bool) -> MasterCard {
    let card: Rc<RefCell<dyn Card>> = match name {
        CardName::Strike => Rc::new(RefCell::new(Strike(upgraded))),
        CardName::Defend => Rc::new(RefCell::new(Defend(upgraded))),
        CardName::Bash => Rc::new(RefCell::new(Bash(upgraded))),
        CardName::Void => Rc::new(RefCell::new(Void)),
        CardName::ShrugItOff => Rc::new(RefCell::new(ShrugItOff(upgraded))),
        CardName::Cleave => Rc::new(RefCell::new(Cleave(upgraded))),
        CardName::Clothesline => Rc::new(RefCell::new(Clothesline(upgraded))),
        CardName::Carnage => Rc::new(RefCell::new(Carnage(upgraded))),
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
        upgraded,
    }
}
