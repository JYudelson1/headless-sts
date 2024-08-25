use std::ops::{Add, AddAssign};

use crate::{
    cards::{make_card, CardName, CardType, MasterCard, Targets},
    effects::{Debuff, DurationDebuffs, Effects, OneTurnBoolDebuffs, PermanentBoolBuffs},
    enemies::EnemyIndex,
    relics::{Relic, Relics},
    screens::VisibleStates,
    state::State,
    utils::{number_between, NotImplemented, Number, StillPlaying},
};

use super::Combat;
use rand::seq::SliceRandom;

impl State {
    pub fn heal(&mut self, mut amt: u16) {
        if matches!(self.visible_screen, VisibleStates::Combat(_))
            && self.relics.contains(Relic::MagicFlower)
        {
            amt = (amt as f32 * 1.5).ceil() as u16;
        }
        if self.relics.contains(Relic::MarkOfTheBloom) {
            amt = 0;
        }

        self.current_health += amt;
        if self.current_health > self.max_health.0 as u16 {
            self.current_health = self.max_health.0 as u16
        }
    }

    pub fn increase_max_hp(&mut self, amt: u16) {
        self.max_health += Number(amt as i16);
        self.heal(amt);
    }

    pub fn lose_hp(&mut self, mut amt: u16) {
        if amt == 0 {
            return;
        }
        // If you are in combat & have intangible, all hp loss goes to 1
        if let VisibleStates::Combat(combat) = &self.visible_screen {
            if combat.self_effects.is_intangible() && amt >= 1 {
                amt = 1;
            }
            // TODO: Self-forming clay
            // TODO: Centennial puzzle
        }

        // Tungsten rod reduces all hp loss by one
        if self.relics.contains(Relic::TungstenRod) && amt >= 1 {
            amt -= 1;
            if amt == 0 {
                return;
            }
        }

        if amt >= self.current_health {
            // Player would die
            // TODO: Check for lizard tail
            // TODO: Check for fairy in a bottle
            // TODO: Show that you lose
            //println!("Player is dead!");
            self.still_playing = StillPlaying::Dead(self.map.current_floor());
            // if self.map.current_floor() > 10 {
            //     println!("Made it to floor {}", self.map.current_floor());
            //     println!("Deck: {:?}", self.main_deck);
            //     println!("relics: {:?}", self.relics.list);
            //     println!("{}", self.map);
            // }
        } else {
            self.current_health -= amt;
        }
    }

    fn debuff_one_enemy(&mut self, debuff: Debuff, enemy_index: EnemyIndex) {
        let relics = &self.relics.clone();
        let combat = self.get_combat();
        let enemy = &mut combat.enemies[enemy_index.0];

        enemy.effects.apply_debuff(debuff, relics);
    }

    pub fn debuff_enemy(
        &mut self,
        debuff: Debuff,
        mut target_type: Targets,
        enemy_index: Option<EnemyIndex>,
    ) {
        // TODO: Effects that alter debuffs
        // TODO: Champion's belt

        if target_type == Targets::One && enemy_index.is_none() {
            // Presumably this is a targeted card randomly played
            target_type = Targets::Random;
        }

        let num_enemies = self.get_combat().enemies.len();
        match target_type {
            Targets::All => {
                for i in 0..num_enemies {
                    self.debuff_one_enemy(debuff, EnemyIndex(i));
                }
            }
            Targets::One => self.debuff_one_enemy(debuff, enemy_index.unwrap()),
            Targets::Random => {
                self.debuff_one_enemy(debuff, EnemyIndex(number_between(0, num_enemies - 1)))
            }
        }
    }
}

