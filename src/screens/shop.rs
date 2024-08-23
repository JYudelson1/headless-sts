use rand::prelude::SliceRandom;
use rand::random;

use crate::{
    cards::{make_card, CardName, CardType},
    potions::Potion,
    relics::{Relic, Relics},
    state::State,
    utils::{number_between, Character, NotImplemented, Rarity},
};

use super::VisibleStates;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Wares {
    Card((CardName, u32)),
    Relic((Relic, u32)),
    Potion((Potion, u32)),
    Removal(u32),
}

impl Wares {
    pub fn new(
        relics: &mut Relics,
        prev_removals: u8,
        character: Character,
        ascension: u8,
    ) -> Result<Vec<Self>, NotImplemented> {
        let mut wares = vec![];

        // Card Remove
        let removal_cost = if relics.contains(Relic::SmilingMask) {
            50
        } else {
            75 + (25 * prev_removals)
        };
        wares.push(Self::Removal(removal_cost as u32));

        // Class cards
        let cards = get_five_class_cards(character)?;
        let on_sale_index = number_between(0, 4);
        for (i, (card, rarity)) in cards.into_iter().enumerate() {
            let mut price = match rarity {
                Rarity::Common => number_between(45, 55),
                Rarity::Uncommon => number_between(68, 82),
                Rarity::Rare => number_between(135, 165),
                _ => panic!(),
            };
            if i == on_sale_index {
                price = (price as f32 * 0.5).floor() as u32;
            }
            if ascension >= 16 {
                price = (price as f32 * 1.1).floor() as u32;
            }
            wares.push(Self::Card((card, price)));
        }
        // Colorless Cards
        let uncommon_colorless = *CardName::colorless_uncommons()
            .choose(&mut rand::thread_rng())
            .unwrap();
        let mut price = number_between(81, 99);
        if ascension >= 16 {
            price = (price as f32 * 1.1).floor() as u32;
        }
        wares.push(Self::Card((uncommon_colorless, price)));

        let uncommon_rare = *CardName::colorless_rares()
            .choose(&mut rand::thread_rng())
            .unwrap();
        let mut price = number_between(162, 198);
        if ascension >= 16 {
            price = (price as f32 * 1.1).floor() as u32;
        }
        wares.push(Self::Card((uncommon_rare, price)));

        // Relics
        // First two relics are random, last is shop
        for _ in 0..2 {
            let (relic, rarity) = random_relic(relics);
            let mut price = match rarity {
                Rarity::Common => number_between(143, 157),
                Rarity::Uncommon => number_between(238, 262),
                Rarity::Rare => number_between(285, 315),
                _ => unreachable!(),
            };
            if ascension >= 16 {
                price = (price as f32 * 1.1).floor() as u32;
            }
            wares.push(Wares::Relic((relic, price)));
        }
        let mut shop_relic_price = number_between(143, 157);
        if ascension >= 16 {
            shop_relic_price = (shop_relic_price as f32 * 1.1).floor() as u32;
        }
        wares.push(Wares::Relic((relics.random_shop(), shop_relic_price)));

        // Potions
        // TODO: Potions

        // Membership card discount
        if relics.contains(Relic::MembershipCard) {
            for ware in &mut wares {
                ware.discount_price(0.5)
            }
        }

        Ok(wares)
    }

    pub fn cost(&self) -> u32 {
        match self {
            Wares::Card((_, cost)) => *cost,
            Wares::Relic((_, cost)) => *cost,
            Wares::Potion((_, cost)) => *cost,
            Wares::Removal(cost) => *cost,
        }
    }

    fn discount_price(&mut self, factor: f32) {
        let new_price = (self.cost() as f32 * factor).ceil() as u32;
        match self {
            Wares::Card((_, cost)) => *cost = new_price,
            Wares::Relic((_, cost)) => *cost = new_price,
            Wares::Potion((_, cost)) => *cost = new_price,
            Wares::Removal(cost) => *cost = new_price,
        }
    }
}

impl State {
    pub fn buy_wares(&mut self, ware: Wares) -> Result<(), NotImplemented> {
        // TODO: Break Maw Bank
        match ware {
            Wares::Card((card, cost)) => {
                let card = make_card(card, false)?;
                self.add_to_deck(card);
                self.gold -= cost;
            }
            Wares::Relic((relic, cost)) => {
                self.collect_relic(relic);
                self.gold -= cost
            }
            Wares::Potion((potion, cost)) => {
                self.potions.add(potion);
                self.gold -= cost
            }
            Wares::Removal(cost) => {
                self.gold -= cost;
                // TODO: Should be able to return after removing
                self.visible_screen = VisibleStates::RemoveCardScreen(1);
            }
        }

        Ok(())
    }
}

fn get_one_class_card(character: Character) -> (CardName, Rarity) {
    let rng = rand::random::<f32>();

    let (common, uncommon) = (0.6, 0.37);

    if rng < common {
        (CardName::random_common(character), Rarity::Common)
    } else if rng < common + uncommon {
        (CardName::random_uncommon(character), Rarity::Uncommon)
    } else {
        (CardName::random_rare(character), Rarity::Rare)
    }
}

fn get_five_class_cards(character: Character) -> Result<Vec<(CardName, Rarity)>, NotImplemented> {
    let mut powers = 0;
    let mut skills = 0;
    let mut attacks = 0;

    let mut cards = vec![];

    while cards.len() < 5 {
        let (card, rarity) = get_one_class_card(character);
        match make_card(card, false)?.card().get_type() {
            CardType::Attack => {
                if attacks < 2 {
                    attacks += 1;
                    cards.push((card, rarity))
                }
            }
            CardType::Power => {
                if powers <= 1 {
                    powers += 1;
                    cards.push((card, rarity))
                }
            }
            CardType::Skill => {
                if skills < 1 {
                    skills += 1;
                    cards.push((card, rarity))
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(cards)
}

pub fn random_relic(relics: &mut Relics) -> (Relic, Rarity) {
    let random = random::<f32>();

    let rarity = if random < 0.5 {
        Rarity::Common
    } else if random < 0.83 {
        Rarity::Uncommon
    } else {
        Rarity::Rare
    };

    let relic = match rarity {
        Rarity::Common => relics.random_common(),
        Rarity::Uncommon => relics.random_uncommon(),
        Rarity::Rare => relics.random_rare(),
        _ => unreachable!(),
    };
    (relic, rarity)
}
