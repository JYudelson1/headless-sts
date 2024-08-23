use crate::{
    actions::{Action, CardRewardChoice, RewardChoice},
    cardrewardrng::CardRewardRng,
    cards::{make_card, make_starter_deck, MasterCard},
    combat::Elites,
    map::{Map, RoomNode},
    potions::PotionBag,
    question_rng::QuestionMarkRng,
    relics::Relics,
    screens::{EventsPool, VisibleStates},
    utils::{Act, Character, Keys, Number, StillPlaying},
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
    pub still_playing: StillPlaying,
    pub question_rng: QuestionMarkRng,
    pub last_elite: Option<Elites>,
    pub fights_this_act: u8,
    pub card_removes_bought: u8,
    pub event_pool: EventsPool,
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
            still_playing: StillPlaying::Playing,
            question_rng: QuestionMarkRng::new(),
            last_elite: None,
            fights_this_act: 0,
            card_removes_bought: 0,
            event_pool: EventsPool::new(),
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        //assert!(self.get_actions().contains(&action));

        match action {
            Action::PlayUntargetedCard(index) => {
                if let Err(error) = self.play_card(index, None){
                    self.still_playing = StillPlaying::NotImplementedError(error)
                }
            },
            Action::PlayTargetedCard((index, enemy)) => {
                if let Err(error) = self.play_card(index, Some(enemy)) {
                    self.still_playing = StillPlaying::NotImplementedError(error)
                }
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
                    CardRewardChoice::Skip => self.to_map(),
                    CardRewardChoice::CardRewardIndex(i) => {
                        if let VisibleStates::CardReward(cards) = &self.visible_screen {
                            let card_reward = &cards[i];
                            let card = make_card(card_reward.card, card_reward.is_upgraded);
                            match card {
                                Ok(card) => {
                                    //println!("Obtained {:?}", card.card().name());
                                    self.add_to_deck(card);
                                    self.to_map();
                                },
                                Err(error) => {
                                    self.still_playing = StillPlaying::NotImplementedError(error)
                                },
                            }
                            
                        } else {
                            panic!("Making card choice not on CardReward screen!");
                        }
                    },
                }
                
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
                if let Err(error) = self._go_to_new_room(room_type) {
                    self.still_playing = StillPlaying::NotImplementedError(error);
                }
            },
            Action::MakeNeowChoice(index) => {
                if let VisibleStates::Neow(blessings) = self.visible_screen {
                    let blessing = blessings[index];
                
                    if let Err(error) = self._apply_neow_blessing(blessing) {
                        self.still_playing = StillPlaying::NotImplementedError(error)
                    } else {
                        // TODO: Is this matches redundant?
                        if matches!(self.visible_screen, VisibleStates::Neow(_)) {
                            self.to_map();
                        }
                    }

                }
            }
            Action::MakeRestChoice(choice) => self.apply_rest_choice(choice),
            Action::Upgrade(id) => {
                self.upgrade_card_in_deck(id);
                if let VisibleStates::UpgradeCardScreen(amt_to_upgrade) = &mut self.visible_screen {
                    if *amt_to_upgrade == 1 {
                        self.to_map();
                    } else {
                        *amt_to_upgrade -= 1;
                    }
                }
            },
            Action::Remove(id) => {
                self.remove_card_in_deck(id);
                if let VisibleStates::RemoveCardScreen(amt_to_remove) = &mut self.visible_screen {
                    if *amt_to_remove == 1 {
                        self.to_map();
                    } else {
                        *amt_to_remove -= 1;
                    }
                }
            },
            Action::Transform(id) => {
                if let Err(error) = self.transform_card_in_deck(id){
                    self.still_playing = StillPlaying::NotImplementedError(error);
                    return;
                }
                if let VisibleStates::TransformCardScreen(amt_to_transform) = &mut self.visible_screen {
                    if *amt_to_transform == 1 {
                        self.to_map();
                    } else {
                        *amt_to_transform -= 1;
                    }
                }
            }
            Action::Purchase(ware) => {
                if let Err(error) = self.buy_wares(ware) {
                    self.still_playing = StillPlaying::NotImplementedError(error);
                }
            },
            Action::LeaveShop => self.to_map(),
            Action::EventAction(event_action) => {
                if let Err(error) = self.apply_event_action(event_action) {
                    self.still_playing = StillPlaying::NotImplementedError(error)
                }
            },
        }

    }
}
