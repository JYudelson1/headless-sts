use crate::{cards::{CardActions, CardIndex, Pile, Targets}, effects::{Debuff, DurationDebuffs}, enemies::EnemyIndex, state::State, utils::{number_between, Number}};

impl State {
    pub fn process_action(&mut self, action: CardActions, target: Option<EnemyIndex>) {
        match action {
            CardActions::Damage((amt, target_type)) => {
                self.damage_enemy(amt, target_type, target);
            }
            CardActions::ApplyVulnerable((amt, target_type)) => {
                let debuff = Debuff::Duration((DurationDebuffs::Vulnerable, amt));
                self.debuff_enemy(debuff, target_type, target);
            },
            CardActions::ApplyWeak((amt, target_type)) => {
                let debuff = Debuff::Duration((DurationDebuffs::Weak, amt));
                self.debuff_enemy(debuff, target_type, target);
            },
            CardActions::Block(mut amt) => {
                amt += self.get_combat().self_effects.get_dexterity();
                if self.get_combat().self_effects.is_frail() {
                    let amt_f = 0.75 * amt.0 as f32;
                    amt = Number(amt_f.floor() as i16);
                }
                self.get_combat().self_block += amt;
            }
            CardActions::Draw(amt) => self.get_combat().draw(amt),
            CardActions::LoseHealth(amt) => self.lose_hp(amt),
            CardActions::UpgradeACardInHand => todo!(),
            CardActions::UpgradeAllCardsInHand => {
                for card in &mut self.get_combat().hand {
                    // Upgrade the inner card without upgrading the MasterCard
                    card.card_mut().upgrade();
                }
            },
            CardActions::BodySlam => {
                let damage_amt = self.get_combat().self_block;
                self.damage_enemy(damage_amt, Targets::One, target);
            },
            CardActions::GainTempStrength(_) => todo!(),
            CardActions::ExhaustRandomCard => {
                let i = number_between(0, self.get_combat().hand.len() - 1);
                let card = self.get_combat().hand.remove(i);
                self.get_combat().exhaust_card(card);
            },
            CardActions::ExhaustSelectedCard => todo!(),
            CardActions::ApplyBuff(buff) => {
                // TODO: Are there relics or powers that interact here?
                self.get_combat().self_effects.apply_buff(buff);
            },
            CardActions::GainEnergy(amt) => self.get_combat().current_energy += amt,
            CardActions::ShuffleCardToPile((card, pile)) => {
                let pile = match pile {
                    Pile::Draw => &mut self.get_combat().deck,
                    Pile::Discard => &mut self.get_combat().discard,
                };
                let index = number_between(0, pile.len() - 1);
                pile.insert(index, card);
            },
            CardActions::ApplyDebuff(debuff) => {
                // TODO: Are there relics or powers that interact here?
                self.get_combat().self_effects.apply_debuff(debuff);
            },
            CardActions::IncreaseMaxEnergy => {
                self.get_combat().max_energy += 1
            },
            CardActions::DoubleBlock => {
                let block = self.get_combat().self_block;
                self.get_combat().gain_block(block)
            },
        }
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
            // Stop early if the combat finished
            if !self.is_in_combat() {
                return;
            }
        }
        // TODO: Apply card double-play effects
        // TODO: Echo form
        // TODO: Necronomicon
        // Then if the card exhausts, move it to exhaust pile
        // Otherwise, move it to the discard
        if card.card().exhausts() {
            self.get_combat().exhaust_card(card);
        } else {
            self.get_combat().discard.push(card);
        }

        // TODO: Relic effects
        // TODO: Art of war
        // TODO: Shuriken
        // TODO: Kunai
        // TODO: Ink Bottle
        // TODO: Pocketwatch
        // TODO: Pen Nib
        // TODO: Others???
    }
}
