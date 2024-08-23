mod event_pools;
pub use event_effects::EventAction;
pub use event_pools::{Events, EventsPool};
pub mod event_effects;

mod big_fish;
mod serpent;

use crate::{state::State, utils::NotImplemented};

impl Events {
    pub fn actions(&self, state: &State) -> Result<Vec<EventAction>, NotImplemented> {
        let actions = match self {
            Events::BigFish => big_fish::actions(state),
            Events::DeadAdventurer => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::FaceTrader => Err(NotImplemented::Event(self.clone()))?,     // TODO
            Events::GoldenIdol => Err(NotImplemented::Event(self.clone()))?,     // TODO
            Events::HypnotizingColoredMushrooms => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::LivingWall => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::ScrapOoze => Err(NotImplemented::Event(self.clone()))?,  // TODO
            Events::ShiningLight => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::TheCleric => Err(NotImplemented::Event(self.clone()))?,  // TODO
            Events::TheSsssserpent => serpent::actions(state),
            Events::WorldOfGoop => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::WingStatue => Err(NotImplemented::Event(self.clone()))?,  // TODO
            Events::BonfireSpirits => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Duplicator => Err(NotImplemented::Event(self.clone()))?,  // TODO
            Events::GoldenShrine => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Lab => Err(NotImplemented::Event(self.clone()))?,         // TODO
            Events::MatchAndKeep => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::OminousForge => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Purifier => Err(NotImplemented::Event(self.clone()))?,    // TODO
            Events::TheDivineFountain => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::TheWomanInBlue => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Transmogrifier => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::UpgradeShrine => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::WeMeetAgain => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::WheelofChange => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::DesignerInSpire => Err(NotImplemented::Event(self.clone()))?, // TODO
        };

        Ok(actions)
    }
}
