// TODO: Figure out whether to have different blessings if you die early

use crate::{
    relics::Relic,
    state::State,
    utils::{number_between, Act, Character, Number},
};

use super::VisibleStates;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FirstBlessing {
    Remove,
    Transform,
    Upgrade,
    ChooseClassCard,
    ChooseUncommonColorless,
    RandomRare,
}

impl FirstBlessing {
    pub fn random() -> Self {
        let x = number_between(0, 5);
        let variants = [
            Self::Remove,
            Self::Transform,
            Self::Upgrade,
            Self::ChooseClassCard,
            Self::ChooseUncommonColorless,
            Self::RandomRare,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SecondBlessing {
    MaxHP,
    NeowsLament,
    RandomCommonRelic,
    Gold100,
    Random3Potions,
}

impl SecondBlessing {
    pub fn random() -> Self {
        let x = number_between(0, 4);
        let variants = [
            Self::MaxHP,
            Self::NeowsLament,
            Self::RandomCommonRelic,
            Self::Gold100,
            Self::Random3Potions,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThirdUpside {
    Remove2,
    Transform2,
    Gold250,
    ChooseRareClassCard,
    ChooseRareColorless,
    BigMaxHP,
}

impl ThirdUpside {
    pub fn random() -> Self {
        let x = number_between(0, 5);
        let variants = [
            Self::Remove2,
            Self::Transform2,
            Self::Gold250,
            Self::ChooseRareClassCard,
            Self::ChooseRareColorless,
            Self::BigMaxHP,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThirdDownside {
    LoseMaxHealth,
    RandomCurse,
    TakeDamage,
    LoseAllGold,
}

impl ThirdDownside {
    pub fn random() -> Self {
        let x = number_between(0, 3);
        let variants = [
            Self::LoseMaxHealth,
            Self::RandomCurse,
            Self::TakeDamage,
            Self::LoseAllGold,
        ];
        variants[x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThirdBlessing {
    pub upside: ThirdUpside,
    pub downside: ThirdDownside,
}

impl ThirdBlessing {
    pub fn random() -> Self {
        loop {
            let upside = ThirdUpside::random();
            let downside = ThirdDownside::random();

            // Forbidden combos
            if (upside == ThirdUpside::BigMaxHP && downside == ThirdDownside::LoseMaxHealth)
                || (upside == ThirdUpside::Gold250 && downside == ThirdDownside::LoseAllGold)
                || (upside == ThirdUpside::Remove2 && downside == ThirdDownside::RandomCurse)
            {
                continue;
            } else {
                return Self { upside, downside };
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NeowsBlessing {
    First(FirstBlessing),
    Second(SecondBlessing),
    Third(ThirdBlessing),
    RelicSwap,
}

pub fn get_neow_blessings() -> [NeowsBlessing; 4]{
    let first = NeowsBlessing::First(FirstBlessing::random());
    let second = NeowsBlessing::Second(SecondBlessing::random());
    let third = NeowsBlessing::Third(ThirdBlessing::random());

    [first, second, third, NeowsBlessing::RelicSwap]
}

impl State {
    pub fn _apply_neow_blessing(&mut self, blessing: NeowsBlessing) {
        match blessing {
            NeowsBlessing::First(bless) => match bless {
                crate::screens::FirstBlessing::Remove => {
                    self.visible_screen = VisibleStates::RemoveCardScreen;
                }
                crate::screens::FirstBlessing::Transform => todo!(),
                crate::screens::FirstBlessing::Upgrade => {
                    self.visible_screen = VisibleStates::UpgradeCardScreen;
                }
                crate::screens::FirstBlessing::ChooseClassCard => {
                    let card_reward = self.card_rng.get_noncombat_choice(3, Act::Act1);
                    self.visible_screen = VisibleStates::CardReward(card_reward);
                },
                crate::screens::FirstBlessing::ChooseUncommonColorless => todo!(),
                crate::screens::FirstBlessing::RandomRare => todo!(),
            },
            NeowsBlessing::Second(bless) => match bless {
                crate::screens::SecondBlessing::MaxHP => {
                    let amt = match self.character {
                        Character::Ironclad => 8,
                        Character::Silent => 6,
                        Character::Defect => 7,
                        Character::Watcher => 7,
                    };
                    self.max_health += Number(amt);
                }
                crate::screens::SecondBlessing::NeowsLament => {
                    self.relics.add(Relic::NeowsLament(3))
                }
                crate::screens::SecondBlessing::RandomCommonRelic => {
                    let relic = self.relics.random_common();
                    self.relics.add(relic)
                }
                crate::screens::SecondBlessing::Gold100 => self.gold += 100,
                crate::screens::SecondBlessing::Random3Potions => todo!(),
            },
            NeowsBlessing::Third(bless) => {
                match bless.upside {
                    crate::screens::ThirdUpside::Remove2 => todo!(),
                    crate::screens::ThirdUpside::Transform2 => todo!(),
                    crate::screens::ThirdUpside::Gold250 => self.gold += 250,
                    crate::screens::ThirdUpside::ChooseRareClassCard => todo!(),
                    crate::screens::ThirdUpside::ChooseRareColorless => todo!(),
                    crate::screens::ThirdUpside::BigMaxHP => {
                        let amt = match self.character {
                            Character::Ironclad => 16,
                            Character::Silent => 12,
                            Character::Defect => 14,
                            Character::Watcher => 14,
                        };
                        self.max_health += Number(amt);
                    },
                }

                match bless.downside {
                    crate::screens::ThirdDownside::LoseMaxHealth => {
                        let amt = match self.character {
                            Character::Ironclad => 8,
                            Character::Silent => 7,
                            Character::Defect => 7,
                            Character::Watcher => 7,
                        };
                        self.max_health -= Number(amt);
                    },
                    crate::screens::ThirdDownside::RandomCurse => todo!(),
                    crate::screens::ThirdDownside::TakeDamage => {
                        let amt = ((self.current_health as f32 / 10.0).floor() * 3.0).floor() as u16;
                        self.current_health -= amt;
                    },
                    crate::screens::ThirdDownside::LoseAllGold => self.gold = 0,
                }
            }
            NeowsBlessing::RelicSwap => {
                let relic = self.relics.random_boss();
                self.relics.remove(0);
                self.relics.add(relic);
            }
        }
    }
}
