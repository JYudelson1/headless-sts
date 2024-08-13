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
        CardName::TrueGrit => Rc::new(RefCell::new(ironclad::truegrit::TrueGrit(upgraded))),
        CardName::TwinStrike => Rc::new(RefCell::new(ironclad::twinstrike::TwinStrike(upgraded))),
        CardName::WarCry => todo!(),
        CardName::WildStrike => Rc::new(RefCell::new(ironclad::wildstrike::WildStrike(upgraded))),
        CardName::BattleTrance => Rc::new(RefCell::new(ironclad::battletrance::BattleTrance(upgraded))),
        CardName::BloodForBlood => todo!(),
        CardName::BurningPact => Rc::new(RefCell::new(ironclad::burningpact::BurningPact(upgraded))),
        CardName::Combust => todo!(),
        CardName::DarkEmbrace => Rc::new(RefCell::new(ironclad::darkembrace::DarkEmbrace(upgraded))),
        CardName::Disarm => todo!(),
        CardName::Dropkick => todo!(),
        CardName::DualWield => todo!(),
        CardName::Entrench => todo!(),
        CardName::Evolve => Rc::new(RefCell::new(ironclad::evolve::Evolve(upgraded))),
        CardName::FeelNoPain => Rc::new(RefCell::new(ironclad::feelnopain::FeelNoPain(upgraded))),
        CardName::FireBreathing => todo!(),
        CardName::FlameBarrier => todo!(),
        CardName::HemoKinesis => Rc::new(RefCell::new(ironclad::hemokinesis::HemoKinesis(upgraded))),
        CardName::InfernalBlade => todo!(),
        CardName::Inflame => Rc::new(RefCell::new(ironclad::inflame::Inflame(upgraded))),
        CardName::Intimidate => Rc::new(RefCell::new(ironclad::intimidate::Intimidate(upgraded))),
        CardName::Metallicize =>  Rc::new(RefCell::new(ironclad::metallicize::Metallicize(upgraded))),
        CardName::PowerThrough => Rc::new(RefCell::new(ironclad::powerthrough::PowerThrough(upgraded))),
        CardName::Pummel => Rc::new(RefCell::new(ironclad::pummel::Pummel(upgraded))),
        CardName::Rage => todo!(),
        CardName::Rampage => Rc::new(RefCell::new(ironclad::rampage::Rampage::new(upgraded))),
        CardName::RecklessCharge => todo!(),
        CardName::Rupture => todo!(),
        CardName::SearingBlow => Rc::new(RefCell::new(ironclad::searingblow::SearingBlow(upgraded as u16))),
        CardName::SecondWind => todo!(),
        CardName::SeeingRed => Rc::new(RefCell::new(ironclad::seeingred::SeeingRed(upgraded))),
        CardName::Sentinel => todo!(),
        CardName::SeverSoul => todo!(),
        CardName::Shockwave => Rc::new(RefCell::new(ironclad::shockwave::Shockwave(upgraded))),
        CardName::SpotWeakness => todo!(),
        CardName::Uppercut => Rc::new(RefCell::new(ironclad::uppercut::Uppercut(upgraded))),
        CardName::Whirlwind => todo!(),
        CardName::Berserk => Rc::new(RefCell::new(ironclad::berserk::Berserk(upgraded))),
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
        CardName::Burn => Rc::new(RefCell::new(shared::burn::Burn(upgraded))),
        CardName::Wound => Rc::new(RefCell::new(shared::wound::Wound)),
        CardName::Dazed => Rc::new(RefCell::new(shared::dazed::Dazed)),
        CardName::AscendersBane => Rc::new(RefCell::new(shared::ascenders_bane::AscendersBane)),
        CardName::Clumsy => Rc::new(RefCell::new(shared::clumsy::Clumsy)),
        CardName::CurseOfTheBell => todo!(),
        CardName::Decay => Rc::new(RefCell::new(shared::decay::Decay)),
        CardName::Doubt => Rc::new(RefCell::new(shared::doubt::Doubt)),
        CardName::Injury => Rc::new(RefCell::new(shared::injury::Injury)),
        CardName::Necronomicurse => todo!(),
        CardName::Normality => todo!(),
        CardName::Pain => todo!(),
        CardName::Parasite => todo!(),
        CardName::Regret => Rc::new(RefCell::new(shared::regret::Regret)),
        CardName::Shame => Rc::new(RefCell::new(shared::shame::Shame)),
        CardName::Writhe => todo!(),
    };
    MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
        upgraded: if upgraded {1} else {0},
    }
}
