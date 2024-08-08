use crate::{
    cards::{card::CardType, card_actions::CardActions, card_trait::Card, CardName},
    effects::{Buff, Debuff, OneTurnBoolDebuffs, PermanentBoolBuffs},
};

#[derive(Debug)]
pub struct BattleTrance(pub bool);

impl Card for BattleTrance {
    fn name(&self) -> CardName {
        CardName::BattleTrance
    }

    fn get_type(&self) -> CardType {
        CardType::Skill
    }

    fn targets(&self) -> bool {
        false
    }

    fn set_upgraded(&mut self, to_set: bool) {
        self.0 = to_set;
    }

    fn can_be_upgraded(&self) -> bool {
        !self.0
    }

    fn is_upgraded(&self) -> bool {
        self.0
    }

    fn play_upgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Draw(4),
            CardActions::ApplyDebuff(Debuff::OneTurnBool(OneTurnBoolDebuffs::NoCardDraw)),
        ]
    }

    fn play_unupgraded(&mut self) -> Vec<CardActions> {
        vec![
            CardActions::Draw(3),
            CardActions::ApplyDebuff(Debuff::OneTurnBool(OneTurnBoolDebuffs::NoCardDraw)),
        ]
    }

    fn get_cost(&self) -> u8 {
        0
    }
}
