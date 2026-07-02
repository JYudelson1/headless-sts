#![allow(unused_results)]
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Serialize, Deserialize};
use crate::{relics::{Relic, Relics}, utils::Number};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Effects {
    poison: Option<Number>,
    pub one_turn_bool_buffs: HashSet<OneTurnBoolBuffs>,
    pub one_turn_bool_debuffs: HashSet<OneTurnBoolDebuffs>,
    pub permanent_bool_buffs: HashSet<PermanentBoolBuffs>,
    pub permanent_bool_debuffs: HashSet<PermanentBoolDebuffs>,
    duration_buffs: HashMap<DurationBuffs, Number>,
    duration_debuffs: HashMap<DurationDebuffs, Number>,
    intensity_buffs: HashMap<IntensityBuffs, Number>,
    intensity_debuffs: HashMap<IntensityDebuffs, Number>,
    intensity_basics: HashMap<IntensityBuffOrDebuff, Number>,
    // A lot of relics have only-in-combat effects
    // It makes sense to hand that info over to combat so we don't often need
    //  to work with the full state
}

impl Effects {
    pub fn is_vulnerable(&self) -> bool {
        match self.duration_debuffs.get(&DurationDebuffs::Vulnerable) {
            Some(amt) => amt.0 > 0,
            None => false,
        }
    }

    pub fn is_frail(&self) -> bool {
        match self.duration_debuffs.get(&DurationDebuffs::Frail) {
            Some(amt) => amt.0 > 0,
            None => false,
        }
    }

    pub fn is_weak(&self) -> bool {
        match self.duration_debuffs.get(&DurationDebuffs::Weak) {
            Some(amt) => amt.0 > 0,
            None => false,
        }
    }

    pub fn is_intangible(&self) -> bool {
        match self.duration_buffs.get(&DurationBuffs::Intangible) {
            Some(amt) => amt.0 > 0,
            None => false,
        }
    }

    pub fn get_strength(&self) -> Number {
        match self.intensity_basics.get(&IntensityBuffOrDebuff::Strength) {
            Some(amt) => *amt,
            None => Number(0),
        }
    }

    pub fn get_dexterity(&self) -> Number {
        match self.intensity_basics.get(&IntensityBuffOrDebuff::Dexterity) {
            Some(amt) => *amt,
            None => Number(0),
        }
    }

    pub fn _get_focus(&self) -> Number {
        match self.intensity_basics.get(&IntensityBuffOrDebuff::Focus) {
            Some(amt) => *amt,
            None => Number(0),
        }
    }

    pub fn get_poison(&self) -> Number {
        match self.poison {
            Some(amt) => amt,
            None => Number(0),
        }
    }

    pub fn get_metallicize(&self) -> Option<Number> {
        self.intensity_buffs.get(&IntensityBuffs::Metallicize).copied()
    }

    pub fn thorns(&self) -> Option<Number> {
        self.intensity_buffs.get(&IntensityBuffs::Thorns).copied()
    }

    pub fn firebreathing(&self) -> Option<Number> {
        self.intensity_buffs.get(&IntensityBuffs::Firebreathing).copied()
    }

    pub fn evolve(&self) -> Option<Number> {
        self.intensity_buffs.get(&IntensityBuffs::Evolve).copied()
    }

    pub fn rage(&self) -> Option<Number> {
        self.intensity_buffs.get(&IntensityBuffs::Rage).copied()
    }

    pub fn spore_cloud(&self) -> Option<Number> {
        self.intensity_buffs
            .get(&IntensityBuffs::SporeCloud)
            .copied()
    }

    fn get_ritual(&self) -> Number {
        match self.intensity_buffs.get(&IntensityBuffs::Ritual) {
            Some(amt) => *amt,
            None => Number(0),
        }
    }

    pub fn get_feel_no_pain(&self) -> Option<Number> {
        self.intensity_buffs
            .get(&IntensityBuffs::FeelNoPain)
            .copied()
    }

    pub fn get_dark_embrace(&self) -> Option<Number> {
        self.intensity_buffs
            .get(&IntensityBuffs::DarkEmbrace)
            .copied()
    }

    pub fn trigger_curl_up(&mut self) -> Option<Number> {
        self.intensity_buffs.remove(&IntensityBuffs::CurlUp)
    }

