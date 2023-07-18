use std::collections::HashMap;

use crate::match_up::match_up::Battalion;

pub fn check_in_range<'a>(
    attacker_map: &mut HashMap<String, Vec<&'a str>>,
    attacker: &'a Vec<Battalion>,
    defender: &'a Vec<Battalion>,
) {
    // loop through army_1 and figure out which of army_2 is in range
    for (battalion_key, in_range_vec) in attacker_map {
        defender.iter().for_each(|battalion| {
            let defender_position = battalion.position;
            let attacker_battalion = attacker
                .iter()
                .find(|battalion| battalion.name == *battalion_key)
                .unwrap();
            let attacker_position = attacker_battalion.position;
            let attacker_range = attacker_battalion.range;

            let distance_between_battalions = attacker_position - defender_position;

            let in_range = distance_between_battalions.abs() - attacker_range <= 0;

            if in_range {
                in_range_vec.push(battalion.name.as_str())
            }
        })
    }
}
