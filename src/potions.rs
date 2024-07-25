pub enum Potion {
    BlockPotion,
    StrengthPotion,
    // TODO: add the rest
}

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
}
