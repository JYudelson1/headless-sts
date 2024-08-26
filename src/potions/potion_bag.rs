use super::Potion;

#[derive(Debug, Clone)]
pub struct PotionBag {
    pub capacity: usize,
    pub potions: Vec<Potion>,
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

    pub fn remove_potion(&mut self, index: usize) -> Potion {
        self.potions.remove(index)
    }
}
