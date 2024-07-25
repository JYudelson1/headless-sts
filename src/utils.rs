use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy)]
pub struct Number(pub i16);

impl Number {
    pub fn add_option(&self, rhs: Option<Self>) -> Option<Self> {
        match rhs {
            Some(orig) => Some(orig + *self),
            None => Some(*self),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let total = self.0 + rhs.0;
        if total >= 999 {
            Self(999)
        } else if total <= -999 {
            Self(-999)
        } else {
            Self(total)
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        let total = self.0 - rhs.0;
        if total >= 999 {
            Self(999)
        } else if total <= -999 {
            Self(-999)
        } else {
            Self(total)
        }
    }
}

pub enum Act {
    Act1,
    Act2,
    Act3,
}

#[derive(Clone, Copy)]
pub enum Character {
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    Ruby,
    Emerald,
    Sapphire,
}

pub struct Keys(pub HashSet<Key>);

impl Keys {
    pub fn has_all(&self) -> bool {
        self.0.len() == 3
    }

    pub fn add_key(&mut self, key: Key) {
        self.0.insert(key);
    }

    pub fn has_key(&self, key: &Key) -> bool {
        self.0.contains(key)
    }

    pub fn new() -> Self {
        Self(HashSet::new())
    }
}
