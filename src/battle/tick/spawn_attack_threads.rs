use std::{
    collections::HashMap,
    thread::{self, ScopedJoinHandle},
};

use crate::{entities::battle::battle::Battle, enums::ArmyName};

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

    /*
     * TICKS ARE RUNNING AS EXPECTED IN A LOOP
     */

    thread::scope(|scope| {
        for n in 0..threads_per_army {
            let index = n + 1;
            let west_thread_num = index * 2;
            let east_thread_num = index * 2 - 1;

            scope.spawn(move || {
                // EAST attacks
                attack_phase_new(
                    attacker_map_east,
                    army_1_state,
                    army_2_state,
                    east_thread_num,
                );

                // WEST attacks
                scope.spawn(move || {
                    attack_phase_new(
                        attacker_map_west,
                        army_2_state,
                        army_1_state,
                        west_thread_num,
                    )
                });
            });
        }
    });
}
