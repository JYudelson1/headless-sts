use std::collections::HashSet;

use crate::utils::Act;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Events {
    BigFish,
    DeadAdventurer,
    FaceTrader,
    GoldenIdol,
    HypnotizingColoredMushrooms,
    LivingWall,
    ScrapOoze,
    ShiningLight,
    TheCleric,
    TheSsssserpent,
    WorldOfGoop,
    WingStatue,
    BonfireSpirits,
    Duplicator,
    GoldenShrine,
    Lab,
    MatchAndKeep,
    OminousForge,
    Purifier,
    TheDivineFountain,
    TheWomanInBlue,
    Transmogrifier,
    UpgradeShrine,
    WeMeetAgain,
    WheelofChange,
    DesignerInSpire,
}

#[derive(Debug)]
pub struct EventsPool(HashSet<Events>);

impl EventsPool {
    pub fn new() -> Self {
        let mut events = HashSet::new();
        for event in ACT_1_EVENTS {
            events.insert(*event);
        }
        for event in SHARED_EVENTS {
            events.insert(*event);
        }
        for event in ACT_1_AND_2_EVENTS {
            events.insert(*event);
        }
        Self(events)
    }

    pub fn next_act(&mut self, act: Act) {
        match act {
            Act::Act1 => unreachable!(),
            Act::Act2 => {
                for event in ACT_1_EVENTS {
                    self.0.remove(event);
                }
                for event in ACT_2_EVENTS {
                    self.0.insert(*event);
                }
                for event in ACT_2_AND_3_EVENTS {
                    self.0.insert(*event);
                }
            }
            Act::Act3 => {
                for event in ACT_2_EVENTS {
                    self.0.remove(event);
                }
                for event in ACT_1_AND_2_EVENTS {
                    self.0.remove(event);
                }
                for event in ACT_3_EVENTS {
                    self.0.insert(*event);
                }
            }
        }
    }

    pub fn random(&mut self) -> Events {
        let item = self.0.iter().next().unwrap().clone();
        self.0.remove(&item);
        item
    }
}

const ACT_1_EVENTS: &[Events] = &[
    Events::BigFish,
    Events::DeadAdventurer,
    Events::FaceTrader,
    Events::GoldenIdol,
    Events::HypnotizingColoredMushrooms,
    Events::LivingWall,
    Events::ScrapOoze,
    Events::ShiningLight,
    Events::TheCleric,
    Events::TheSsssserpent,
    Events::WorldOfGoop,
    Events::WingStatue,
];

const ACT_2_EVENTS: &[Events] = &[]; //TODO!
const ACT_3_EVENTS: &[Events] = &[]; //TODO!
const ACT_1_AND_2_EVENTS: &[Events] = &[Events::FaceTrader];
const ACT_2_AND_3_EVENTS: &[Events] = &[Events::DesignerInSpire];

const SHARED_EVENTS: &[Events] = &[
    Events::BonfireSpirits,
    Events::Duplicator,
    Events::GoldenShrine,
    Events::Lab,
    Events::MatchAndKeep,
    Events::OminousForge,
    Events::Purifier,
    Events::TheDivineFountain,
    Events::TheWomanInBlue,
    Events::Transmogrifier,
    Events::UpgradeShrine,
    Events::WeMeetAgain,
    Events::WheelofChange,
];
