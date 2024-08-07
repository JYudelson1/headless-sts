use crate::{cardrewardrng::CombatType, enemies::EnemyType, utils::Act};

pub fn get_enemies(act: &Act, floor: u8, combat_type: CombatType) -> Vec<EnemyType> {
    // TODO: Is there something weird about elite enemies? Like they dont happen twice?
    // TODO: Easy pools
    vec![EnemyType::JawWorm]
    //todo!()
}
