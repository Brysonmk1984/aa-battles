use std::{collections::HashMap, ops::Deref};

use crate::match_up::match_up::Battalion;
use rand::Rng;

/**
* fn attack_phase -
    Loops through every available field in the attacker_map and for each one, checks the list of possible targets (the defender vec)
    If there is an available target, the attack sequence is ran
*/
pub fn attack_phase<'a, 'b>(
    attacker_map: &HashMap<String, Vec<&'a str>>,
    attacker: &'b mut Vec<Battalion>,
    defender: &'b mut Vec<Battalion>,
) {
    // For each attacker name in the map, if valid target, set marching=false and run attack sequence
    attacker_map.iter().for_each(|entry| {
        let (attacking_b_name, in_range_vec) = entry;
        let defending_b_name = in_range_vec.get(0);

        let mut a_battalion = attacker
            .iter_mut()
            .find(|battalion| battalion.name == *attacking_b_name)
            .unwrap();

        // If any valid targets for the attacker, run attack sequence
        if defending_b_name.is_some() {
            let mut d_battalion = defender
                .iter_mut()
                .find(|battalion| battalion.name == *defending_b_name.unwrap())
                .unwrap();

            a_battalion.set_is_marching(false);
            //println!("{} {}", a_battalion.name, d_battalion.name);
            run_attack_sequence(&mut a_battalion, &mut d_battalion);
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
fn run_attack_sequence(attacker: &mut Battalion, defender: &mut Battalion) {
    // println!(
    //     "Defender info in attack seq: {} {}",
    //     defender.name, defender.count
    // );
    // Do one attack attempt for each member of a battalion
    for n in 0..attacker.count {
        if defender.count == 0 {
            return;
        }

        let has_dodged_attack =
            try_dodge(attacker.accuracy, defender.agility, defender.is_marching);
        if has_dodged_attack {
            continue;
        }

        let has_blocked_attack = try_block(defender.shield_rating);
        if has_blocked_attack {
            continue;
        }

        // IF hasn't dodged or blocked, need to figure out how to  reduce battalion count
        defender.decrement();
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_try_block(d_shield_rating: f64) -> bool {
//         assert!(false, true);
//     }
// }
