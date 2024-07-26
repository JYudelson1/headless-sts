use crate::{
    enemies::{ConcreteEnemy, EnemyIndex, EnemyIntent},
    state::State,
    utils::Number,
};

use super::{combat_fns::calculate_damage, Combat};

impl Combat {
    pub fn cycle_enemy_intents(&mut self) {
        for enemy in &mut self.enemies {
            enemy.next_intent();
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

impl State {
    fn enemy_action(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.get_combat().enemies[enemy_index.0];
        let action = enemy.get_intent();

        match action {
            EnemyIntent::Damage(amt) => self.enemy_attack(amt, enemy_index),
            EnemyIntent::Block(amt) => enemy.block_intent(amt),
            EnemyIntent::Buff(buff) => enemy.effects.apply_buff(buff),
            EnemyIntent::Stun => (),
            EnemyIntent::Sleep => (),
            EnemyIntent::AttackAndBlock(attack, block) => {
                self.enemy_attack(attack, enemy_index);
                let enemy = &mut self.get_combat().enemies[enemy_index.0];
                enemy.block_intent(block);
            }
            EnemyIntent::BuffAndBlock(buff, block) => {
                enemy.block_intent(block);
                enemy.effects.apply_buff(buff);
            }
            EnemyIntent::MultiAttack((amt, times)) => {
                for _ in 0..times {
                    // If the enemy dies partway through, stop attacking
                    if !self.get_combat().enemies[enemy_index.0].is_dead() {
                        self.enemy_attack(amt, enemy_index);
                    }
                    
                }
            },
        }
    }

    pub fn enemy_actions(&mut self) {
        for i in 0..self.get_combat().num_enemies() {
            self.enemy_action(EnemyIndex(i));
        }
    }

    pub fn enemy_attack(&mut self, damage_intent: Number, enemy_index: EnemyIndex) {
        let self_effects = &self.get_combat().self_effects.clone();
        let enemy = &mut self.get_combat().enemies[enemy_index.0];

        // TODO: Check for thorns on self

        let real_damage = calculate_damage(&enemy.effects, self_effects, damage_intent);

        self.damage_self(real_damage);
    }
}
