use std::{collections::HashMap, env, ops::Deref, thread};

use crate::{
    types::{ArmorType, ArmyName, Battalion, Battle, Belligerent, StartingDirection, WeaponType},
    util::{push_log, push_stat_armor, push_stat_block, push_stat_dodge, WEAPON_ARMOR_CELL},
    IS_MARCHING_AGILITY_MOD,
};
use rand::Rng;
use std::string::ToString;

pub fn attack_phase_new(
    attacker_map_east: &HashMap<ArmyName, Vec<ArmyName>>,
    attacker_map_west: &HashMap<ArmyName, Vec<ArmyName>>,
    battle_state: &Battle,
) {
    let closure = |attacker_map: &HashMap<ArmyName, Vec<ArmyName>>| {
        let mut x = 0u128;

        for i in 1..100 {
            x += i;
        }
        x
    };

    let result = thread::scope(|scope| {
        let east_first = scope.spawn(|| closure(attacker_map_east));
        let west_first = scope.spawn(|| closure(attacker_map_west));

        loop {
            if east_first.is_finished() && west_first.is_finished() {
                return (east_first.join().unwrap(), west_first.join().unwrap());
            }
        }
    });

    // Since we'll have results for two possible scenarios, where east attacks first AND west attacks first
    // We can take the result and average the two for our final result

}
