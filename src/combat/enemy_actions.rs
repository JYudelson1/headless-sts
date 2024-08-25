use crate::{
    cards::{make_card, Pile},
    enemies::{ConcreteEnemy, EnemyIndex, EnemyIntent},
    state::State,
    utils::{number_between, Number},
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

        self.apply_enemy_action(action, enemy_index);
    }

    fn apply_enemy_action(&mut self, action: EnemyIntent, enemy_index: EnemyIndex) {
        let enemy = &mut self.get_combat().enemies[enemy_index.0];
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
            }
            EnemyIntent::ShuffleCardToPile(card, pile, upgraded) => {
                let pile = match pile {
                    Pile::Draw => &mut self.get_combat().deck,
                    Pile::Discard => &mut self.get_combat().discard,
                };
                let card = make_card(card, upgraded).expect("Enemies can only insert status cards, which have already been implemented");
                let index = number_between(0, pile.len());
                pile.insert(index, card);
            }
            EnemyIntent::Multiple(intents) => {
                for intent in intents {
                    self.apply_enemy_action(intent, enemy_index);
                }
            }
            EnemyIntent::Debuff(debuff) => {
                let relics = &self.relics.clone();
                self.get_combat().self_effects.apply_debuff(debuff, relics);
            },
        }
    } 

    pub fn enemy_actions(&mut self) {
        for i in 0..self.get_combat().num_enemies() {
            self.enemy_action(EnemyIndex(i));
        }
    }

    pub fn enemy_attack(&mut self, damage_intent: Number, enemy_index: EnemyIndex) {
        let relics = &self.relics.clone();
        let self_effects = &self.get_combat().self_effects.clone();
        let enemy = &mut self.get_combat().enemies[enemy_index.0];

        let real_damage = calculate_damage(&enemy.effects, self_effects, damage_intent, relics);

        self.attack_damage_self(real_damage);

        // Check for thorns on self
        if let Some(thorns) = self_effects.thorns() {
            self.direct_damage_enemy(enemy_index, thorns.0 as u16)
                .unwrap();
        }
    }
}
