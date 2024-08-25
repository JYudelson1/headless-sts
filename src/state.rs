use std::{collections::HashSet, mem};

use uuid::Uuid;

use crate::{
    actions::{Action, CardRewardChoice, RewardChoice},
    cardrewardrng::CardRewardRng,
    cards::{make_card, make_starter_deck, MasterCard},
    combat::{CardInHandPurpose, CombatOver, Elites},
    map::{Map, RoomNode},
    potions::{potion_bag::PotionBag, potion_rng::PotionRng},
    question_rng::QuestionMarkRng,
    relics::Relics,
    screens::{EventsPool, VisibleStates},
    utils::{Act, Character, Keys, NotImplemented, Number, StillPlaying},
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
    pub potion_rng: PotionRng,
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
            potion_rng: PotionRng::new(),
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        //assert!(self.get_actions().contains(&action));

        match action {
            Action::PlayUntargetedCard(index) => {
                let possible_end = self.play_card_from_hand(index, None);
                self.maybe_end_combat(possible_end);
            },
            Action::PlayTargetedCard((index, enemy)) => {
                let possible_end = self.play_card_from_hand(index, Some(enemy));
                self.maybe_end_combat(possible_end);
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
            Action::EndTurn => {
                if let Err(err) = self.end_turn() {
                    self.still_playing = StillPlaying::NotImplementedError(err);
                }
            },
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
            Action::TakeRelicLeave(relic) => {self.collect_relic(relic); self.to_map() },
            Action::TakeKeyLeave(key) => { self.keys.add_key(key); self.to_map() },
            Action::UsePotionNoTargets(index) => {
                let possible_end = self.use_potion(index, None);
                self.maybe_end_combat(possible_end)
            },
            Action::UsePotionTargets((index, enemy_index)) => {
                let possible_end = self.use_potion(index, Some(enemy_index));
                self.maybe_end_combat(possible_end)
            },
            Action::DiscardPotion(index) => self.discard_potion(index),
            Action::Duplicate(id) => {
                self.duplicate_card_in_deck(id);
                self.to_map(); //TODO: Dolly's mirror should send back to shop sometimes
            }
            Action::ChooseCardInHand(id) => {
                let mut change = false;
                if let VisibleStates::ChoosingCardInHand((_, _, left, in_hand, already_chosen)) = &mut self.visible_screen {
                    *left -= 1;
                    let _ = in_hand.remove(&id);
                    let _ = already_chosen.insert(id);
                    change = true
                }
                if change {
                    let screen = mem::replace(&mut self.visible_screen, VisibleStates::Rest);
                    if let VisibleStates::ChoosingCardInHand((combat, purpose, _, _, cards)) = screen {
                        self.visible_screen = VisibleStates::Combat(combat);
                        match purpose {
                            CardInHandPurpose::Exhaust => {
                                let exhaust_result = self.exhaust_many(cards);
                                self.maybe_end_combat(exhaust_result);
                            },
                            CardInHandPurpose::PutOnTopOfDeck => self.put_from_hand_to_deck(cards),
                            CardInHandPurpose::Duplicate => todo!(),
                            CardInHandPurpose::Upgrade => todo!(),
                        }
                    }
                }
            },
        }
    }

    fn maybe_end_combat(&mut self, possible_end: Result<CombatOver, NotImplemented>) {
        match possible_end {
            Ok(CombatOver::Yes) => {
                if let Err(error) = self.end_combat() {
                    self.still_playing = StillPlaying::NotImplementedError(error)
                }
            }
            Ok(CombatOver::No) => (),
            Err(error) => self.still_playing = StillPlaying::NotImplementedError(error),
        }
    }

    fn put_from_hand_to_deck(&mut self, cards: HashSet<Uuid>) {
        for id in cards.iter() {
            let card = self.get_combat().get_card_from_hand(*id);
            self.get_combat().put_on_deck(card);
        }
    }

    fn exhaust_many(&mut self, cards: HashSet<Uuid>) -> Result<CombatOver, NotImplemented> {
        let relics = &self.relics.clone();
        for id in cards.iter() {
            let card = self.get_combat().get_card_from_hand(*id);
            let over = self.get_combat().exhaust_card(card, relics)?;
            if over == CombatOver::Yes {
                return Ok(CombatOver::Yes);
            }
        }
        Ok(CombatOver::No)
    }
}
