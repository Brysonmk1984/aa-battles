use std::collections::HashMap;

use crate::battle::tick::check_in_range::check_in_range;
use crate::match_up::match_up::Battalion;

pub fn run_tick<'a>(army_1: Vec<Battalion>, army_2: Vec<Battalion>) {
    //https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut in_range_map_1: HashMap<String, Vec<&str>> = HashMap::new();
    let mut in_range_map_2: HashMap<String, Vec<&str>> = HashMap::new();
    //in_range_map_1.insert(army_1[0].name.clone(), vec![])

    army_1.iter().for_each(|army| {
        in_range_map_1.insert(army.name.clone(), Vec::new());
    });

    army_2.iter().for_each(|army| {
        in_range_map_2.insert(army.name.clone(), vec![]);
    });

    check_in_range(&mut in_range_map_1, &army_1, &army_2);

    println!("{in_range_map_1:?}");
}
