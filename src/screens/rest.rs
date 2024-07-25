use crate::{
    actions::{Action, RestChoice},
    relics::Relic,
    state::State,
    utils::Key,
};

impl State {
    pub fn get_rest_actions(&self) -> Vec<Action> {
        let mut actions = vec![RestChoice::Skip];

        if !self.relics.contains(Relic::CoffeeDripper) {
            actions.push(RestChoice::Rest);
        }
        if !self.relics.contains(Relic::FusionHammer) {
            actions.push(RestChoice::Smith);
        }
        if self.relics.has_valid_girya() {
            actions.push(RestChoice::Lift);
        }
        if self.relics.contains(Relic::PeacePipe) {
            actions.push(RestChoice::Toke);
        }
        if self.relics.contains(Relic::Shovel) {
            actions.push(RestChoice::Dig);
        }
        if !self.keys.has_key(&crate::utils::Key::Ruby) {
            actions.push(RestChoice::TakeRubyKey)
        }

        actions
            .iter()
            .map(|choice| Action::MakeRestChoice(*choice))
            .collect()
    }

    pub fn apply_rest_choice(&mut self, choice: RestChoice) {
        match choice {
            RestChoice::Skip => (),
            RestChoice::Smith => todo!(),
            RestChoice::Rest => todo!(),
            RestChoice::Toke => todo!(),
            RestChoice::TakeRubyKey => self.keys.add_key(Key::Ruby),
            RestChoice::Lift => self.relics.increase_girya(),
            RestChoice::Dig => todo!(),
        }
    }
}