    pub fn get_and_reduce_regenerate(&mut self) -> Option<Number> {
        if let Some(regenerate) = self.intensity_buffs.get_mut(&IntensityBuffs::Regenerate) {
            if regenerate.0 > 0 {
                *regenerate -= Number(1);
                return Some(*regenerate);
            }
        }
        None
    }

    pub fn apply_buff(&mut self, buff: Buff) {
        match buff {
            Buff::Basic((buff, amt)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                add_to_map(&mut self.intensity_basics, buff, amt);
            }
            Buff::OneTurnBool(buff) => {
                // TODO: Maybe this also needs to be a match? Unsure
                self.one_turn_bool_buffs.insert(buff);
            }
            Buff::PermanentBool(buff) => {
                self.permanent_bool_buffs.insert(buff);
            }
            Buff::Intensity((buff, amt)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                add_to_map(&mut self.intensity_buffs, buff, amt);
            }
            Buff::Duration((buff, length)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                add_to_map(&mut self.duration_buffs, buff, length);
            }
        }
    }

    fn use_artifact(&mut self) -> bool {
        let mut out = false;
        if let Some(artifact) = self.intensity_buffs.get_mut(&IntensityBuffs::Artifact) {
            if artifact.0 > 0 {
                *artifact -= Number(1);
                out = true;
            }
        }

        if out {
            remove_zeros(&mut self.intensity_buffs);
        }
        out
    }

