use super::attack_phase::attack::attack_phase;
use super::march::march_phase;
use super::range_find::update_in_range_map;
use crate::match_up::match_up::{Battalion, StartingDirection};
use crate::BattleState;
use std::collections::HashMap;

pub fn run_tick(battle_state: &mut BattleState, total_combined_count: i32) -> i32 {
    //https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut in_range_map_1: HashMap<String, Vec<&str>> = HashMap::new();
    let mut in_range_map_2: HashMap<String, Vec<&str>> = HashMap::new();

    battle_state.army_1_state.iter().for_each(|army| {
        in_range_map_1.insert(army.name.clone(), Vec::new());
    });

    battle_state.army_2_state.iter().for_each(|army| {
        in_range_map_2.insert(army.name.clone(), vec![]);
    });

    let army_1_clone = battle_state.army_1_state.clone();
    let army_2_clone = battle_state.army_2_state.clone();

    // STEP 1: Check for range
    update_in_range_map(&mut in_range_map_1, &army_1_clone, &army_2_clone);
    update_in_range_map(&mut in_range_map_2, &army_2_clone, &army_1_clone);

    // STEP 2: Attack Battalions within range
    // STEP 2a: army_1 Attacks army_2 (Concurrently with step 2b)
    attack_phase(
        &in_range_map_1,
        &mut battle_state.army_1_state,
        &mut battle_state.army_2_state,
    );
    // STEP 2b: army_2 Attacks army_1 (Concurrently with step 2a)
    attack_phase(
        &in_range_map_2,
        &mut battle_state.army_2_state,
        &mut battle_state.army_1_state,
    );

    // STEP 3: March forward
    march_phase(&mut battle_state.army_1_state, &StartingDirection::WEST);
    march_phase(&mut battle_state.army_2_state, &StartingDirection::EAST);

    //println!("{in_range_map_1:?} \n\n {in_range_map_2:?}");

    let mut a1 = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut a2 = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let new_total = a1 + a2;

    new_total
}
