use std::collections::HashSet;

use crate::{relics::Relic, utils::Number};

#[derive(Clone)]
pub struct Effects {
    strength: Option<Number>,
    dexterity: Option<Number>,
    poison: Option<Number>,
    vulnerable: Option<Number>,
    weak: Option<Number>,
    focus: Option<Number>,
    thorns: Option<Number>,
    frail: Option<Number>,
    intangible: Option<Number>,
    // TODO: Other effects
    // TODO: Other buffs??
    // A lot of relics have only-in-combat effects
    // It makes sense to hand that info over to combat so we don't often need
    //  to work with the full state
    pub relevant_relics: HashSet<Relic>,
}

impl Effects {
    pub fn is_vulnerable(&self) -> bool {
        self.vulnerable.is_some()
    }

    pub fn is_frail(&self) -> bool {
        self.frail.is_some()
    }

    pub fn is_weak(&self) -> bool {
        self.weak.is_some()
    }

    pub fn is_intangible(&self) -> bool {
        self.intangible.is_some()
    }

    pub fn get_strength(&self) -> Number {
        match self.strength {
            Some(amt) => amt,
            None => Number(0),
        }
    }

    pub fn get_dexterity(&self) -> Number {
        match self.dexterity {
            Some(amt) => amt,
            None => Number(0),
        }
    }

    pub fn get_focus(&self) -> Number {
        match self.focus {
            Some(amt) => amt,
            None => Number(0),
        }
    }

    pub fn thorns(&self) -> Option<Number> {
        self.thorns
    }

    pub fn apply_buff(&mut self, buff: Buff) {
        match buff {
            Buff::Strength(amt) => self.strength = amt.add_option(self.strength),
            Buff::Dexterity(amt) => self.dexterity = amt.add_option(self.dexterity),
            Buff::Focus(amt) => self.focus = amt.add_option(self.focus),
            Buff::Thorns(amt) => self.thorns = amt.add_option(self.thorns),
            Buff::Intangible(amt) => self.intangible = amt.add_option(self.intangible),
        }
    }

    pub fn apply_debuff(&mut self, debuff: Debuff) {
        // TODO: artifact stuff
        match debuff {
            Debuff::Weak(amt) => {
                // Ginger: cannot gain weak
                if !self.relevant_relics.contains(&Relic::Ginger) {
                    self.weak = amt.add_option(self.weak);
                }
            },
            Debuff::Vulnerable(amt) => self.vulnerable = amt.add_option(self.vulnerable),
            Debuff::Frail(amt) => {
                // Turnip: cannot gain frail
                if !self.relevant_relics.contains(&Relic::Turnip) {
                    self.frail = amt.add_option(self.frail);
                }
            },
        }
    }

    pub fn new() -> Self {
        Self {
            strength: None,
            poison: None,
            vulnerable: None,
            weak: None,
            frail: None,
            focus: None,
            thorns: None,
            dexterity: None,
            relevant_relics: HashSet::new(),
            intangible: None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Buff {
    Strength(Number),
    Focus(Number),
    Thorns(Number),
    Dexterity(Number),
    Intangible(Number),
}

#[derive(Copy, Clone)]
pub enum Debuff {
    Weak(Number),
    Vulnerable(Number),
    Frail(Number),
}