pub fn calculate_damage(
    source_effects: &Effects,
    target_effects: &Effects,
    damage: Number,
    relics: &Relics,
) -> Number {
    let mut damage = damage.0 as f32;
    // Factor in strength
    damage += source_effects.get_strength().0 as f32;
    // Factor in vulnerability
    if target_effects.is_vulnerable() {
        match relics.contains(Relic::PaperPhrog) {
            true => damage *= 1.75,
            false => damage *= 1.5,
        }
    }
    // Factor in weakness
    if source_effects.is_weak() {
        match relics.contains(Relic::PaperKrane) {
            true => damage *= 0.6,
            false => damage *= 0.75,
        }
    }

    if damage < 0.0 {
        damage = 0.0;
    }

    Number(damage.floor() as i16)
}

impl Combat {
    pub fn reshuffle(&mut self) {
        self.deck.append(&mut self.discard);
        self.discard = vec![];
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn gain_block(&mut self, amt: Number) {
        self.self_block += amt;
        // TODO: Block effects
        // TODO: Juggernaut
    }

    fn enemy_loses_block(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.enemies[enemy_index.0];
        if enemy
            .effects
            .permanent_bool_buffs
            .contains(&PermanentBoolBuffs::Barricade)
        {
            return;
        }
        enemy.current_block = Number(0)
    }

    pub fn enemies_lose_block(&mut self) {
        for i in 0..self.num_enemies() {
            self.enemy_loses_block(EnemyIndex(i));
        }
    }

    fn enemy_end_of_turn(&mut self, enemy_index: EnemyIndex) {
        let enemy = &mut self.enemies[enemy_index.0];
        // Metallicize
        let metal = enemy.effects.get_metallicize();
        if let Some(amt) = metal {
            enemy.current_block += amt;
        }
    }

    pub fn end_enemies_turn(&mut self) {
        for i in 0..self.num_enemies() {
            self.enemy_end_of_turn(EnemyIndex(i))
        }
    }

    pub fn create_card_in_hand(&mut self, card: MasterCard) {
        if self.hand.len() == 10 {
            self.discard.push(card)
        } else {
            self.hand.push(card)
        }
    }

    pub fn create_fresh_card_in_hand(
        &mut self,
        card: CardName,
        upgraded: bool,
    ) -> Result<(), NotImplemented> {
        let card = make_card(card, upgraded)?;
        self.create_card_in_hand(card);
        Ok(())
    }

    pub fn block_goes_away(&mut self, relics: &Relics) {
        // Barricade
        if self
            .self_effects
            .permanent_bool_buffs
            .contains(&PermanentBoolBuffs::Barricade)
        {
            return;
        }

        // Calipers
        let has_calipers = relics.contains(Relic::Calipers);
        let block = &mut self.self_block;
        if has_calipers && block.0 > 15 {
            *block -= Number(15)
        } else {
            *block = Number(0)
        }
    }

    fn draw_1(&mut self, relics: &Relics) -> Result<CombatOver, NotImplemented> {
        // Cannot draw if all cards are in hand
        if self.deck.is_empty() && self.discard.is_empty() {
            return Ok(CombatOver::No);
        }
        // Cannot draw if hand is full
        if self.hand.len() >= 10 {
            return Ok(CombatOver::No);
        }
        // If draw pile is empty, reshuffle
        if self.deck.is_empty() {
            self.reshuffle()
        }
        // Take the top card from deck and move to hand
        let top_card = self.deck.remove(0);
        let name = top_card.card().name();
        let card_type = top_card.card().get_type();
        self.hand.push(top_card);

        // On Draw Effects
        // TODO: Deus Ex
        // Void
        if name == CardName::Void {
            if self.current_energy > 0 {
                self.current_energy -= 1;
            }
        }
        // Firebreathing
        if let Some(firebreathing) = self.self_effects.firebreathing() {
            if card_type == CardType::Status || card_type == CardType::Curse {
                let maybe_over = self.direct_damage_all_enemies(firebreathing.0 as u16, relics)?;
                if maybe_over == CombatOver::Yes {
                    return Ok(CombatOver::Yes);
                }
            }
        }

        // Evolve
        if let Some(evolve_amt) = self.self_effects.evolve() {
            if card_type == CardType::Status {
                let combat_over = self.draw(evolve_amt.0 as u8, relics)?;
                if combat_over == CombatOver::Yes {return Ok(CombatOver::Yes);}
            }
        }

        Ok(CombatOver::No)
    }

    pub fn draw(&mut self, amt: u8, relics: &Relics) -> Result<CombatOver, NotImplemented> {
        // Don't draw if no card draw
        if self
            .self_effects
            .one_turn_bool_debuffs
            .contains(&OneTurnBoolDebuffs::NoCardDraw)
        {
            return Ok(CombatOver::No);
        }

        for _ in 0..amt {
            let over = self.draw_1(relics)?;
            if over == CombatOver::Yes {
                return Ok(CombatOver::Yes);
            }
        }
        Ok(CombatOver::No)
    }

    pub fn exhaust_card(
        &mut self,
        card: MasterCard,
        relics: &Relics,
    ) -> Result<CombatOver, NotImplemented> {
        // Feel no pain
        if let Some(amt) = self.self_effects.get_feel_no_pain() {
            self.gain_block(amt);
        }
        // Dark embrace
        if let Some(amt) = self.self_effects.get_dark_embrace() {
            let combat_over = self.draw(amt.0 as u8, relics)?;
            if combat_over == CombatOver::Yes {return Ok(CombatOver::Yes);}
        }
        // TODO: Necronomicurse goes here
        self.exhaust.push(card);

        Ok(CombatOver::No)
    }

    pub fn enemy_lose_hp(
        &mut self,
        enemy_index: EnemyIndex,
        mut amt: u16,
        relics: &Relics,
    ) -> Result<CombatOver, NotImplemented> {
        let has_the_boot = relics.contains(Relic::TheBoot);
        let enemy = &mut self.enemies[enemy_index.0];
        if enemy.effects.is_intangible() {
            amt = 1;
        }
        if has_the_boot && amt < 5 {
            amt = 5;
        }

        if amt <= enemy.current_hp {
            enemy.current_hp -= amt;
            // Tell the enemy that it lost HP
            enemy.lost_hp();
        } else {
            enemy.current_hp = 0;
            // Activate spore cloud (fungi beast)
            if let Some(spore_cloud) = enemy.effects.spore_cloud() {
                self.self_effects.apply_debuff(
                    Debuff::Duration((DurationDebuffs::Vulnerable, spore_cloud)),
                    relics,
                );
            }
        }
        Ok(self.check_if_over())
    }

    fn begin_enemy_turn_effects(
        &mut self,
        enemy_index: EnemyIndex,
        relics: &Relics,
    ) -> Result<CombatOver, NotImplemented> {
        let enemy = &mut self.enemies[enemy_index.0];
        // Poison
        let poison = enemy.effects.get_poison();
        if poison.0 > 0 {
            match self.enemy_lose_hp(enemy_index, poison.0 as u16, relics) {
                Ok(combat_over) => {
                    if combat_over == CombatOver::Yes {
                        return Ok(CombatOver::Yes);
                    }
                }
                Err(err) => Err(err)?,
            }
        }
        // Increment the enemies effects
        let enemy = &mut self.enemies[enemy_index.0];
        enemy.effects.increment_turn();
        Ok(CombatOver::No)
    }

    pub fn begin_enemy_turn(&mut self, relics: &Relics) -> Result<CombatOver, NotImplemented> {
        let num_enemies = self.num_enemies();
        for i in 0..num_enemies {
            match self.begin_enemy_turn_effects(EnemyIndex(i), relics) {
                Ok(over) => {
                    if over == CombatOver::Yes {
                        return Ok(CombatOver::Yes);
                    }
                }
                Err(err) => Err(err)?,
            }
        }
        Ok(CombatOver::No)
    }

    pub fn direct_damage_enemy(
        &mut self,
        enemy_index: EnemyIndex,
        mut amt: u16,
        relics: &Relics,
    ) -> Result<(DamagedEnemy, CombatOver), NotImplemented> {
        let enemy = &mut self.enemies[enemy_index.0];

        if amt < enemy.current_block.0 as u16 {
            enemy.current_block -= Number(amt as i16);
            return Ok((DamagedEnemy::No, CombatOver::No));
        } else {
            amt -= enemy.current_block.0 as u16;
            enemy.current_block = Number(0);
            let combat_over = self.enemy_lose_hp(enemy_index, amt, relics)?;
            return Ok((DamagedEnemy::Yes, combat_over));
        }
    }

    pub fn direct_damage_all_enemies(
        &mut self,
        amt: u16,
        relics: &Relics,
    ) -> Result<CombatOver, NotImplemented> {
        for enemy_index in 0..self.enemies.len() {
            match self.direct_damage_enemy(EnemyIndex(enemy_index), amt, relics) {
                Ok(over) => {
                    if over.1 == CombatOver::Yes {
                        return Ok(CombatOver::Yes);
                    }
                }
                Err(err) => Err(err)?,
            }
        }

        Ok(CombatOver::No)
    }

    pub fn direct_damage_random_enemy(
        &mut self,
        amt: u16,
        relics: &Relics,
    ) -> Result<CombatOver, NotImplemented> {
        let index = number_between(0, self.enemies.len() - 1);
        match self.direct_damage_enemy(EnemyIndex(index), amt, relics) {
            Ok((_, over)) => Ok(over),
            Err(e) => Err(e),
        }
    }

    pub fn attack_damage_enemy(
        &mut self,
        enemy_index: EnemyIndex,
        amt: u16,
        relics: &Relics,
    ) -> Result<(CombatOver, HpLoss), NotImplemented> {
        let (damaged_enemy, combat_over) = self.direct_damage_enemy(enemy_index, amt, relics)?;
        let mut hp_loss = HpLoss(0);

        if combat_over == CombatOver::No && damaged_enemy == DamagedEnemy::Yes {
            // TODO: Real curl up triggers after card, not multi-attack
            let enemy = &mut self.enemies[enemy_index.0];
            if let Some(block_amt) = enemy.effects.trigger_curl_up() {
                enemy.current_block += block_amt;
            }
            // TODO: Malleable here
        }

        // Check for thorns
        let enemy = &mut self.enemies[enemy_index.0];
        if let Some(thorns) = enemy.effects.thorns() {
            hp_loss += self.damage_self(thorns);
        }

        Ok((combat_over, hp_loss))
    }

    pub fn damage_enemy(
        &mut self,
        damage_amt: Number,
        mut target_type: Targets,
        target: Option<EnemyIndex>,
        relics: &Relics,
    ) -> Result<(CombatOver, HpLoss), NotImplemented> {
        let self_effects = &self.self_effects.clone();
        let enemies = &self.enemies;
        let mut damages: Vec<(EnemyIndex, u16)> = vec![];
        let mut hp_loss = HpLoss(0);

        if target_type == Targets::One && target.is_none() {
            // Presumably this is a targeted card randomly played
            target_type = Targets::Random;
        }

        match target_type {
            Targets::All => {
                // Calculate damage and apply it for each enemy individually, in order
                for (i, enemy) in enemies.iter().enumerate() {
                    let total_damage =
                        calculate_damage(self_effects, &enemy.effects, damage_amt, relics);
                    damages.push((EnemyIndex(i), total_damage.0 as u16))
                }
            }
            Targets::One => {
                let enemy = &enemies[target.unwrap().0];
                let total_damage =
                    calculate_damage(self_effects, &enemy.effects, damage_amt, relics);
                damages.push((target.unwrap(), total_damage.0 as u16))
            }
            Targets::Random => {
                let enemy_index = number_between(0, enemies.len() - 1);
                let enemy = &enemies[enemy_index];
                let total_damage =
                    calculate_damage(self_effects, &enemy.effects, damage_amt, relics);
                damages.push((EnemyIndex(enemy_index), total_damage.0 as u16))
            }
        }

        for (enemy_index, amt) in damages {
            let (over, hp) = self.attack_damage_enemy(enemy_index, amt, relics)?;
            hp_loss += hp;
            if over == CombatOver::Yes {
                return Ok((CombatOver::Yes, hp_loss));
            }
        }

        Ok((CombatOver::No, hp_loss))
    }

    pub fn heavyblade_enemy(
        &mut self,
        strength_scale: Number,
        target: Option<EnemyIndex>,
        relics: &Relics,
    ) -> Result<(CombatOver, HpLoss), NotImplemented> {
        let enemies = &self.enemies;
        let enemy_index = match target {
            Some(target) => target.0,
            None => number_between(0, enemies.len() - 1),
        };
        let enemy = &enemies[enemy_index];

        let mut damage = 14 as f32;
        // Factor in strength
        damage += self.self_effects.get_strength().0 as f32 * strength_scale.0 as f32;
        // Factor in vulnerability
        if enemy.effects.is_vulnerable() {
            match relics.contains(Relic::PaperPhrog) {
                true => damage *= 1.75,
                false => damage *= 1.5,
            }
        }
        // Factor in weakness
        if self.self_effects.is_weak() {
            match relics.contains(Relic::PaperKrane) {
                true => damage *= 0.6,
                false => damage *= 0.75,
            }
        }

        if damage < 0.0 {
            damage = 0.0;
        }

        let damage = damage.floor() as u16;
        self.attack_damage_enemy(EnemyIndex(enemy_index), damage, relics)
    }

    pub fn discard_hand_end_of_turn(
        &mut self,
        relics: &Relics,
    ) -> Result<(CombatOver, HpLoss), NotImplemented> {
        let hand_size = self.hand.len();
        let mut hp_loss = HpLoss(0);

        for i in (0..hand_size).rev() {
            //// Hold in hand effects:
            // TODO: Watcher retain cards
            let card_name = self.hand[i].card().name().clone();
            match card_name {
                CardName::Burn => {
                    let burn_is_upgraded = self.hand[i].card().is_upgraded();
                    hp_loss += match burn_is_upgraded {
                        true => self.damage_self(Number(4)),
                        false => self.damage_self(Number(2)),
                    }
                }
                CardName::Decay => {
                    hp_loss += self.damage_self(Number(2));
                }
                CardName::Doubt => self
                    .self_effects
                    .apply_debuff(Debuff::Duration((DurationDebuffs::Weak, Number(1))), relics),
                CardName::Shame => self.self_effects.apply_debuff(
                    Debuff::Duration((DurationDebuffs::Frail, Number(1))),
                    relics,
                ),
                CardName::Regret => hp_loss += HpLoss(hand_size as u16),
                _ => {
                    // For any other card:
                    // If the card is ethereal, exhaust it
                    if self.hand[i].card().is_ethereal() {
                        let card = self.hand.remove(i);
                        let combat_over = self.exhaust_card(card, relics)?;
                        if combat_over == CombatOver::Yes {return Ok((CombatOver::Yes, hp_loss));}
                    } else if !self.hand[i].card().retains() {
                        // Else discard if not retained
                        // And if you dont have Runic Pyramid
                        if !relics.contains(Relic::RunicPyramid) {
                            self.discard.push(self.hand.remove(i));
                        }
                    }
                    continue;
                }
            }
            // Always discard burns and curses, even if they retain
            self.discard.push(self.hand.remove(i));
        }

        Ok((CombatOver::No, hp_loss))
    }

    pub fn damage_self(&mut self, mut amt: Number) -> HpLoss {
        if amt < self.self_block {
            self.self_block -= amt;
        } else {
            amt -= self.self_block;
            self.self_block = Number(0);
        }

        HpLoss(amt.0 as u16)
    }

    pub fn attack_damage_self(&mut self, amt: Number) -> HpLoss {
        // TODO: Are there specific thing to check for??
        self.damage_self(amt)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatOver {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamagedEnemy {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HpLoss(pub u16);

impl Add for HpLoss {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for HpLoss {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
