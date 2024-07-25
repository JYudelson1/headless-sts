use crate::{relics::Relic, state::State, utils::Number};

pub fn collect_relic(state: &mut State, relic: Relic) {
    match relic {
        Relic::PotionBelt => state.potions.increase_size(2),
        Relic::Strawberry => state.increase_max_hp(7),
        Relic::Pear => state.increase_max_hp(10),
        Relic::Mango => state.increase_max_hp(14),
        Relic::LeesWaffle => {
            state.increase_max_hp(7);
            state.current_health = state.max_health.0 as u16;
        }
        Relic::OldCoin => state.gold += Number(300),
        Relic::WarPaint => {
            // TODO
        }
        Relic::Whetstone => {
            // TODO
        }
        Relic::BottledFlame => {
            // TODO
        }
        Relic::BottledLightning => {
            // TODO
        }
        Relic::BottledTornado => {
            // TODO
        }
        Relic::DollysMirror => {
            // TODO
        }
        Relic::Astrolabe => {
            //TODO
        }
        Relic::PandorasBox => {
            // TODO
        }
        Relic::CallingBell => {
            // TODO
        }
        Relic::EmptyCage => {
            // TODO
        }
        Relic::TinyHouse => {
            // TODO
        }
        _ => (),
    }
    state.relics.add(relic);
}
