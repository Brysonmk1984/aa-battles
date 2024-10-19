use super::phases::march::march_phase;
use super::phases::range_find::update_in_range_map;

use crate::battle::tick::sync_attack::sync_attack;
use crate::enums::{ArmyName, StartingDirection};
use crate::Battle;
use std::collections::HashMap;
use std::thread::spawn;

pub fn run_tick(battle_state: &mut Battle) -> u32 {
    let mut in_range_map_east: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
    let mut in_range_map_west: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();

    battle_state.army_1_state.iter().for_each(|army| {
        in_range_map_east.insert(army.name.clone(), Vec::new());
    });

    battle_state.army_2_state.iter().for_each(|army| {
        in_range_map_west.insert(army.name.clone(), Vec::new());
    });

    // STEP 1: Check for range
    let east_marchers = update_in_range_map(
        &mut in_range_map_east,
        &battle_state.army_1_state,
        &battle_state.army_2_state,
    );
    let west_marchers = update_in_range_map(
        &mut in_range_map_west,
        &battle_state.army_2_state,
        &battle_state.army_1_state,
    );

    let cloned_state = battle_state.clone();

    // STEP 2: Do attacks (both sides)
    let new_state = sync_attack(&in_range_map_east, &in_range_map_west, cloned_state);
    let is_m = (
        &new_state.army_1_state[0].is_marching.get(),
        &new_state.army_2_state[0].is_marching.get(),
    );

    // Update state with result of sync_attack
    battle_state.army_1_state = new_state.army_1_state;
    battle_state.army_2_state = new_state.army_2_state;

    // STEP 3: Adjust Counts
    let mut eastern_army_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count.get();
        sum
    });
    let mut western_army_count = battle_state.army_2_state.iter().fold(0, |mut sum, b| {
        sum += b.count.get();
        sum
    });
    println!("{western_army_count}, {eastern_army_count}");
    println!("------- March Phase ------");
    // STEP 4: March forward
    if western_army_count > 0 && eastern_army_count >= 0 {
        march_phase(&mut battle_state.army_1_state, &StartingDirection::EAST);
        march_phase(&mut battle_state.army_2_state, &StartingDirection::WEST);
    }

    let total_combined_count = western_army_count + eastern_army_count;
    total_combined_count.into()
}
