mod event_pools;
pub use event_effects::EventAction;
pub use event_pools::{Events, EventsPool};
pub mod event_effects;

mod big_fish;
mod face_trader;
mod scrap_ooze;
mod serpent;

use crate::{state::State, utils::NotImplemented};

impl Events {
    pub fn actions(&self, state: &State) -> Result<Vec<EventAction>, NotImplemented> {
        let actions = match self {
            Events::BigFish => big_fish::actions(state),
            Events::DeadAdventurer => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::FaceTrader => face_trader::actions(state.max_health.0, state.ascension), 
            Events::GoldenIdol => Err(NotImplemented::Event(self.clone()))?,     // TODO
            Events::HypnotizingColoredMushrooms => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::LivingWall => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::ScrapOoze(amt) => scrap_ooze::actions(state, *amt), 
            Events::ShiningLight => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::TheCleric => Err(NotImplemented::Event(self.clone()))?,  // TODO
            Events::TheSsssserpent => serpent::actions(state),
            Events::WorldOfGoop => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::WingStatue => Err(NotImplemented::Event(self.clone()))?,  // TODO
            Events::BonfireSpirits => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Duplicator => vec![EventAction::Leave, EventAction::Duplicate],
            Events::GoldenShrine => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Lab => Err(NotImplemented::Event(self.clone()))?,         // TODO
            Events::MatchAndKeep => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::OminousForge => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Purifier => vec![EventAction::Leave, EventAction::Remove],
            Events::TheDivineFountain => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::TheWomanInBlue => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::Transmogrifier => vec![EventAction::Leave, EventAction::Transform],
            Events::UpgradeShrine => vec![EventAction::Leave, EventAction::Upgrade],
            Events::WeMeetAgain => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::WheelofChange => Err(NotImplemented::Event(self.clone()))?, // TODO
            Events::DesignerInSpire => Err(NotImplemented::Event(self.clone()))?, // TODO
        };

        Ok(actions)
    }
}
