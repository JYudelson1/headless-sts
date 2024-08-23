use crate::{relics::Relic, state::State};

impl State {
    pub fn collect_relic(&mut self, relic: Relic) {
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
            // Relic::WarPaint => {
            //     todo!()
            // }
            // Relic::Whetstone => {
            //     todo!()
            // }
            // Relic::BottledFlame => {
            //     todo!()
            // }
            // Relic::BottledLightning => {
            //     todo!()
            // }
            // Relic::BottledTornado => {
            //     todo!()
            // }
            // Relic::DollysMirror => {
            //     todo!()
            // }
            // Relic::Astrolabe => {
            //     //TODO
            // }
            // Relic::PandorasBox => {
            //     todo!()
            // }
            // Relic::CallingBell => {
            //     todo!()
            // }
            // Relic::EmptyCage => {
            //     todo!()
            // }
            // Relic::TinyHouse => {
            //     todo!()
            // }
            // Relic::Necronomicon => {
            //     todo!()
            // }

            _ => (),
        }
        self.relics.add(relic);
    }
}
