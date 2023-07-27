use std::collections::HashMap;

use crate::battle::tick::attack::attack;
use crate::battle::tick::check_in_range::check_in_range;
use crate::match_up::match_up::Battalion;
use crate::BattleState;

use super::march::march;

pub fn run_tick(battle_state: &mut BattleState) {
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
    check_in_range(&mut in_range_map_1, &army_1_clone, &army_2_clone);
    check_in_range(&mut in_range_map_2, &army_2_clone, &army_1_clone);

    // STEP 2: Attack Battalions within range
    // STEP 2a: army_1 Attacks army_2 (Concurrently with step 2b)
    attack(
        &in_range_map_1,
        &mut battle_state.army_1_state,
        &mut battle_state.army_2_state,
    );
    // STEP 2b: army_2 Attacks army_1 (Concurrently with step 2a)
    attack(
        &in_range_map_2,
        &mut battle_state.army_2_state,
        &mut battle_state.army_1_state,
    );

    // STEP 3: March forward

    march(&mut battle_state.army_1_state);
    //march(&mut marching_battalions_2);

    println!("{in_range_map_1:?} \n\n {in_range_map_2:?}");
}
