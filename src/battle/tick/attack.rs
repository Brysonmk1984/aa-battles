use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::match_up::match_up::Battalion;

pub fn attack<'a>(
    attacker_map: &mut HashMap<String, Vec<&'a str>>,
    attacker: &'a Vec<Battalion>,
    defender: &'a Vec<Battalion>,
) {
    attacker_map.into_iter().for_each(|entry| {
        let (attacking_battalion, in_range_vec) = entry;
        println!("cake {attacking_battalion} {in_range_vec:?}");

        // randomly select an opposing battalion in range
        // TODO: filter out air units when attacker can't hit air
        let defending_b_name = in_range_vec.choose(&mut rand::thread_rng());

        // If any valid targets for the attacker, run attack sequence
        if defending_b_name.is_some() {
            let a_battalion = attacker
                .iter()
                .find(|battalion| battalion.name == *attacking_battalion)
                .unwrap();

            let d_battalion = defender
                .iter()
                .find(|battalion| battalion.name == *defending_b_name.unwrap())
                .unwrap();

            run_attack_sequence(a_battalion, d_battalion);
        }
    });
}

fn run_attack_sequence(attacker: &Battalion, defender: &Battalion) {
    println!("ATTA: {attacker:?} DEFENDER: {defender:?}");
}

// Check if attacker can attack defender (has range to hit flyer)
fn is_valid_target(attacker: &String, defender: &String, is_flying: bool) {}

// All attacks have a chance to miss, which is higher if they're marching
//fn try_dodge(attacker: &String, defender: &String, is_marching: bool) {}

// Shielded defenders have a chance to block
//fn try_block(attacker: &String, defender: &String, is_marching: bool) {}
