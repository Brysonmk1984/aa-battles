use std::{collections::HashMap, env};

use rand::seq::SliceRandom;

use crate::{match_up::match_up::Battalion, MIN_RANGE_ATTACK_AIR};

pub fn update_in_range_map<'a>(
    attacker_map: &mut HashMap<String, Vec<&'a str>>,
    attacker: &'a Vec<Battalion>,
    defender: &'a Vec<Battalion>,
) {
    // loop through army_1 and figure out which of army_2 is in range
    for (battalion_key, in_range_vec) in attacker_map {
        let mut flyer_vec = Vec::new();
        let mut ground_vec = Vec::new();

        // For each battalion in the defender's army, determine which are in range of the attacker
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
            // println!(
            //     "{attacker_position} {defender_position} {distance_between_battalions} {in_range}"
            // );

            // TODO: Consider a more elaborate check for range finding when both are marching and march past each other rather than attack
            // For now, resolved this by adjusting speed down and range up.
            if in_range && battalion.count > 0 {
                // insert defenders flyers in the flyer vec, otherwise the ground vec
                if attacker_range > MIN_RANGE_ATTACK_AIR && battalion.flying {
                    // println!(
                    //     "{} IN RANGE OF {}, CAN HIT FLYER",
                    //     attacker_battalion.name, battalion.name
                    // );
                    // In range, can hit air, and enemy is flying
                    flyer_vec.push(battalion.name.as_str());
                } else if !battalion.flying {
                    // println!(
                    //     "{} IN RANGE OF {}, A GROUND FORCE",
                    //     attacker_battalion.name, battalion.name
                    // );
                    // In range, enemy is non-flyer
                    ground_vec.push(battalion.name.as_str())
                } else {
                    // println!(
                    //     "{} IN RANGE OF {}, BUT CANNOT HIT FLYER",
                    //     attacker_battalion.name, battalion.name
                    // );
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
