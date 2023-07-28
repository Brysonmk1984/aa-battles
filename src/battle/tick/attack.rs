use std::{collections::HashMap, ops::Deref};

use crate::match_up::match_up::Battalion;
use rand::Rng;

/**
* fn attack -
    Loops through every available field in the attacker_map and for each one, checks the list of possible targets (the defender vec)
    If there is an available target, the attack sequence is ran
*/
pub fn attack<'a, 'b>(
    attacker_map: &HashMap<String, Vec<&'a str>>,
    attacker: &'b mut Vec<Battalion>,
    defender: &'b mut Vec<Battalion>,
) {
    // For each attacker name in the map, if valid target, set marching=false and run attack sequence
    attacker_map.iter().for_each(|entry| {
        let (attacking_b_name, in_range_vec) = entry;

        let defending_b_name = in_range_vec.get(0);

        // If any valid targets for the attacker, run attack sequence
        if defending_b_name.is_some() {
            run_attack_sequence(
                attacker,
                defender,
                attacking_b_name.as_str(),
                *defending_b_name.unwrap(),
            );
        } else {
            // If attacker had no valid targets (defenders), then army will march forward
            let mut a_battalion = attacker
                .iter_mut()
                .find(|battalion| battalion.name == *attacking_b_name)
                .unwrap();

            a_battalion.set_is_marching(true);
        }
    });
}

/**
* fn run_attack_sequence -
   Parent function for running functions related to an attack: try_dodge, try_block, decrement
*/
fn run_attack_sequence(
    attacker: &mut Vec<Battalion>,
    defender: &mut Vec<Battalion>,
    attacking_b_name: &str,
    defending_b_name: &str,
) {
    println!("ATTACKER: {attacking_b_name} DEFENDER: {defending_b_name}");

    let mut a_battalion = attacker
        .iter_mut()
        .find(|battalion| battalion.name == *attacking_b_name)
        .unwrap();
    // Weird that that is set here. It's an artifact of needing to pass in the entire vec
    a_battalion.set_is_marching(false);

    let mut d_battalion = defender
        .iter_mut()
        .find(|battalion| battalion.name == defending_b_name)
        .unwrap();

    // Do one attack attempt for each member of a battalion
    for n in 0..a_battalion.count {
        if d_battalion.count == 0 {
            return;
        }

        let has_dodged_attack = try_dodge(
            a_battalion.accuracy,
            d_battalion.agility,
            d_battalion.is_marching,
        );
        if has_dodged_attack {
            continue;
        }

        let has_blocked_attack = try_block(d_battalion.shield_rating);
        if has_blocked_attack {
            continue;
        }

        // IF hasn't dodged or blocked, need to figure out how to  reduce battalion count
        d_battalion.decrement();
    }
}

/**
* fn try_dodge -
   Checks if an attack is dodged. All attacks have a chance to miss, which is higher if they're marching
*/
fn try_dodge(a_accuracy: f64, d_agility: f64, d_is_marching: bool) -> bool {
    let is_marching_modifier = if d_is_marching { 0.25 } else { 0.0 };

    // 1.0 accuracy = 100% chance to hit - (agility + is_marching)
    let chance_to_dodge = d_agility + is_marching_modifier;
    //println!("ACC{a_accuracy} DDG{chance_to_dodge}");
    let chance_to_hit = ((a_accuracy - chance_to_dodge) * 100.0) as u64;

    let random_dodge_num = rand::thread_rng().gen_range(0..100);
    //println!("CTH{chance_to_hit} RND{random_dodge_num}");
    let successfully_dodged = chance_to_hit < random_dodge_num;
    // if successfully_dodged {
    //     println!("DODGED! {successfully_dodged}")
    // }

    successfully_dodged
}

/**
* fn try_block -
   Checks if an attack is dodged. Only shielded defenders have a chance to block
*/
fn try_block(d_shield_rating: f64) -> bool {
    let chance_to_block = (d_shield_rating * 100.0) as u64;
    let random_block_num = rand::thread_rng().gen_range(0..100);

    let successfully_blocked = chance_to_block > random_block_num;
    // if successfully_blocked {
    //     println!("BLOCKED! {successfully_blocked}")
    // }

    successfully_blocked
}
