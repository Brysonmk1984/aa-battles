use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::match_up::match_up::Battalion;

pub fn check_in_range<'a>(
    attacker_map: &mut HashMap<String, Vec<&'a str>>,
    attacker: &'a Vec<Battalion>,
    defender: &'a Vec<Battalion>,
) {
    // loop through army_1 and figure out which of army_2 is in range
    for (battalion_key, in_range_vec) in attacker_map {
        let mut flyer_vec = Vec::new();
        let mut ground_vec = Vec::new();

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
                // insert defenders flyers in the flyer vec, otherwise the ground vec
                if attacker_range > 0 && battalion.flying {
                    // In range, can hit air, and enemy is flying
                    flyer_vec.push(battalion.name.as_str());
                } else if !battalion.flying {
                    // In range, enemy is non-flyer
                    ground_vec.push(battalion.name.as_str())
                } else {
                    // In range, can't hit enemy flyer
                }
            }
        });

        // Randomly shuffle the two vecs, this will dictate priority with attacks
        flyer_vec.shuffle(&mut rand::thread_rng());
        ground_vec.shuffle(&mut rand::thread_rng());

        // Flyers will be prioritized over ground enemies
        let combined_vec = [flyer_vec, ground_vec].concat();

        // push arranged, combined vec items into the in_range vec on the attacker
        combined_vec.iter().for_each(|b_name| {
            in_range_vec.push(b_name);
        });
    }
}
