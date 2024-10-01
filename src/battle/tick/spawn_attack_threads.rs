use std::{collections::HashMap, thread};

use crate::types::{ArmyName, Battalion, Battle};

use super::phases::attack_new::attack_phase_new;

const threads_per_army: u8 = 1;

pub fn spawn_attack_threads(
    attacker_map_east: &HashMap<ArmyName, Vec<ArmyName>>,
    attacker_map_west: &HashMap<ArmyName, Vec<ArmyName>>,
    battle_state: &Battle,
) {
    let Battle {
        army_1_state,
        army_2_state,
    } = battle_state;

    thread::scope(|scope| {
        for n in 1..threads_per_army {
            let west_thread_num = n * 2;
            let east_thread_num = n * 2 - 1;

            // EAST attacks
            scope.spawn(|| {
                attack_phase_new(
                    attacker_map_east,
                    army_1_state,
                    army_2_state,
                    east_thread_num,
                )
            });
            // WEST attacks
            scope.spawn(|| {
                attack_phase_new(
                    attacker_map_west,
                    army_2_state,
                    army_1_state,
                    west_thread_num,
                )
            });
        }
    });
}