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
    cards::CardIndex,
    combat::{get_enemies, Combat},
    enemies::EnemyIndex,
    map::{Map, RoomType},
    question_rng::QuestionMark,
    relics::Relic,
    state::State,
    utils::Key,
};

#[derive(Debug)]
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
    pub fn to_map(&mut self) {
        self.visible_screen = VisibleStates::Map(self.map.clone());
    }

    fn to_combat(&mut self, combat_type: CombatType) {
        // Reset Relics for combat
        self.relics.reset_start_of_combat();
        // Get the enemies
        let enemies = get_enemies(&self.act, combat_type);
        let combat = Combat::new(enemies, combat_type, self.ascension, &self.relics, &self.main_deck);
        self.visible_screen = VisibleStates::Combat(combat);

        self.start_combat_turn();
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

    fn to_question_mark(&mut self) {
        // Serpent head
        if self.relics.contains(Relic::SerpentHead) {
            self.gold += 50;
        }

        match self.question_rng.get_question_mark(&mut self.relics) {
            QuestionMark::NormalFight => self.to_combat(CombatType::Normal),
            QuestionMark::TreasureRoom => self.to_treasure(),
            QuestionMark::Shop => todo!(),
            QuestionMark::Event => todo!(),
        }
    }

    pub fn combat_finished(&mut self) {
        self.visible_screen = VisibleStates::Reward(self.make_rewards_screen());
    }

    pub fn _go_to_new_room(&mut self, room: RoomType) {
        match room {
            RoomType::Monster => self.to_combat(CombatType::Normal),
            RoomType::Event => self.to_question_mark(),
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
                    if !card.card().is_playable() { continue }
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
            }
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