    pub fn apply_debuff(&mut self, debuff: Debuff, relics: &Relics) {
        match debuff {
            Debuff::Basic((debuff, amt)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                // TODO: Maybe assert that amt is actually negative?
                if !self.use_artifact() {
                    add_to_map(&mut self.intensity_basics, debuff, amt);
                }
            },
            Debuff::OneTurnBool(debuff) => {
                if !self.use_artifact() {
                    self.one_turn_bool_debuffs.insert(debuff);
                }
            },
            Debuff::PermanentBool(debuff) => {
                if !self.use_artifact() {
                    self.permanent_bool_debuffs.insert(debuff);
                }
            },
            Debuff::Intensity((debuff, amt)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                if !self.use_artifact() {
                    add_to_map(&mut self.intensity_debuffs, debuff, amt);
                }
            },
            Debuff::Duration((debuff, length)) => match debuff {
                DurationDebuffs::Weak => {
                    // Ginger: cannot gain weak
                    if !relics.contains(Relic::Ginger) {
                        if !self.use_artifact() {
                            add_to_map(&mut self.duration_debuffs, DurationDebuffs::Weak, length)
                        }
                    }
                }
                DurationDebuffs::Vulnerable => {
                    if !self.use_artifact() {
                        add_to_map(&mut self.duration_debuffs, DurationDebuffs::Vulnerable, length)
                    }
                },
                DurationDebuffs::Frail => {
                    // Turnip: cannot gain frail
                    if !relics.contains(Relic::Turnip) {
                        if !self.use_artifact() {
                            add_to_map(&mut self.duration_debuffs, DurationDebuffs::Frail, length)
                        }
                    }
                },
            },
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_turn(&mut self) {
        // Lose one turn effects
        self.one_turn_bool_buffs = HashSet::new();
        self.one_turn_bool_debuffs = HashSet::new();

        // Duration effects tick down
        decrement_map(&mut self.duration_buffs);
        decrement_map(&mut self.duration_debuffs);

        // Poison ticks down (damage is done somewhere else)
        match self.poison {
            Some(amt) => {
                self.poison = Some(amt - Number(1));
                if self.poison == Some(Number(0)) {
                    self.poison = None;
                }
            },
            None => (),
        }

        // Apply ritual
        self.apply_buff(Buff::Basic((
            IntensityBuffOrDebuff::Strength,
            self.get_ritual(),
        )));

        if let Some(shackled) = self.intensity_debuffs.remove(&IntensityDebuffs::Shackled) {
            self.apply_buff(Buff::Basic((
                IntensityBuffOrDebuff::Strength,
                shackled,
            )));
        }
        if let Some(strength_down) = self.intensity_debuffs.remove(&IntensityDebuffs::StrengthDown) {
            self.apply_buff(Buff::Basic((
                IntensityBuffOrDebuff::Strength,
                strength_down,
            )));
        }
        self.clear_intensity_debuffs();

        // Lose all Rage,
        self.intensity_buffs.remove(&IntensityBuffs::Rage);

        // Remove zero buffs (affects regenerate)
        remove_zeros(&mut self.intensity_buffs);
    }

    fn clear_intensity_debuffs(&mut self) {
        let mut new_debuffs = HashMap::new();
        if self.intensity_debuffs.contains_key(&IntensityDebuffs::Slow) {
            new_debuffs.insert(IntensityDebuffs::Slow, Number(0));
        }
        self.intensity_debuffs = new_debuffs;
    }

    pub fn cleanse_debuffs(&mut self) {
        self.duration_debuffs = HashMap::new();
        self.intensity_debuffs = HashMap::new();
        self.one_turn_bool_debuffs = HashSet::new();
        self.permanent_bool_debuffs = HashSet::new();
        for val in self.intensity_basics.values_mut() {
            if *val < Number(0) {
                *val = Number(0)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Buff {
    Basic((IntensityBuffOrDebuff, Number)),
    OneTurnBool(OneTurnBoolBuffs),
    PermanentBool(PermanentBoolBuffs),
    Intensity((IntensityBuffs, Number)),
    Duration((DurationBuffs, Number)),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Debuff {
    Basic((IntensityBuffOrDebuff, Number)),
    OneTurnBool(OneTurnBoolDebuffs),
    PermanentBool(PermanentBoolDebuffs),
    Intensity((IntensityDebuffs, Number)),
    Duration((DurationDebuffs, Number)),
}

// TODO: Add all the unique debuffs from https://slay-the-spire.fandom.com/wiki/Debuff#Debuffs

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OneTurnBoolDebuffs {
    NoCardDraw, // Status: Implemented
    Entangled, // Status: Implemented
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermanentBoolDebuffs {
    Confused // Status: Not Implemented (TODO)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OneTurnBoolBuffs {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermanentBoolBuffs {
    Barricade, // Status: Implemented
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationBuffs {
    Intangible, // Status: Implemented
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationDebuffs {
    Weak, // Status: Implemented
    Vulnerable, // Status: Implemented
    Frail, // Status: Implemented
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntensityBuffs {
    Thorns, // Status: Implemented
    Metallicize, // Status: Implemented
    Ritual, // Status: Implemented
    Firebreathing, // Status: Implemented
    Evolve, // Status: Implemented
    Artifact, // Status: Implemented
    FeelNoPain, // Status: Not Implemented (TODO)
    DarkEmbrace, // Status: Not Implemented (TODO)
    CurlUp, // Status: Not Implemented (TODO)
    Rage, // Status: Implemented
    SporeCloud, // Status: Not Implemented (TODO)
    Combust, // Status: Not Implemented (TODO)
    Regenerate, // Status: Implemented
    Buffer, // Status: Not Implemented (TODO)
    DrawNextTurn // Status: Not Implemented (TODO)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntensityDebuffs {
    Shackled, // Status: Implemented
    StrengthDown, // Status: Implemented
    Slow // Status: Not Implemented (TODO)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntensityBuffOrDebuff {
    Strength, // Status: Implemented
    Focus, // Status: Not Implemented (TODO)
    Dexterity, // Status: Implemented
}

fn remove_zeros<T: Hash + Eq + PartialEq + Clone>(map: &mut HashMap<T, Number>) {
    let mut to_remove = vec![];
    for (key, value) in map.iter() {
        if *value == Number(0) {
            to_remove.push(key.clone());
        }
    }
    for item in to_remove {
        map.remove(&item);
    }
}

fn decrement_map<T: Hash + Eq + PartialEq + Clone>(map: &mut HashMap<T, Number>) {
    for (_, value) in map.iter_mut() {
        *value -= Number(1);
    }
    remove_zeros(map);
}

fn add_to_map<T: Hash + Eq + PartialEq>(map: &mut HashMap<T, Number>, key: T, amt: Number) {
    let current = map.get(&key);
    let amt = amt.add_option(current.copied()).unwrap();
    map.insert(key, amt);
}
