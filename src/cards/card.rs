use rand::seq::SliceRandom;

use crate::utils::Character;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardName {
    Strike,
    Defend,
    Bash,
    Void,
    ShrugItOff,
}

impl CardName {
    pub fn random_common(character: Character) -> Self {
        match character {
            Character::Ironclad => Self::random_from(Self::ironclad_commons()),
            Character::Silent => Self::random_from(Self::silent_commons()),
            Character::Defect => Self::random_from(Self::defect_commons()),
            Character::Watcher => Self::random_from(Self::watcher_commons()),
        }
    }

    pub fn random_uncommon(character: Character) -> Self {
        match character {
            Character::Ironclad => Self::random_from(Self::ironclad_uncommons()),
            Character::Silent => Self::random_from(Self::silent_uncommons()),
            Character::Defect => Self::random_from(Self::defect_uncommons()),
            Character::Watcher => Self::random_from(Self::watcher_uncommons()),
        }
    }

    pub fn random_rare(character: Character) -> Self {
        match character {
            Character::Ironclad => Self::random_from(Self::ironclad_rares()),
            Character::Silent => Self::random_from(Self::silent_rares()),
            Character::Defect => Self::random_from(Self::defect_rares()),
            Character::Watcher => Self::random_from(Self::watcher_rares()),
        }
    }

    fn random_from(pool: Vec<Self>) -> Self {
        *pool.choose(&mut rand::thread_rng()).unwrap()
    }

    fn ironclad_commons() -> Vec<Self> {
        vec![CardName::ShrugItOff]
    }

    fn ironclad_uncommons() -> Vec<Self> {
        vec![]
    }

    fn ironclad_rares() -> Vec<Self> {
        vec![]
    }

    fn silent_commons() -> Vec<Self> {
        vec![]
    }

    fn silent_uncommons() -> Vec<Self> {
        vec![]
    }

    fn silent_rares() -> Vec<Self> {
        vec![]
    }

    fn defect_commons() -> Vec<Self> {
        vec![]
    }

    fn defect_uncommons() -> Vec<Self> {
        vec![]
    }

    fn defect_rares() -> Vec<Self> {
        vec![]
    }

    fn watcher_commons() -> Vec<Self> {
        vec![]
    }

    fn watcher_uncommons() -> Vec<Self> {
        vec![]
    }

    fn watcher_rares() -> Vec<Self> {
        vec![]
    }
}

pub enum CardType {
    Attack,
    Power,
    Skill,
    Status,
    Curse,
}
