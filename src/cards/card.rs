#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardName {
    Strike,
    Defend,
    Bash,
    Void,
    ShrugItOff,
}

impl CardName {
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

pub enum CardType {
    Attack,
    Power,
    Skill,
    Status,
    Curse,
}
