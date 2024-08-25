mod events;
mod neow;
mod rest;
mod rewards;
mod shop;
mod treasure;

use std::collections::HashSet;

pub use events::*;
pub use neow::*;
pub use rewards::{CardReward, RewardsScreen};
pub use shop::Wares;
use treasure::{Chest, ChestRelicType};
use uuid::Uuid;

use crate::{
    actions::{Action, CardRewardChoice, RewardChoice},
    cardrewardrng::CombatType,
    cards::{CardActions, CardIndex, MasterCard},
    combat::{get_enemies, CardInHandPurpose, Combat},
    enemies::EnemyIndex,
    map::{Map, RoomType},
    question_rng::QuestionMark,
    relics::Relic,
    state::State,
    utils::{Act, Key, NotImplemented},
};

#[derive(Debug)]
pub enum VisibleStates {
    Reward(RewardsScreen),
    CardReward(Vec<CardReward>),
    Neow([NeowsBlessing; 4]),
    Map(Map),
    Combat(Combat),
    Treasure(ChestRelicType),
    Rest,
    Shop(Vec<Wares>),
    RemoveCardScreen(usize),
    UpgradeCardScreen(usize),
    TransformCardScreen(usize),
    DuplicateCardScreen,
    Event(Events),
    ChoosingCardInHand((Combat, CardInHandPurpose, usize, HashSet<Uuid>, HashSet<Uuid>, Option<Vec<CardActions>>))
}

impl VisibleStates {
    pub fn new() -> Self {
        Self::Neow(get_neow_blessings())
    }
}

impl State {
    pub fn to_map(&mut self) {
        self.visible_screen = VisibleStates::Map(self.map.clone());
    }

    fn to_combat(&mut self, combat_type: CombatType) -> Result<(), NotImplemented> {
        // Reset Relics for combat
        self.relics.reset_start_of_combat();
        // Get the enemies
        let enemies = get_enemies(
            &self.act,
            combat_type,
            self.last_elite,
            self.fights_this_act,
            self.map.boss
        );

        //println!("Fighting {enemies:?}");

        let combat = Combat::new(
            enemies,
            combat_type,
            self.ascension,
            &mut self.relics,
            &self.main_deck,
            self.current_health
        );
        self.visible_screen = VisibleStates::Combat(combat?);

        // Note: Just starting combat will never insta-end it
        let _ = self.start_combat_turn()?;

        Ok(())
    }

    fn to_treasure(&mut self) -> Result<(), NotImplemented> {
        let has_sapphire_key = self.keys.has_key(&Key::Sapphire);
        let chest = Chest::new_random(has_sapphire_key, &mut self.relics);

        // Auto pickup the gold
        self.gold += chest.gold;

        self.visible_screen = VisibleStates::Treasure(chest.relic);

        Ok(())
    }

    fn to_rest(&mut self) {
        self.visible_screen = VisibleStates::Rest;

        // Ancient tea set proc:
        self.relics.turn_on_tea_set();
    }

    fn to_shop(&mut self) -> Result<(), NotImplemented> {
        // Meal ticket
        if self.relics.contains(Relic::MealTicket) {
            self.heal(15);
        }
        // Construct shop
        let shop = Wares::new(&mut self.relics, self.card_removes_bought, self.character, self.ascension)?;
        self.visible_screen = VisibleStates::Shop(shop);

        Ok(())
    }

    fn to_event(&mut self) -> Result<(), NotImplemented> {
        // TODO: Ssserpent head

        // Get the event
        let event = self.event_pool.random();
        self.visible_screen = VisibleStates::Event(event);

        Ok(())
    }

    fn to_question_mark(&mut self) -> Result<(), NotImplemented> {
        // Serpent head
        if self.relics.contains(Relic::SerpentHead) {
            self.gold += 50;
        }

        match self.question_rng.get_question_mark(&mut self.relics) {
            QuestionMark::NormalFight => self.to_combat(CombatType::Normal),
            QuestionMark::TreasureRoom => self.to_treasure(),
            QuestionMark::Shop => self.to_shop(),
            QuestionMark::Event => self.to_event(),
        }
    }

    pub fn combat_finished(&mut self) -> Result<(), NotImplemented> {
        let combat_type = self.get_combat().combat_type;

        if combat_type != CombatType::Boss {
            self.visible_screen = VisibleStates::Reward(self.make_rewards_screen());
        } else {
            Err(NotImplemented::DefeatedBoss)?
        }

        Ok(())
    }

    pub fn _go_to_new_room(&mut self, room: RoomType) -> Result<(), NotImplemented> {
        match room {
            RoomType::Monster => {
                self.fights_this_act += 1;
                self.to_combat(CombatType::Normal)?
            },
            RoomType::Event => self.to_question_mark()?,
            RoomType::Elite => self.to_combat(CombatType::Elite)?,
            RoomType::Rest => self.to_rest(),
            RoomType::Merchant => self.to_shop()?,
            RoomType::Treasure => self.to_treasure()?,
            RoomType::Boss => {
                // Reset easy/hard encounter pool
                self.fights_this_act = 0;
                // Panto Heal
                if self.relics.contains(Relic::Pantograph) {
                    self.heal(25);
                }
                // Reset events for next act
                if self.act == Act::Act1 {
                    self.event_pool.next_act(Act::Act2);
                } else if self.act == Act::Act2 {
                    self.event_pool.next_act(Act::Act3);
                }
                // Reset potion rng for next fight
                self.potion_rng.reset();

                println!("Made it to the boss!! at {}", self.map.current_floor());
                println!("Deck: {:?}", self.main_deck);
                println!("relics: {:?}", self.relics.list);
                println!("{}", self.map);
                self.to_combat(CombatType::Boss)?;
                
            },
        };
        // TODO: Activate maw bank

        Ok(())
    }

