use crate::{cards::CardActions, enemies::EnemyIndex, screens::VisibleStates, state::State, utils::Number};

impl State {
    pub fn process_action(&mut self, action: CardActions, target: Option<EnemyIndex>) {
        if let VisibleStates::Combat(combat) = &mut self.visible_screen {
            match action {
                CardActions::Damage((amt, target_type)) => todo!(),
                CardActions::ApplyVulnerable((amt, target_type)) => todo!(),
                CardActions::ApplyWeak((amt, target_type)) => todo!(),
                CardActions::Block(mut amt) => {
                    amt += combat.self_effects.get_dexterity();
                    if combat.self_effects.is_frail() {
                        let amt_f = 0.75 * amt.0 as f32;
                        amt = Number(amt_f.floor() as i16);
                    }
                    combat.self_block += amt;
                },
                CardActions::Draw(amt) => todo!(),
            }
            // After every action is fully resolved, check if the combat is finished
            self.maybe_end_combat();
        } else {
            panic!("Should not process card actions out of combat!");
        }
    }
}
