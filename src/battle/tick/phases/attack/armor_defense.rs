use crate::{
    enums::{ArmorType, StartingDirection, WeaponType},
    util::{push_stat_armor, WEAPON_ARMOR_CELL},
};
use rand::Rng;

pub fn try_armor_defense(
    armor: ArmorType,
    weapon: WeaponType,
    starting_direction: StartingDirection,
) -> bool {
    let weapon_armor_map = WEAPON_ARMOR_CELL.get().unwrap();
    let weapon_armor_combo = weapon.to_string() + "-" + armor.to_string().as_str();

    let chance_to_hit_option = weapon_armor_map.get(weapon_armor_combo.as_str());

    if let Some(hit_float) = chance_to_hit_option {
        let random_defense_num = rand::thread_rng().gen_range(0..100);
        if random_defense_num < (*hit_float * 100.0).round() as i64 {
            // Successful hit, unsuccessful armor defense
            return false;
        } else {
            push_stat_armor(starting_direction);
            return true;
        }

        return false;
    } else {
        panic!("WeaponType-ArmorType not supported! {weapon_armor_combo}")
    }
}
