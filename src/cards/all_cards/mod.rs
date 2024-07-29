mod defect;
mod ironclad;
mod shared;
mod silent;
mod watcher;

use std::{cell::RefCell, rc::Rc};

use super::{
    card_trait::{Card, MasterCard},
    CardName,
};

pub fn make_card(name: CardName, upgraded: bool) -> MasterCard {
    let card: Rc<RefCell<dyn Card>> = match name {
        CardName::Strike => Rc::new(RefCell::new(shared::strike::Strike(upgraded))),
        CardName::Defend => Rc::new(RefCell::new(shared::defend::Defend(upgraded))),
        CardName::Bash => Rc::new(RefCell::new(ironclad::bash::Bash(upgraded))),
        CardName::Void => Rc::new(RefCell::new(shared::void::Void)),
        CardName::ShrugItOff => Rc::new(RefCell::new(ironclad::shrugitoff::ShrugItOff(upgraded))),
        CardName::Cleave => Rc::new(RefCell::new(ironclad::cleave::Cleave(upgraded))),
        CardName::Clothesline => Rc::new(RefCell::new(ironclad::clothesline::Clothesline(upgraded))),
        CardName::Carnage => Rc::new(RefCell::new(ironclad::carnage::Carnage(upgraded))),
        CardName::Anger => Rc::new(RefCell::new(ironclad::anger::Anger(upgraded))),
        CardName::GhostlyArmor => Rc::new(RefCell::new(ironclad::ghostlyarmor::GhostlyArmor(upgraded))),
        CardName::Barricade => Rc::new(RefCell::new(ironclad::barricade::Barricade(upgraded))),
        CardName::Armaments => Rc::new(RefCell::new(ironclad::armaments::Armaments(upgraded))),
        CardName::Bloodletting => Rc::new(RefCell::new(ironclad::bloodletting::Bloodletting(upgraded))),
        CardName::BodySlam => todo!(),
        CardName::Bludgeon => Rc::new(RefCell::new(ironclad::bludgeon::Bludgeon(upgraded))),
        CardName::Impervious => Rc::new(RefCell::new(ironclad::impervious::Impervious(upgraded))),
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
        upgraded,
    }
}
