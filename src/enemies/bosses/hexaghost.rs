use crate::{
    cards::{CardName, Pile},
    effects::{Buff, Effects, IntensityBuffOrDebuff},
    utils::Number,
};

use super::super::{enemy_trait::Enemy, ConcreteEnemy, EnemyIntent, EnemyType};

#[derive(Debug, Clone)]
pub struct Hexaghost {
    intent: HexaghostAttacks,
    turn_num: usize,
    player_hp: u16,
}

impl Enemy for Hexaghost {
    fn next_intent(&mut self, ascension: u8) {
        self.turn_num += 1;
        self.intent = HexaghostAttacks::next_attack(self.turn_num, ascension, self.player_hp);
    }

    fn get_current_intent(&self) -> EnemyIntent {
        self.intent.to_intent()
    }

    fn duplicate(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
enum HexaghostAttacks {
    Activate,
    Divider(u16),
    Inferno(Number),
    Sear((usize, bool)),
    Tackle(Number),
    Inflame(Number),
}

impl HexaghostAttacks {
    fn to_intent(&self) -> EnemyIntent {
        match self {
            HexaghostAttacks::Activate => EnemyIntent::Sleep,
            HexaghostAttacks::Divider(player_hp) => {
                let dmg = Number((*player_hp as f32 / 12.0).floor() as i16 + 1);
                EnemyIntent::MultiAttack((dmg, 6))
            }
            HexaghostAttacks::Inferno(dmg) => {
                let mut actions = vec![];

                actions.push(EnemyIntent::MultiAttack((*dmg, 6)));
                for _ in 0..3 {
                    let action =
                        EnemyIntent::ShuffleCardToPile(CardName::Burn, Pile::Discard, true);
                    actions.push(action);
                }
                actions.push(EnemyIntent::UpgradeAllBurns);

                EnemyIntent::Multiple(actions)
            }
            HexaghostAttacks::Sear((amt, upgraded)) => {
                let mut actions = vec![];

                actions.push(EnemyIntent::Damage(Number(6)));
                for _ in 0..*amt {
                    let action =
                        EnemyIntent::ShuffleCardToPile(CardName::Burn, Pile::Discard, *upgraded);
                    actions.push(action);
                }

                EnemyIntent::Multiple(actions)
            }
            HexaghostAttacks::Tackle(damage) => EnemyIntent::MultiAttack((*damage, 2)),
            HexaghostAttacks::Inflame(strength) => EnemyIntent::BuffAndBlock(
                Buff::Basic((IntensityBuffOrDebuff::Strength, *strength)),
                Number(12),
            ),
        }
    }

    fn tackle(ascension: u8) -> Self {
        if ascension >= 4 {
            Self::Tackle(Number(6))
        } else {
            Self::Tackle(Number(5))
        }
    }

    fn sear(ascension: u8, turn_num: usize) -> Self {
        if ascension >= 19 {
            Self::Sear((2, HexaghostAttacks::burn_is_upgraded(turn_num)))
        } else {
            Self::Sear((1, HexaghostAttacks::burn_is_upgraded(turn_num)))
        }
    }

    fn divider(player_hp: u16) -> Self {
        Self::Divider(player_hp)
    }

    fn inflame(ascension: u8) -> Self {
        if ascension >= 19 {
            Self::Inflame(Number(3))
        } else {
            Self::Inflame(Number(2))
        }
    }

    fn inferno(ascension: u8) -> Self {
        if ascension >= 4 {
            Self::Inferno(Number(3))
        } else {
            Self::Inferno(Number(2))
        }
    }

    fn burn_is_upgraded(turn_num: usize) -> bool {
        if turn_num >= 9 {
            true
        } else {
            false
        }
    }

    pub fn next_attack(turn_num: usize, ascension: u8, player_hp: u16) -> Self {
        if turn_num == 1 {
            return Self::divider(player_hp);
        }

        let true_turn = (turn_num - 2) % 6;

        match true_turn {
            0 => Self::sear(ascension, turn_num),
            1 => Self::tackle(ascension),
            2 => Self::sear(ascension, turn_num),
            3 => Self::inflame(ascension),
            4 => Self::sear(ascension, turn_num),
            5 => Self::inferno(ascension),
            _ => unreachable!(),
        }
    }
}

impl Hexaghost {
    pub fn new(ascension: u8, player_hp: u16) -> ConcreteEnemy {
        let hp = if ascension >= 9 { 264 } else { 250 };

        let ghost = Hexaghost {
            intent: HexaghostAttacks::Activate,
            turn_num: 0,
            player_hp,
        };

        ConcreteEnemy {
            effects: Effects::new(),
            max_hp: hp,
            current_hp: hp,
            current_block: Number(0),
            inner: Box::new(ghost),
            enemy_type: EnemyType::Hexaghost,
            ascension: ascension,
        }
    }
}
