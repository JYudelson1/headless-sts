use crate::relics::Relics;

#[derive(Debug)]
pub struct QuestionMarkRng {
    fight_chance: f32,
    treasure_chance: f32,
    shop_chance: f32,
}

impl QuestionMarkRng {
    // TODO: Reset at beginning of each act
    pub fn new() -> Self {
        Self {
            fight_chance: 0.1,
            treasure_chance: 0.02,
            shop_chance: 0.03,
        }
    }

    fn update_probs(&mut self, encountered: QuestionMark) {
        match encountered {
            QuestionMark::NormalFight => {
                self.treasure_chance += 0.02;
                self.shop_chance += 0.03;
                self.fight_chance = 0.10;
            }
            QuestionMark::TreasureRoom => {
                self.treasure_chance = 0.02;
                self.shop_chance += 0.03;
                self.fight_chance += 0.10;
            }
            QuestionMark::Shop => {
                self.treasure_chance += 0.02;
                self.shop_chance = 0.03;
                self.fight_chance += 0.10;
            }
            QuestionMark::Event => (),
        }
    }

    pub fn get_question_mark(&mut self, relics: &mut Relics) -> QuestionMark {
        let tiny_chest_trigger = relics.tiny_chest_is_on();

        let outcome = if tiny_chest_trigger {
            QuestionMark::TreasureRoom
        } else {
            // Increase tiny chest count if it exists
            relics.increase_tiny_chest();

            // Juzu turns normal fights into events
            self.fight_chance = 0.0;

            let x = rand::random::<f32>();
            if x < self.fight_chance {
                QuestionMark::NormalFight
            } else if x < self.fight_chance + self.treasure_chance {
                QuestionMark::TreasureRoom
            } else if x < self.fight_chance + self.treasure_chance + self.shop_chance {
                QuestionMark::Shop
            } else {
                QuestionMark::Event
            }
        };
        self.update_probs(outcome);

        outcome
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum QuestionMark {
    NormalFight,
    TreasureRoom,
    Shop,
    Event,
}
