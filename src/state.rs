use crate::{
    actions::{Action, CardRewardChoice, RewardChoice},
    cardrewardrng::CardRewardRng,
    cards::{make_card, make_starter_deck, MasterCard},
    combat::Elites,
    map::{Map, RoomNode},
    potions::PotionBag,
    question_rng::QuestionMarkRng,
    relics::Relics,
    screens::VisibleStates,
    utils::{Act, Character, Keys, Number},
};

#[derive(Debug)]
pub struct State {
    pub act: Act,
    pub visible_screen: VisibleStates,
    pub card_rng: CardRewardRng,
    pub potions: PotionBag,
    pub map: Map,
    pub ascension: u8,
    pub max_health: Number,
    pub current_health: u16,
    pub gold: u32,
    pub current_floor: u8,
    pub character: Character,
    pub relics: Relics,
    pub main_deck: Vec<MasterCard>,
    pub keys: Keys,
    pub still_playing: bool,
    pub question_rng: QuestionMarkRng,
    pub last_elite: Option<Elites>,
    pub fights_this_act: u8,
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
            max_health: Number(80),
            current_health: if ascension >= 6 { 72 } else { 80 },
            gold: 99,
            current_floor: 0,
            character,
            relics: Relics::new(character),
            main_deck: make_starter_deck(character),
            keys: Keys::new(),
            still_playing: true,
            question_rng: QuestionMarkRng::new(),
            last_elite: None,
            fights_this_act: 0,
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        assert!(self.get_actions().contains(&action));

        match action {
            Action::PlayUntargetedCard(index) => {
                self.play_card(index, None);
            },
            Action::PlayTargetedCard((index, enemy)) => {
                self.play_card(index, Some(enemy));
            },
            Action::CollectReward(choice) => {
                match choice {
                    RewardChoice::Skip => self.to_map(),
                    RewardChoice::RewardIndex(index) => {
                        self.take_reward(index);
                    },
                }
            },
            Action::MakeCardChoice(choice) => {
                match choice {
                    // TODO: This should send to map in the case of a whale bonus
                    // Or a dreamcatcher card reward
                    // Maybe they should be seperate actions??
                    // TODO: When skipping, should send back to reward screen
                    CardRewardChoice::Skip => (),
                    CardRewardChoice::CardRewardIndex(i) => {
                        if let VisibleStates::CardReward(cards) = &self.visible_screen {
                            let card_reward = &cards[i];
                            let card = make_card(card_reward.card, card_reward.is_upgraded);
                            self.main_deck.push(card);
                        } else {
                            panic!("Making card choice not on CardReward screen!");
                        }
                    },
                }
                self.to_map();
            },
            Action::EndTurn => self.end_turn(),
            Action::TraverseMap(node_x) => {
                let node = RoomNode {
                    x: node_x as usize,
                    floor: self.map.next_floor_num()
                };
                let room_type = self.map.get_room(node).expect("Options should all be real rooms");

                // Update map location
                self.map.go_to_room(node);

                // Change the screen
                self._go_to_new_room(room_type);
            },
            Action::MakeNeowChoice(index) => {
                if let VisibleStates::Neow(blessings) = self.visible_screen {
                    let blessing = blessings[index];
                    self._apply_neow_blessing(blessing);

                    if matches!(self.visible_screen, VisibleStates::Neow(_)) {
                        self.to_map();
                    }
                }
            }
            Action::MakeRestChoice(choice) => self.apply_rest_choice(choice),
            Action::Upgrade(id) => {
                self.upgrade_card_in_deck(id);
                if matches!(self.visible_screen, VisibleStates::UpgradeCardScreen) {
                    self.to_map();
                }
            },
            Action::Remove(id) => {
                self.remove_card_in_deck(id);
                if matches!(self.visible_screen, VisibleStates::RemoveCardScreen) {
                    self.to_map();
                }
            },
        }

    }
}
