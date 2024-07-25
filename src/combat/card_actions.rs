use crate::{cards::{CardActions, CardIndex}, enemies::EnemyIndex, screens::VisibleStates, state::State, utils::Number};

impl State {
    pub fn process_action(&mut self, action: CardActions, target: Option<EnemyIndex>) {
        match action {
            CardActions::Damage((amt, target_type)) => todo!(),
            CardActions::ApplyVulnerable((amt, target_type)) => todo!(),
            CardActions::ApplyWeak((amt, target_type)) => todo!(),
            CardActions::Block(mut amt) => {
                amt += self.get_combat().self_effects.get_dexterity();
                if self.get_combat().self_effects.is_frail() {
                    let amt_f = 0.75 * amt.0 as f32;
                    amt = Number(amt_f.floor() as i16);
                }
                self.get_combat().self_block += amt;
            }
            CardActions::Draw(amt) => self.get_combat().draw(amt),
        }
        // After every action is fully resolved, check if the combat is finished
        self.maybe_end_combat();
    }

    pub fn play_card(&mut self, card_index: CardIndex, target: Option<EnemyIndex>) {
        // Find the card
        let mut card = self.get_combat().hand.remove(card_index.0);
        // If the card costs too much, it cannot be played
        let cost = card.card().get_cost();
        assert!(cost <= self.get_combat().current_energy);
        // Lose that amount of energy
        self.get_combat().current_energy -= cost;
        // Apply every card action in order
        let actions = card.card_mut().play();
        for action in actions {
            self.process_action(action, target);
        }
        // Then if the card exhausts, move it to exhaust pile
        // Otherwise, move it to the discard
        if card.card().exhausts() {
            self.get_combat().exhaust.push(card);
        } else {
            self.get_combat().discard.push(card);
        }
    }
}
