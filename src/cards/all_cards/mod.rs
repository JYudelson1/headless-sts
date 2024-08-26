mod defect;
mod ironclad;
mod shared;
mod silent;
mod watcher;

use std::{cell::RefCell, rc::Rc};

use crate::utils::NotImplemented;

use super::{
    card_trait::{Card, MasterCard},
    CardName,
};

pub fn make_card(name: CardName, upgraded: bool) -> Result<MasterCard, NotImplemented> {
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
        CardName::Clash => Rc::new(RefCell::new(ironclad::clash::Clash(upgraded))),
        CardName::Flex => Err(NotImplemented::Card(name))?,
        CardName::Havoc => Rc::new(RefCell::new(ironclad::havoc::Havoc(upgraded))),
        CardName::Headbutt => Err(NotImplemented::Card(name))?,
        CardName::HeavyBlade => Rc::new(RefCell::new(ironclad::heavyblade::HeavyBlade(upgraded))),
        CardName::IronWave => Rc::new(RefCell::new(ironclad::ironwave::IronWave(upgraded))),
        CardName::PerfectedStrike => Rc::new(RefCell::new(ironclad::perfectedstrike::PerfectedStrike(upgraded))),
        CardName::PommelStrike => Rc::new(RefCell::new(ironclad::pommelstrike::PommelStrike(upgraded))),
        CardName::SwordBoomerang => Rc::new(RefCell::new(ironclad::swordboomerang::SwordBoomerang(upgraded))),
        CardName::ThunderClap => Rc::new(RefCell::new(ironclad::thunderclap::ThunderClap(upgraded))),
        CardName::TrueGrit => Rc::new(RefCell::new(ironclad::truegrit::TrueGrit(upgraded))),
        CardName::TwinStrike => Rc::new(RefCell::new(ironclad::twinstrike::TwinStrike(upgraded))),
        CardName::WarCry => Rc::new(RefCell::new(ironclad::warcry::WarCry(upgraded))),
        CardName::WildStrike => Rc::new(RefCell::new(ironclad::wildstrike::WildStrike(upgraded))),
        CardName::BattleTrance => Rc::new(RefCell::new(ironclad::battletrance::BattleTrance(upgraded))),
        CardName::BloodForBlood => Err(NotImplemented::Card(name))?,
        CardName::BurningPact => Rc::new(RefCell::new(ironclad::burningpact::BurningPact(upgraded))),
        CardName::Combust => Err(NotImplemented::Card(name))?,
        CardName::DarkEmbrace => Rc::new(RefCell::new(ironclad::darkembrace::DarkEmbrace(upgraded))),
        CardName::Disarm => Rc::new(RefCell::new(ironclad::disarm::Disarm(upgraded))),
        CardName::Dropkick => Err(NotImplemented::Card(name))?,
        CardName::DualWield => Err(NotImplemented::Card(name))?,
        CardName::Entrench => Rc::new(RefCell::new(ironclad::entrench::Entrench(upgraded))),
        CardName::Evolve => Rc::new(RefCell::new(ironclad::evolve::Evolve(upgraded))),
        CardName::FeelNoPain => Rc::new(RefCell::new(ironclad::feelnopain::FeelNoPain(upgraded))),
        CardName::FireBreathing => Err(NotImplemented::Card(name))?,
        CardName::FlameBarrier => Err(NotImplemented::Card(name))?,
        CardName::HemoKinesis => Rc::new(RefCell::new(ironclad::hemokinesis::HemoKinesis(upgraded))),
        CardName::InfernalBlade => Err(NotImplemented::Card(name))?,
        CardName::Inflame => Rc::new(RefCell::new(ironclad::inflame::Inflame(upgraded))),
        CardName::Intimidate => Rc::new(RefCell::new(ironclad::intimidate::Intimidate(upgraded))),
        CardName::Metallicize =>  Rc::new(RefCell::new(ironclad::metallicize::Metallicize(upgraded))),
        CardName::PowerThrough => Rc::new(RefCell::new(ironclad::powerthrough::PowerThrough(upgraded))),
        CardName::Pummel => Rc::new(RefCell::new(ironclad::pummel::Pummel(upgraded))),
        CardName::Rage => Rc::new(RefCell::new(ironclad::rage::Rage(upgraded))),
        CardName::Rampage => Rc::new(RefCell::new(ironclad::rampage::Rampage::new(upgraded))),
        CardName::RecklessCharge => Rc::new(RefCell::new(ironclad::recklesscharge::RecklessCharge(upgraded))),
        CardName::Rupture => Err(NotImplemented::Card(name))?,
        CardName::SearingBlow => Rc::new(RefCell::new(ironclad::searingblow::SearingBlow(upgraded as u16))),
        CardName::SecondWind => Err(NotImplemented::Card(name))?,
        CardName::SeeingRed => Rc::new(RefCell::new(ironclad::seeingred::SeeingRed(upgraded))),
        CardName::Sentinel => Err(NotImplemented::Card(name))?,
        CardName::SeverSoul => Err(NotImplemented::Card(name))?,
        CardName::Shockwave => Rc::new(RefCell::new(ironclad::shockwave::Shockwave(upgraded))),
        CardName::SpotWeakness => Err(NotImplemented::Card(name))?,
        CardName::Uppercut => Rc::new(RefCell::new(ironclad::uppercut::Uppercut(upgraded))),
        CardName::Whirlwind => Err(NotImplemented::Card(name))?,
        CardName::Berserk => Rc::new(RefCell::new(ironclad::berserk::Berserk(upgraded))),
        CardName::Brutality => Err(NotImplemented::Card(name))?,
        CardName::Corruption => Err(NotImplemented::Card(name))?,
        CardName::DemonForm => Rc::new(RefCell::new(ironclad::demonform::DemonForm(upgraded))),
        CardName::DoubleTap => Err(NotImplemented::Card(name))?,
        CardName::Exhume => Err(NotImplemented::Card(name))?,
        CardName::Feed => Err(NotImplemented::Card(name))?,
        CardName::FiendFire => Err(NotImplemented::Card(name))?,
        CardName::Juggernaut => Err(NotImplemented::Card(name))?,
        CardName::LimitBreak => Err(NotImplemented::Card(name))?,
        CardName::Offering => Rc::new(RefCell::new(ironclad::offering::Offering(upgraded))),
        CardName::Reaper => Err(NotImplemented::Card(name))?,
        CardName::Slimed => Rc::new(RefCell::new(shared::slimed::Slimed(upgraded))),
        CardName::Burn => Rc::new(RefCell::new(shared::burn::Burn(upgraded))),
        CardName::Wound => Rc::new(RefCell::new(shared::wound::Wound)),
        CardName::Dazed => Rc::new(RefCell::new(shared::dazed::Dazed)),
        CardName::AscendersBane => Rc::new(RefCell::new(shared::ascenders_bane::AscendersBane)),
        CardName::Clumsy => Rc::new(RefCell::new(shared::clumsy::Clumsy)),
        CardName::CurseOfTheBell => Err(NotImplemented::Card(name))?,
        CardName::Decay => Rc::new(RefCell::new(shared::decay::Decay)),
        CardName::Doubt => Rc::new(RefCell::new(shared::doubt::Doubt)),
        CardName::Injury => Rc::new(RefCell::new(shared::injury::Injury)),
        CardName::Necronomicurse => Err(NotImplemented::Card(name))?,
        CardName::Normality => Err(NotImplemented::Card(name))?,
        CardName::Pain => Err(NotImplemented::Card(name))?,
        CardName::Parasite => Err(NotImplemented::Card(name))?,
        CardName::Regret => Rc::new(RefCell::new(shared::regret::Regret)),
        CardName::Shame => Rc::new(RefCell::new(shared::shame::Shame)),
        CardName::Writhe => Err(NotImplemented::Card(name))?,
        CardName::Apparition => Rc::new(RefCell::new(shared::apparition::Apparition(upgraded))),
        CardName::BandageUp => Err(NotImplemented::Card(name))?,
        CardName::Blind => Err(NotImplemented::Card(name))?,
        CardName::DarkShackles => Err(NotImplemented::Card(name))?,
        CardName::DeepBreath => Err(NotImplemented::Card(name))?,
        CardName::Discovery => Err(NotImplemented::Card(name))?,
        CardName::DramaticEntrance => Err(NotImplemented::Card(name))?,
        CardName::Enlightenment => Err(NotImplemented::Card(name))?,
        CardName::Finesse => Err(NotImplemented::Card(name))?,
        CardName::FlashOfSteel => Err(NotImplemented::Card(name))?,
        CardName::Forethought => Err(NotImplemented::Card(name))?,
        CardName::GoodInstincts => Err(NotImplemented::Card(name))?,
        CardName::Impatience => Err(NotImplemented::Card(name))?,
        CardName::JackOfAllTrades => Err(NotImplemented::Card(name))?,
        CardName::Madness => Err(NotImplemented::Card(name))?,
        CardName::MindBlast => Err(NotImplemented::Card(name))?,
        CardName::Panacea => Err(NotImplemented::Card(name))?,
        CardName::PanicButton => Err(NotImplemented::Card(name))?,
        CardName::Purity => Err(NotImplemented::Card(name))?,
        CardName::SwiftStrike => Err(NotImplemented::Card(name))?,
        CardName::Trip => Err(NotImplemented::Card(name))?,
        CardName::Apotheosis => Err(NotImplemented::Card(name))?,
        CardName::Chrysalis => Err(NotImplemented::Card(name))?,
        CardName::HandOfGreed => Err(NotImplemented::Card(name))?,
        CardName::Magnetism => Err(NotImplemented::Card(name))?,
        CardName::MasterOfStrategy => Err(NotImplemented::Card(name))?,
        CardName::Mayhem => Err(NotImplemented::Card(name))?,
        CardName::Metamorphosis => Err(NotImplemented::Card(name))?,
        CardName::Panache => Err(NotImplemented::Card(name))?,
        CardName::SadisticNature => Err(NotImplemented::Card(name))?,
        CardName::SecretTechnique => Err(NotImplemented::Card(name))?,
        CardName::SecretWeapon => Err(NotImplemented::Card(name))?,
        CardName::TheBomb => Err(NotImplemented::Card(name))?,
        CardName::ThinkingAhead => Err(NotImplemented::Card(name))?,
        CardName::Transmutation => Err(NotImplemented::Card(name))?,
        CardName::Violence => Err(NotImplemented::Card(name))?,
    };
    Ok(MasterCard {
        card,
        id: uuid::Uuid::new_v4(),
        upgraded: if upgraded {1} else {0},
    })
}
