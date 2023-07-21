use std::collections::HashMap;

use crate::battle::tick::attack::attack;
use crate::battle::tick::check_in_range::check_in_range;
use crate::match_up::match_up::Battalion;

pub fn run_tick(army_1: Vec<Battalion>, army_2: Vec<Battalion>) {
    //https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut in_range_map_1: HashMap<String, Vec<&str>> = HashMap::new();
    let mut in_range_map_2: HashMap<String, Vec<&str>> = HashMap::new();

    army_1.iter().for_each(|army| {
        in_range_map_1.insert(army.name.clone(), Vec::new());
    });

    army_2.iter().for_each(|army| {
        in_range_map_2.insert(army.name.clone(), vec![]);
    });

    // STEP 1: Check for range
    check_in_range(&mut in_range_map_1, &army_1, &army_2);
    check_in_range(&mut in_range_map_2, &army_2, &army_1);

    // STEP 2: Attack Battalions within range
    // STEP 2a: army_1 Attacks army_2 (Concurrently with step 2b)
    attack(&mut in_range_map_1, &army_1, &army_2);
    // STEP 2b: army_2 Attacks army_1 (Concurrently with step 2a)

    //STEP 3: Reconcilliation - lower army counts by results of 2aa & 2bb

    // STEP 4: March forward

    //println!("{in_range_map_1:?} \n\n {in_range_map_2:?}");
}
