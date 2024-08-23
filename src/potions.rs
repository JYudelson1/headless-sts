use crate::{combat::Combat, effects::{Buff, IntensityBuffOrDebuff}, relics::Relic, state::State, utils::Number};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Potion {
    BlockPotion,
    StrengthPotion,
    // TODO: add the rest
}

#[derive(Debug)]
pub struct PotionBag {
    capacity: usize,
    potions: Vec<Potion>,
}

impl PotionBag {
    pub fn new(ascension: u8) -> Self {
        Self {
            capacity: if ascension >= 11 { 2 } else { 3 },
            potions: vec![],
        }
    }

    pub fn add(&mut self, potion: Potion) {
        if self.potions.len() < self.capacity {
            self.potions.push(potion);
        }
    }

    pub fn increase_size(&mut self, added_slots: usize) {
        self.capacity += added_slots;
    }

    pub fn len(&self) -> usize {
        self.potions.len()
    }

    pub fn remove_potion(&mut self, index: usize) -> Potion {
        self.potions.remove(index)
    }
}

impl Combat {
    pub fn use_combat_potion(&mut self, potion: Potion) {
        match potion {
            Potion::BlockPotion => self.self_block += Number(12),
            Potion::StrengthPotion => self.self_effects.apply_buff(Buff::Basic((IntensityBuffOrDebuff::Strength, Number(2)))),
        }
    }
}

impl State {
    fn use_non_combat_potion(&mut self, potion: Potion) -> bool {
        match potion {
            _ => false,
        }
    }

    pub fn use_potion(&mut self, index: usize) {
        let potion = self.potions.remove_potion(index);

        let potion_used = self.use_non_combat_potion(potion);

        if !potion_used {
            self.get_combat().use_combat_potion(potion)
        }

        if self.relics.contains(Relic::ToyOrnithopter) {
            self.heal(5);
        }
    }

    pub fn discard_potion(&mut self, index: usize) {
        self.potions.remove_potion(index);
    }
}
