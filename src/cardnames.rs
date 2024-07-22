pub enum CardName {
    Strike,
    Defend,
    Bash,
}

impl CardName {
    pub fn targets(&self) -> bool {
        match self {
            CardName::Strike => true,
            CardName::Defend => false,
            CardName::Bash => true,
        }
    }

    pub fn random_common() -> Self {
        todo!()
    }

    pub fn random_uncommon() -> Self {
        todo!()
    }

    pub fn random_rare() -> Self {
        todo!()
    }
}
