use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{relics::Relic, utils::Number};

#[derive(Clone, Debug, Default)]
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
    pub relevant_relics: HashSet<Relic>,
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

    pub fn get_focus(&self) -> Number {
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

    pub fn get_metallicize(&self) -> Number {
        match self.intensity_buffs.get(&IntensityBuffs::Metallicize) {
            Some(amt) => *amt,
            None => Number(0),
        }
    }

    pub fn thorns(&self) -> Option<Number> {
        match self.intensity_buffs.get(&IntensityBuffs::Thorns) {
            Some(amt) => Some(*amt),
            None => None,
        }
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

    pub fn apply_debuff(&mut self, debuff: Debuff) {
        // TODO: artifact stuff
        match debuff {
            Debuff::Basic((debuff, amt)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                add_to_map(&mut self.intensity_basics, debuff, amt);
            },
            Debuff::OneTurnBool(debuff) => {
                self.one_turn_bool_debuffs.insert(debuff);
            },
            Debuff::PermanentBool(debuff) => {
                self.permanent_bool_debuffs.insert(debuff);
            },
            Debuff::Intensity((debuff, amt)) => {
                // TODO: Maybe this also needs to be a match? Unsure
                add_to_map(&mut self.intensity_debuffs, debuff, amt);
            },
            Debuff::Duration((debuff, length)) => match debuff {
                DurationDebuffs::Weak => {
                    // Ginger: cannot gain weak
                    if !self.relevant_relics.contains(&Relic::Ginger) {
                        add_to_map(&mut self.duration_debuffs, DurationDebuffs::Weak, length)
                    }
                }
                DurationDebuffs::Vulnerable => {
                    add_to_map(&mut self.duration_debuffs, DurationDebuffs::Vulnerable, length)
                },
                DurationDebuffs::Frail => {
                    // Turnip: cannot gain frail
                    if !self.relevant_relics.contains(&Relic::Turnip) {
                        add_to_map(&mut self.duration_debuffs, DurationDebuffs::Frail, length)
                    }
                },
            },
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_turn(&mut self) {
        // TODO: Basic stats up/down should go here?
        self.one_turn_bool_buffs = HashSet::new();
        self.one_turn_bool_debuffs = HashSet::new();
        decrement_map(&mut self.duration_buffs);
        decrement_map(&mut self.duration_debuffs);
        match self.poison {
            Some(amt) => {
                self.poison = Some(amt - Number(1));
                if self.poison == Some(Number(0)) {
                    self.poison = None;
                }
            },
            None => (),
        }
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

#[derive(Copy, Clone, Debug)]
pub enum Buff {
    Basic((IntensityBuffOrDebuff, Number)),
    OneTurnBool(OneTurnBoolBuffs),
    PermanentBool(PermanentBoolBuffs),
    Intensity((IntensityBuffs, Number)),
    Duration((DurationBuffs, Number)),
}

#[derive(Copy, Clone, Debug)]
pub enum Debuff {
    Basic((IntensityBuffOrDebuff, Number)),
    OneTurnBool(OneTurnBoolDebuffs),
    PermanentBool(PermanentBoolDebuffs),
    Intensity((IntensityDebuffs, Number)),
    Duration((DurationDebuffs, Number)),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OneTurnBoolDebuffs {
    NoCardDraw,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PermanentBoolDebuffs {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OneTurnBoolBuffs {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PermanentBoolBuffs {
    Barricade,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DurationBuffs {
    Intangible,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DurationDebuffs {
    Weak,
    Vulnerable,
    Frail,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntensityBuffs {
    Thorns,
    Metallicize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntensityDebuffs {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntensityBuffOrDebuff {
    Strength,
    Focus,
    Dexterity,
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
