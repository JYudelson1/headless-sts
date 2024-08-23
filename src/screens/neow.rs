use rand::prelude::SliceRandom;

use crate::{
    cardrewardrng::CombatType,
    cards::{make_card, CardName},
    relics::Relic,
    state::State,
    utils::{number_between, Act, Character, NotImplemented, Number},
};

use super::{CardReward, VisibleStates};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
    pub fn _apply_neow_blessing(&mut self, blessing: NeowsBlessing) -> Result<(), NotImplemented> {
        match blessing {
            NeowsBlessing::First(bless) => match bless {
                FirstBlessing::Remove => {
                    self.visible_screen = VisibleStates::RemoveCardScreen(1);
                }
                FirstBlessing::Transform => {
                    self.visible_screen = VisibleStates::TransformCardScreen(1);
                }
                FirstBlessing::Upgrade => {
                    self.visible_screen = VisibleStates::UpgradeCardScreen(1);
                }
                FirstBlessing::ChooseClassCard => {
                    let card_reward = self.card_rng.get_noncombat_card_choice(3, self.character);
                    self.visible_screen = VisibleStates::CardReward(card_reward);
                },
                FirstBlessing::ChooseUncommonColorless => {
                    let rewards = CardName::colorless_uncommons().choose_multiple(&mut rand::thread_rng(), 3).map(|name| CardReward { card: *name, is_upgraded: false }).collect();
                    self.visible_screen = VisibleStates::CardReward(rewards);
                },
                FirstBlessing::RandomRare => {
                    let rare = self.card_rng.get_rewards(1, CombatType::Boss, &Act::Act3, self.character)[0].card;
                    self.main_deck.push(make_card(rare, false)?);
                },
            },
            NeowsBlessing::Second(bless) => match bless {
                SecondBlessing::MaxHP => {
                    let amt = match self.character {
                        Character::Ironclad => 8,
                        Character::Silent => 6,
                        Character::Defect => 7,
                        Character::Watcher => 7,
                    };
                    self.increase_max_hp(amt);
                }
                SecondBlessing::NeowsLament => {
                    self.relics.add(Relic::NeowsLament(3))
                }
                SecondBlessing::RandomCommonRelic => {
                    let relic = self.relics.random_common();
                    self.relics.add(relic)
                }
                SecondBlessing::Gold100 => self.gold += 100,
                SecondBlessing::Random3Potions => Err(NotImplemented::Neow(blessing))?,
            },
            NeowsBlessing::Third(bless) => {
                match bless.upside {
                    ThirdUpside::Remove2 => {
                        self.visible_screen = VisibleStates::RemoveCardScreen(2)
                    }
                    ThirdUpside::Transform2 => {
                        self.visible_screen = VisibleStates::TransformCardScreen(2)
                    }
                    ThirdUpside::Gold250 => self.gold += 250,
                    ThirdUpside::ChooseRareClassCard => {
                        let rares = self.card_rng.get_rewards(3, CombatType::Boss, &Act::Act3, self.character)[0].card;
                        self.main_deck.push(make_card(rares, false)?);
                    },
                    ThirdUpside::ChooseRareColorless => {
                        let rewards = CardName::colorless_rares().choose_multiple(&mut rand::thread_rng(), 3).map(|name| CardReward { card: *name, is_upgraded: false }).collect();
                        self.visible_screen = VisibleStates::CardReward(rewards);
                    },
                    ThirdUpside::BigMaxHP => {
                        let amt = match self.character {
                            Character::Ironclad => 16,
                            Character::Silent => 12,
                            Character::Defect => 14,
                            Character::Watcher => 14,
                        };
                        self.increase_max_hp(amt);
                    },
                }

                match bless.downside {
                    ThirdDownside::LoseMaxHealth => {
                        let amt = match self.character {
                            Character::Ironclad => 8,
                            Character::Silent => 7,
                            Character::Defect => 7,
                            Character::Watcher => 7,
                        };
                        self.max_health -= Number(amt);
                        if self.current_health > self.max_health.0 as u16 {
                            self. current_health = self.max_health.0 as u16;
                        }
                    },
                    ThirdDownside::RandomCurse => {
                        let curses = CardName::transform_curses();
                        let curse = curses.choose(&mut rand::thread_rng());
                        let card = make_card(*curse.unwrap(), false)?;
                        self.add_to_deck(card);
                    }
                    ThirdDownside::TakeDamage => {
                        let amt = ((self.current_health as f32 / 10.0).floor() * 3.0).floor() as u16;
                        self.current_health -= amt;
                    },
                    ThirdDownside::LoseAllGold => self.gold = 0,
                }
            }
            NeowsBlessing::RelicSwap => {
                let relic = self.relics.random_boss();
                self.relics.remove(0);
                self.relics.add(relic);
            }
        }

        Ok(())
    }
}
