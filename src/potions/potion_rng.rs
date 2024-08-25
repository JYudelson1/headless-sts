use rand::random;

#[derive(Debug)]
pub struct PotionRng(f32);

impl PotionRng {
    pub fn new() -> Self {
        Self(0.4)
    }

    pub fn reset(&mut self) {
        self.0 = 0.4;
    }

    pub fn maybe_get_potion(&mut self) -> bool {
        let x = random::<f32>();

        if x < self.0 {
            self.reset();
            return true;
        } else {
            self.0 += 0.1;
            return false;
        }
    }
}
