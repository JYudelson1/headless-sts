mod neow;
mod rest;
mod rewards;
mod treasure;

pub use neow::*;
pub use rewards::{CardReward, RewardsScreen};
use treasure::Chest;

use crate::{
    actions::{Action, CardRewardChoice, RewardChoice},
    cardrewardrng::CombatType,
    combat::{get_enemies, Combat},
    map::{Map, RoomType},
    state::State,
    utils::Key,
};

pub enum VisibleStates {
    Reward(RewardsScreen),
    CardReward(Vec<CardReward>),
    Neow([NeowsBlessing; 4]),
    Map(Map),
    Combat(Combat),
    Treasure(Chest),
    Rest,
    RemoveCardScreen,
    UpgradeCardScreen
}

impl VisibleStates {
    pub fn new() -> Self {
        Self::Neow(get_neow_blessings())
    }
}

impl State {
    fn to_map(&mut self) {
        self.visible_screen = VisibleStates::Map(self.map.clone());
    }

    fn to_combat(&mut self, combat_type: CombatType) {
        // Reset Relics for combat
        self.relics.reset_start_of_combat();
        // Get the enemies
        let enemies = get_enemies(&self.act, self.current_floor, combat_type);
        let combat = Combat::new(enemies, combat_type, self.ascension, &self.relics, &self.main_deck);
        self.visible_screen = VisibleStates::Combat(combat);
    }

    fn to_treasure(&mut self) {
        let has_sapphire_key = self.keys.has_key(&Key::Sapphire);
        let chest = Chest::new_random(has_sapphire_key, &mut self.relics);

        self.visible_screen = VisibleStates::Treasure(chest);
    }

    fn to_rest(&mut self) {
        self.visible_screen = VisibleStates::Rest;

        // Ancient tea set proc:
        self.relics.turn_on_tea_set();
    }

    pub fn combat_finished(&mut self) {
        self.visible_screen = VisibleStates::Reward(self.make_rewards_screen());
    }

    pub fn _go_to_new_room(&mut self, room: RoomType) {
        match room {
            RoomType::Monster => self.to_combat(CombatType::Normal),
            RoomType::Event => todo!(),
            RoomType::Elite => self.to_combat(CombatType::Elite),
            RoomType::Rest => self.to_rest(),
            RoomType::Merchant => todo!(),
            RoomType::Treasure => self.to_treasure(),
            RoomType::Boss => todo!(),
        }
        // TODO: Activate maw bank
    }

    pub fn get_actions(&self) -> Vec<Action> {
        let mut actions = vec![];
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
            VisibleStates::Combat(_) => todo!(),
            VisibleStates::Treasure(_) => todo!(),
            VisibleStates::Rest => {
                actions.append(&mut self.get_rest_actions());
            }
            VisibleStates::RemoveCardScreen => {
                for card in &self.main_deck {
                    if card.card().can_be_removed() {
                        actions.push(Action::Remove(card.id));
                    }
                }
            },
            VisibleStates::UpgradeCardScreen => {
                for card in &self.main_deck {
                    if card.card().can_be_upgraded() {
                        actions.push(Action::Upgrade(card.id));
                    }
                }
            },
        }

        actions
    }
}
