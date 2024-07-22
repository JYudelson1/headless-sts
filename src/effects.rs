use crate::utils::Number;

#[derive(Clone)]
pub struct Effects {
    strength: Option<Number>,
    poison: Option<Number>,
    vulnerable: Option<Number>,
    // TODO: Other effects
    // TODO: Other buffs??
}

impl Effects {
    pub fn is_vulnerable(&self) -> bool {
        self.vulnerable.is_some()
    }

    pub fn apply_buff(&mut self, buff: Buff) {
        match buff {
            Buff::Strength(amt) => {
                self.strength = amt.add_option(self.strength);
            }
        }
    }

    pub fn new() -> Self {
        Self {
            strength: None,
            poison: None,
            vulnerable: None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Buff {
    Strength(Number),
}
