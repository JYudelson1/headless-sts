use crate::{cards::CardActions, enemies::EnemyIndex, state::State};

impl State {
    pub fn process_action(&mut self, action: CardActions, target: Option<EnemyIndex>) {
        match action {
            CardActions::Damage((amt, target_type)) => todo!(),
            CardActions::ApplyVulnerable((amt, target_type)) => todo!(),
            CardActions::ApplyWeak((amt, target_type)) => todo!(),
            CardActions::Block(amt) => todo!(),
            CardActions::Draw(amt) => todo!(),
        }
    }
}
