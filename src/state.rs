use std::backtrace;

use crate::{
    actions::Action,
    cardrewardrng::CardRewardRng,
    cards::{make_starter_deck, MasterCard},
    map::Map,
    potions::PotionBag,
    relics::{Relic, Relics},
    screens::{NeowsBlessing, VisibleStates},
    utils::{Act, Character, Number},
};

pub struct State {
    pub act: Act,
    // TODO: map
    pub visible_screen: VisibleStates,
    pub card_rng: CardRewardRng,
    pub potions: PotionBag,
    pub map: Map,
    pub ascension: u8,
    pub max_health: Number,
    pub current_health: u16,
    pub gold: Number,
    pub current_floor: u8,
    pub character: Character,
    pub relics: Relics,
    pub main_deck: Vec<MasterCard>,
}

impl State {
    pub fn new(character: Character, ascension: u8) -> Self {
        Self {
            act: Act::Act1,
            visible_screen: VisibleStates::new(),
            card_rng: CardRewardRng::new(),
            potions: PotionBag::new(ascension),
            map: Map::new(Act::Act1, ascension),
            ascension,
            //TODO: non-ironclad
            max_health: Number(80),
            current_health: if ascension >= 6 { 72 } else { 80 },
            gold: Number(99),
            current_floor: 0,
            character,
            relics: Relics::new(character),
            main_deck: make_starter_deck(character)
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        assert!(self.visible_screen.get_actions().contains(&action));

        match action {
            Action::PlayUntargetedCard(_) => todo!(),
            Action::PlayTargetedCard(_) => todo!(),
            Action::CollectReward(_) => todo!(),
            Action::MakeCardChoice(_) => todo!(),
            Action::EndTurn => todo!(),
            Action::TraverseMap(node) => todo!(),
            Action::MakeNeowChoice(index) => {
                if let VisibleStates::Neow(blessings) = self.visible_screen {
                    let blessing = blessings[index];
                    self._apply_neow_blessing(blessing);
                }
            }
        }
    }

    fn _apply_neow_blessing(&mut self, blessing: NeowsBlessing) {
        match blessing {
            NeowsBlessing::First(bless) => match bless {
                crate::screens::FirstBlessing::Remove => todo!(),
                crate::screens::FirstBlessing::Transform => todo!(),
                crate::screens::FirstBlessing::Upgrade => todo!(),
                crate::screens::FirstBlessing::ChooseClassCard => todo!(),
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
                crate::screens::SecondBlessing::NeowsLament => self.relics.add(Relic::NeowsLament),
                crate::screens::SecondBlessing::RandomCommonRelic => {
                    let relic = self.relics.random_common();
                    self.relics.add(relic)
                }
                crate::screens::SecondBlessing::Gold100 => self.gold += Number(100),
                crate::screens::SecondBlessing::Random3Potions => todo!(),
            },
            NeowsBlessing::Third(bless) => {
                match bless.upside {
                    crate::screens::ThirdUpside::Remove2 => todo!(),
                    crate::screens::ThirdUpside::Transform2 => todo!(),
                    crate::screens::ThirdUpside::Gold250 => todo!(),
                    crate::screens::ThirdUpside::ChooseRareClassCard => todo!(),
                    crate::screens::ThirdUpside::ChooseRareColorless => todo!(),
                    crate::screens::ThirdUpside::BigMaxHP => todo!(),
                }

                match bless.downside {
                    crate::screens::ThirdDownside::LoseMaxHealth => todo!(),
                    crate::screens::ThirdDownside::RandomCurse => todo!(),
                    crate::screens::ThirdDownside::TakeDamage => todo!(),
                    crate::screens::ThirdDownside::LoseAllGold => todo!(),
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