    pub fn get_actions(&self) -> Result<Vec<Action>, NotImplemented> {
        let mut actions = vec![];
        // Can always discard any potion
        for i in 0..self.potions.potions.len() {
            actions.push(Action::DiscardPotion(i))
        }
        // Screen-dependent actions
        match &self.visible_screen {
            VisibleStates::Reward(rewards) => {
                actions.push(Action::CollectReward(RewardChoice::Skip));
                for i in 0..rewards.0.len() {
                    actions.push(Action::CollectReward(RewardChoice::RewardIndex(i)));
                }
            }
            VisibleStates::CardReward(card_rewards) => {
                actions.push(Action::MakeCardChoice(CardRewardChoice::Skip));
                for i in 0..card_rewards.len() {
                    actions.push(Action::MakeCardChoice(CardRewardChoice::CardRewardIndex(i)));
                }
            }
            VisibleStates::Neow(_) => {
                // TODO: Support for only one choice
                for i in 0..4 {
                    actions.push(Action::MakeNeowChoice(i));
                }
            }
            VisibleStates::Map(map) => {
                for node in map.next_rooms() {
                    actions.push(Action::TraverseMap(node.x as u8));
                }
            }
            VisibleStates::Combat(combat) => {
                // println!("{combat:?}");
                // Can always end turn
                actions.push(Action::EndTurn);
                // TODO: Add potions
                
                // Add every playable card in hand
                // Cards are playable if they say so, and you have enough energy
                // TODO: Relics
                // TODO: Medkit, Blue Candle
                for (i, card) in combat.hand.iter().enumerate() {
                    if !card.card().is_playable(&combat.hand) { continue }
                    if card.card().get_cost() > combat.current_energy { continue }
                    // Targeted cards are added for each possible target
                    if card.card().targets() {
                        for e in 0..combat.num_enemies() {
                            actions.push(Action::PlayTargetedCard((CardIndex(i), EnemyIndex(e))));
                        }
                    } else {
                        actions.push(Action::PlayUntargetedCard(CardIndex(i)));
                    }
                }
                // Combat potions
                for (i, potion) in self.potions.potions.iter().enumerate() {
                if potion.is_combat() {
                    if potion.targets() {
                        // Potion can be used for every target
                        for e in 0..combat.num_enemies() {
                            actions.push(Action::UsePotionTargets((i, EnemyIndex(e))));
                        }
                    } else {
                        actions.push(Action::UsePotionNoTargets(i));
                    }
                }
            }
            }
            VisibleStates::Treasure(chest) => {
                actions.push(Action::LeaveShop);
                match chest {
                    ChestRelicType::None => (),
                    ChestRelicType::Relic(relic) => {
                        actions.push(Action::TakeRelicLeave(relic.clone()));
                    },
                    ChestRelicType::RelicOrKey(relic) => {
                        actions.push(Action::TakeRelicLeave(relic.clone()));
                        actions.push(Action::TakeKeyLeave(Key::Sapphire))
                    },
                }
            },
            VisibleStates::Rest => {
                actions.append(&mut self.get_rest_actions());
            }
            VisibleStates::RemoveCardScreen(_) => {
                for card in &self.main_deck {
                    if card.card().can_be_removed() {
                        actions.push(Action::Remove(card.id));
                    }
                }
            },
            VisibleStates::UpgradeCardScreen(_) => {
                for card in &self.main_deck {
                    if card.card().can_be_upgraded() {
                        actions.push(Action::Upgrade(card.id));
                    }
                }
            },
            VisibleStates::TransformCardScreen(_) => {
                for card in &self.main_deck {
                    if card.card().can_be_removed() {
                        actions.push(Action::Transform(card.id));
                    }
                }
            },
            VisibleStates::Shop(wares) => {
                actions.push(Action::LeaveShop);
                for ware in wares {
                    if self.gold >= ware.cost() {
                        actions.push(Action::Purchase(ware.clone()));
                    }
                }
            },
            VisibleStates::Event(event) => {
                for action in event.actions(&self)? {
                    actions.push(Action::EventAction(action));
                }
            },
            VisibleStates::DuplicateCardScreen => {
                for card in &self.main_deck {
                    actions.push(Action::Duplicate(card.id));
                }
            },
            VisibleStates::ChoosingCardInHand((_, _, left, in_hand, _, _)) => {
                assert!(*left > 0);
                for id in in_hand {
                    actions.push(Action::ChooseCardInHand(*id))
                }
            },
        }

        if !self.is_in_combat() {
            // Can use non-combat potions here
            for (i, potion) in self.potions.potions.iter().enumerate() {
                if potion.is_noncombat() {
                    actions.push(Action::UsePotionNoTargets(i));
                }
            }
        }

        Ok(actions)
    }
}
