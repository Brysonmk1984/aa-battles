use std::collections::HashMap;

use crate::{match_up::match_up::Battalion, service::query::Army};

//use super::check_in_range::check_in_range;

pub fn run_tick(army_1: Vec<Battalion>, army_2: Vec<Battalion>) {
    //https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut in_range_map_1: HashMap<String, Vec<&str>> = HashMap::new();
    let mut in_range_map_2: HashMap<String, Vec<&str>> = HashMap::new();
    //in_range_map_1.insert(army_1[0].name.clone(), vec![])

    army_1.iter().for_each(|army| {
        in_range_map_1.insert(army.name.clone(), vec![]);
    });

    println!("{in_range_map_1:?}");

    // let (ah1, ah2) = check_in_range(in_range_map_1, in_range_map_2);
    // in_range_map_1 = ah1;
    // in_range_map_2 = ah2;
}
