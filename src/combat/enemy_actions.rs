use crate::{
    enemies::{ConcreteEnemy, EnemyIndex, EnemyIntent},
    utils::Number,
};

use super::Combat;

impl Combat {
    pub fn apply_enemy_action(&mut self, enemy_index: EnemyIndex, action: EnemyIntent) {
        let enemy = &mut self.enemies[enemy_index.0];

        match action {
            EnemyIntent::Damage(_) => todo!(),
            EnemyIntent::Block(amt) => enemy.block_intent(amt),
            EnemyIntent::Buff(buff) => enemy.effects.apply_buff(buff),
            EnemyIntent::Stun => (),
            EnemyIntent::Sleep => (),
            EnemyIntent::AttackAndBlock(_, _) => todo!(),
            EnemyIntent::BuffAndBlock(buff, block) => {
                enemy.block_intent(block);
                enemy.effects.apply_buff(buff);
            }
            EnemyIntent::MultiAttack(_) => todo!(),
        }
    }
}

impl ConcreteEnemy {
    pub fn block_intent(&mut self, mut amt: Number) {
        // Affected by dexterity ONLY (TODO: is that correct?)
        amt += self.effects.get_dexterity();
        self.current_block += amt;
    }
}
