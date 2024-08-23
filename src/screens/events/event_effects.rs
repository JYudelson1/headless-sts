use crate::{
    cards::{make_card, CardName},
    screens::shop::random_relic,
    state::State,
    utils::NotImplemented,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EventAction {
    Heal(u16),
    GainMaxHp(u16),
    RandomRelic,
    GainBlankCard(CardName),
    Leave,
    Multiple(Vec<EventAction>),
    GainGold(u32),
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
        }

        // By default, every action results in going back to the map
        self.to_map();

        Ok(())
    }
}
