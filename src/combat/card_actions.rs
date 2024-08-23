use crate::{cards::{CardActions, CardIndex, CardType, MasterCard, Pile, Targets}, effects::{Debuff, DurationDebuffs}, enemies::EnemyIndex, state::State, utils::{number_between, NotImplemented, Number}};

impl State {
    pub fn process_action(
        &mut self,
        action: CardActions,
        target: Option<EnemyIndex>,
    ) -> Result<(), NotImplemented> {
        match action {
            CardActions::Damage((amt, target_type)) => {
                self.damage_enemy(amt, target_type, target)?;
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
            CardActions::UpgradeACardInHand => Err(NotImplemented::ChoosingFromHand)?,
            CardActions::UpgradeAllCardsInHand => {
                for card in &mut self.get_combat().hand {
                    // Upgrade the inner card without upgrading the MasterCard
                    card.card_mut().upgrade();
                }
            },
            CardActions::BodySlam => {
                let damage_amt = self.get_combat().self_block;
                self.damage_enemy(damage_amt, Targets::One, target)?;
            },
            CardActions::GainTempStrength(_) => todo!(),
            CardActions::ExhaustRandomCard => {
                if self.get_combat().hand.len() == 0 { return Ok(())}
                let i = number_between(0, self.get_combat().hand.len() - 1);
                let card = self.get_combat().hand.remove(i);
                self.get_combat().exhaust_card(card);
            },
            CardActions::ExhaustSelectedCard => Err(NotImplemented::ChoosingFromHand)?,
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
                let index = number_between(0, pile.len());
                pile.insert(index, card);
            },
            CardActions::ApplyDebuff(debuff) => {
                // TODO: Are there relics or powers that interact here?
                let relics = self.relics.clone();
                self.get_combat().self_effects.apply_debuff(debuff, &relics);
            },
            CardActions::IncreaseMaxEnergy => {
                self.get_combat().max_energy += 1
            },
            CardActions::DoubleBlock => {
                let block = self.get_combat().self_block;
                self.get_combat().gain_block(block)
            },
            CardActions::AddCardToHand(card) => {
                self.get_combat().create_card_in_hand(card);
            },
            CardActions::AddFreshCardToHand((card, upgraded)) => self.get_combat().create_fresh_card_in_hand(card, upgraded)?,
            CardActions::Havoc => {
                let combat = self.get_combat();
                // Cannot havoc if all cards are in hand
                if combat.deck.is_empty() && combat.discard.is_empty() {
                    return Ok(());
                }
                // If draw pile is empty, reshuffle
                if combat.deck.is_empty() {
                    combat.reshuffle()
                }
                // Take the top card
                let mut card = combat.deck.remove(0);
                // Play it
                self.play_card_effects(&mut card, None)?;
                if !self.is_in_combat() {
                    return Ok(());
                }
                // Exhaust the card
                let combat = self.get_combat();
                combat.exhaust_card(card);
            },
            CardActions::PerfectedStrike(amt) => {
                let mut damage = Number(6);

                for card in &self.get_combat().hand {
                    if card.card().is_a_strike() {
                        damage += amt;
                    }
                }
                for card in &self.get_combat().discard {
                    if card.card().is_a_strike() {
                        damage += amt;
                    }
                }
                for card in &self.get_combat().deck {
                    if card.card().is_a_strike() {
                        damage += amt;
                    }
                }

                self.damage_enemy(damage, Targets::One, target)?;
            },
        }

        Ok(())
    }

    pub fn play_card_from_hand(
        &mut self,
        card_index: CardIndex,
        target: Option<EnemyIndex>,
    ) -> Result<(), NotImplemented> {
        //println!("state of combat: {:#?}", self.get_combat());
        // Find the card
        let mut card = self.get_combat().hand.remove(card_index.0);
        // DEBUG
        //println!("Playing {:?}", card.card().name());
        // If the card costs too much, it cannot be played
        let cost = card.card().get_cost();
        assert!(cost <= self.get_combat().current_energy);
        // Lose that amount of energy
        self.get_combat().current_energy -= cost;

        // Actually play the card
        self.play_card_effects(&mut card, target)?;

        // Then if the card exhausts, move it to exhaust pile
        // Otherwise, move it to the discard
        if !self.is_in_combat() {
            return Ok(());
        }
        if card.card().exhausts() {
            self.get_combat().exhaust_card(card);
        } else {
            self.get_combat().discard.push(card);
        }

        Ok(())
    }

    fn play_card_effects(
        &mut self,
        card: &mut MasterCard,
        target: Option<EnemyIndex>,
    ) -> Result<(), NotImplemented> {
        // Apply every card action in order
        let actions = card.card_mut().play();
        for action in actions {
            self.process_action(action, target)?;
            // Stop early if the combat finished
            if !self.is_in_combat() {
                return Ok(());
            }
        }
        //// TODO: Apply card double-play effects
        // TODO: Echo form
        // TODO: Necronomicon

        //// TODO: Relic effects
        // TODO: Art of war
        // TODO: Shuriken
        // TODO: Kunai
        // TODO: Ink Bottle
        // TODO: Pocketwatch
        // TODO: Pen Nib
        // TODO: Others???

        //// Power effects
        // Rage
        if let Some(rage) = self.get_combat().self_effects.rage() {
            if card.card().get_type() == CardType::Attack {
                self.get_combat().gain_block(rage);
            }
        }

        Ok(())
    }
}
