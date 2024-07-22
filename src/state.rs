use crate::{
    combat::CardRewardRng,
    map::Map,
    potions::PotionBag,
    screens::VisibleStates,
    utils::{Act, Number},
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
    pub gold: u16,
    pub current_floor: u8,
}

impl State {
    pub fn new(ascension: u8) -> Self {
        Self {
            act: Act::Act1,
            visible_screen: todo!(),
            card_rng: CardRewardRng::new(),
            potions: PotionBag::new(ascension),
            map: Map::new(Act::Act1, ascension),
            ascension,
            //TODO: non-ironclad
            max_health: Number(80),
            current_health: if ascension >= 6 { 72 } else { 80 },
            gold: 99,
            current_floor: 0,
        }
    }
}
