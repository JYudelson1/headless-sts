use crate::{
    enemies::{ConcreteEnemy, EnemyIndex, EnemyIntent},
    utils::Number,
};

use super::Combat;

impl Combat {
    fn enemy_action(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.enemies[enemy_index.0];
        let action = enemy.get_intent();

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

    pub fn cycle_enemy_intents(&mut self) {
        for enemy in &mut self.enemies {
            enemy.next_intent();
        }
    }

    pub fn enemy_actions(&mut self) {
        for i in 0..self.num_enemies() {
            self.enemy_action(EnemyIndex(i));
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
