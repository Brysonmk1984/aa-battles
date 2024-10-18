use std::{
    collections::HashMap,
    thread::{self, ScopedJoinHandle},
};

use crate::{
    entities::{battalion::battalion::Battalion, battle::battle::Battle},
    enums::{ArmyName, StartingDirection},
};

use super::phases::{attack::attack_phase, march::handle_direction_check};

pub fn sync_attack(
    attacker_map_east: &HashMap<ArmyName, Vec<ArmyName>>,
    attacker_map_west: &HashMap<ArmyName, Vec<ArmyName>>,
    battle_state: Battle,
) -> Battle {
    // EAST attacks
    let mut state_clone_one = battle_state.clone();
    let mut west_defenders_left = attack_phase(
        attacker_map_east,
        &state_clone_one.army_1_state,
        &mut state_clone_one.army_2_state,
    )
    .clone();

    // WEST attacks
    let mut state_clone_two = battle_state.clone();
    let mut east_defenders_left = attack_phase(
        attacker_map_west,
        &state_clone_two.army_2_state,
        &mut state_clone_two.army_1_state,
    )
    .clone();

    handle_direction_check(
        &mut east_defenders_left,
        &mut west_defenders_left,
        StartingDirection::EAST,
    );
    handle_direction_check(
        &mut west_defenders_left,
        &mut east_defenders_left,
        StartingDirection::WEST,
    );

    // Need to get the marching status from when they went through their own attack phase
    // SETTING EAST IS_MARCHING STATUS FROM CLONE_ONE (THEIR ATTACK)
    east_defenders_left.iter_mut().for_each(|d| {
        let matching_b = state_clone_one
            .army_1_state
            .iter()
            .find(|b| b.name == d.name);

        d.is_marching.set(matching_b.unwrap().is_marching.get());
    });

    // SETTING WEST IS_MARCHING STATUS FROM CLONE_TWO (THEIR ATTACK)
    west_defenders_left.iter_mut().for_each(|d| {
        let matching_b = state_clone_two
            .army_2_state
            .iter()
            .find(|b| b.name == d.name);

        d.is_marching.set(matching_b.unwrap().is_marching.get());
    });

    Battle {
        army_1_state: east_defenders_left,
        army_2_state: west_defenders_left,
    }
}
