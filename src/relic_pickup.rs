use crate::{relics::Relic, state::State, utils::NotImplemented};

impl State {
    pub fn collect_relic(&mut self, relic: Relic) -> Result<(), NotImplemented> {
        match relic {
            Relic::PotionBelt => self.potions.increase_size(2),
            Relic::Strawberry => self.increase_max_hp(7),
            Relic::Pear => self.increase_max_hp(10),
            Relic::Mango => self.increase_max_hp(14),
            Relic::LeesWaffle => {
                self.increase_max_hp(7);
                self.current_health = self.max_health.0 as u16;
            }
            Relic::OldCoin => self.gold += 300,
            Relic::WarPaint => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::Whetstone => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::BottledFlame => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::BottledLightning => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::BottledTornado => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::DollysMirror => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::Astrolabe => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::PandorasBox => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::CallingBell => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::EmptyCage => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::TinyHouse => {
                return Err(NotImplemented::Relic(relic));
            }
            Relic::Necronomicon => {
                return Err(NotImplemented::Relic(relic));
            }

            _ => (),
        }
        self.relics.add(relic);
        Ok(())
    }
}
