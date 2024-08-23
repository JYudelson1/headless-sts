use crate::{
    cards::{make_card, CardName},
    screens::{shop::random_relic, VisibleStates},
    state::State,
    utils::NotImplemented,
};

use super::Events;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EventAction {
    Heal(u16),
    GainMaxHp(u16),
    RandomRelic,
    GainBlankCard(CardName),
    Leave,
    Multiple(Vec<EventAction>),
    GainGold(u32),
    LoseHp(u16),
    ChanceForAction((Box<EventAction>, u8, Option<Box<EventAction>>)),
    GoToScrapOoze(usize),
    Remove,
    Upgrade,
    Transform,
}

impl State {
    pub fn apply_event_action(&mut self, event_action: EventAction) -> Result<(), NotImplemented> {
        match event_action {
            EventAction::Heal(amt) => self.heal(amt),
            EventAction::GainMaxHp(amt) => self.increase_max_hp(amt),
            EventAction::Leave => (),
            EventAction::Multiple(inner) => {
                for action in inner {
                    self.apply_event_action(action)?;
                    return Ok(()); // We return early so as not to swallow screen changes
                }
            }
            EventAction::RandomRelic => {
                let relic = random_relic(&mut self.relics).0;
                self.collect_relic(relic);
            }
            EventAction::GainBlankCard(name) => {
                let card = make_card(name, false)?;
                self.add_to_deck(card);
            }
            EventAction::GainGold(amt) => self.gold += amt,
            EventAction::LoseHp(amt) => self.lose_hp(amt),
            EventAction::ChanceForAction((action, prob, otherwise)) => {
                let prob = prob as f32 / 100.0;
                if rand::random::<f32>() < prob {
                    self.apply_event_action(action.as_ref().clone())?
                } else {
                    if let Some(other) = otherwise {
                        self.apply_event_action(other.as_ref().clone())?
                    }
                }
            },
            EventAction::GoToScrapOoze(amt) => {
                self.visible_screen = VisibleStates::Event(Events::ScrapOoze(amt));
                return Ok(())
            },
            EventAction::Remove => {
                self.visible_screen = VisibleStates::RemoveCardScreen(1);
                return Ok(());
            },
            EventAction::Upgrade => {
                self.visible_screen = VisibleStates::UpgradeCardScreen(1);
                return Ok(());
            },
            EventAction::Transform => {
                self.visible_screen = VisibleStates::TransformCardScreen(1);
                return Ok(());
            },
        }

        // By default, every action results in going back to the map
        self.to_map();

        Ok(())
    }
}
