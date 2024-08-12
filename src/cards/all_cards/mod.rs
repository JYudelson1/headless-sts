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
        CardName::BodySlam => Rc::new(RefCell::new(ironclad::bodyslam::BodySlam(upgraded))),
        CardName::Bludgeon => Rc::new(RefCell::new(ironclad::bludgeon::Bludgeon(upgraded))),
        CardName::Impervious => Rc::new(RefCell::new(ironclad::impervious::Impervious(upgraded))),
        CardName::Clash => todo!(),
        CardName::Flex => todo!(),
        CardName::Havoc => todo!(),
        CardName::Headbutt => todo!(),
        CardName::HeavyBlade => todo!(),
        CardName::IronWave => Rc::new(RefCell::new(ironclad::ironwave::IronWave(upgraded))),
        CardName::PerfectedStrike => todo!(),
        CardName::PommelStrike => Rc::new(RefCell::new(ironclad::pommelstrike::PommelStrike(upgraded))),
        CardName::SwordBoomerang => Rc::new(RefCell::new(ironclad::swordboomerang::SwordBoomerang(upgraded))),
        CardName::ThunderClap => Rc::new(RefCell::new(ironclad::thunderclap::ThunderClap(upgraded))),
        CardName::TrueGrit => todo!(),
        CardName::TwinStrike => todo!(),
        CardName::WarCry => todo!(),
        CardName::WildStrike => todo!(),
        CardName::BattleTrance => Rc::new(RefCell::new(ironclad::battletrance::BattleTrance(upgraded))),
        CardName::BloodForBlood => todo!(),
        CardName::BurningPact => todo!(),
        CardName::Combust => todo!(),
        CardName::DarkEmbrace => todo!(),
        CardName::Disarm => todo!(),
        CardName::Dropkick => todo!(),
        CardName::DualWield => todo!(),
        CardName::Entrench => todo!(),
        CardName::Evolve => Rc::new(RefCell::new(ironclad::evolve::Evolve(upgraded))),
        CardName::FeelNoPain => todo!(),
        CardName::FireBreathing => todo!(),
        CardName::FlameBarrier => todo!(),
        CardName::HemoKinesis => todo!(),
        CardName::InfernalBlade => todo!(),
        CardName::Inflame => todo!(),
        CardName::Intimidate => todo!(),
        CardName::Metallicize =>  Rc::new(RefCell::new(ironclad::metallicize::Metallicize(upgraded))),
        CardName::PowerThrough => todo!(),
        CardName::Pummel => todo!(),
        CardName::Rage => todo!(),
        CardName::Rampage => Rc::new(RefCell::new(ironclad::rampage::Rampage::new(upgraded))),
        CardName::RecklessCharge => todo!(),
        CardName::Rupture => todo!(),
        CardName::SearingBlow => todo!(),
        CardName::SecondWind => todo!(),
        CardName::SeeingRed => Rc::new(RefCell::new(ironclad::seeingred::SeeingRed(upgraded))),
        CardName::Sentinel => todo!(),
        CardName::SeverSoul => todo!(),
        CardName::Shockwave => todo!(),
        CardName::SpotWeakness => todo!(),
        CardName::Uppercut => todo!(),
        CardName::Whirlwind => todo!(),
        CardName::Berserk => todo!(),
        CardName::Brutality => todo!(),
        CardName::Corruption => todo!(),
        CardName::DemonForm => Rc::new(RefCell::new(ironclad::demonform::DemonForm(upgraded))),
        CardName::DoubleTap => todo!(),
        CardName::Exhume => todo!(),
        CardName::Feed => todo!(),
        CardName::FiendFire => todo!(),
        CardName::Juggernaut => todo!(),
        CardName::LimitBreak => todo!(),
        CardName::Offering => Rc::new(RefCell::new(ironclad::offering::Offering(upgraded))),
        CardName::Reaper => todo!(),
        CardName::Slimed => Rc::new(RefCell::new(shared::slimed::Slimed(upgraded))),
        CardName::Burn => Rc::new(RefCell::new(shared::burn::Burn)),
        CardName::Wound => Rc::new(RefCell::new(shared::wound::Wound)),
        CardName::Dazed => Rc::new(RefCell::new(shared::dazed::Dazed)),
        CardName::AscendersBane => todo!(),
        CardName::Clumsy => Rc::new(RefCell::new(shared::clumsy::Clumsy)),
        CardName::CurseOfTheBell => todo!(),
        CardName::Decay => todo!(),
        CardName::Doubt => todo!(),
        CardName::Injury => todo!(),
        CardName::Necronomicurse => todo!(),
        CardName::Normality => todo!(),
        CardName::Pain => todo!(),
        CardName::Parasite => todo!(),
        CardName::Pride => todo!(),
        CardName::Regret => todo!(),
        CardName::Shame => todo!(),
        CardName::Writhe => todo!(),
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
        upgraded,
    }
}
