use std::collections::HashMap;

use crate::match_up::match_up::Battalion;
use rand::Rng;
pub fn attack<'a>(
    attacker_map: &mut HashMap<String, Vec<&'a str>>,
    attacker: &'a Vec<Battalion>,
    defender: &'a Vec<Battalion>,
) {
    attacker_map.into_iter().for_each(|entry| {
        let (attacking_battalion, in_range_vec) = entry;
        println!("entry: {attacking_battalion} {in_range_vec:?}");

        let mut a_battalion = attacker
            .iter()
            .find(|battalion| battalion.name == *attacking_battalion)
            .unwrap()
            .clone();

        if a_battalion.range > 0 {
            println!("has range : {}", a_battalion.range);
        }

        let defending_b_name = in_range_vec.get(0);

        // If any valid targets for the attacker, run attack sequence
        if defending_b_name.is_some() {
            let mut d_battalion = defender
                .iter()
                .find(|battalion| battalion.name == *defending_b_name.unwrap())
                .unwrap()
                .clone();

            run_attack_sequence(&mut a_battalion, &mut d_battalion);
        }
    });
}

fn run_attack_sequence(attacker: &mut Battalion, defender: &mut Battalion) {
    println!("ATTA: {attacker:?} DEFENDER: {defender:?}");

    //let has_dodged_attack = try_dodge(attacker.accuracy, defender.agility, false);

    // if has_dodged_attack {
    //     return;
    // }

    // let has_blocked_attack = try_block(defender.shield_rating);
    // if has_blocked_attack {
    //     return;
    // }

    defender.decrement();
    // IF hasn't dodged or blocked, need to figure out how to  reduce battalion count
}

// All attacks have a chance to miss, which is higher if they're marching
fn try_dodge(a_accuracy: f64, d_agility: f64, d_is_marching: bool) -> bool {
    let is_marching_modifier = if d_is_marching { 0.25 } else { 0.0 };

    // 1.0 accuracy = 100% chance to hit - (agility + is_marching)
    let chance_to_dodge = d_agility + is_marching_modifier;
    println!("{a_accuracy} {chance_to_dodge}");
    let chance_to_hit = ((a_accuracy - chance_to_dodge) * 100.0) as u64;

    let random_dodge_num = rand::thread_rng().gen_range(0..100);

    let successfully_dodged = chance_to_hit > random_dodge_num;

    successfully_dodged
}

// Shielded defenders have a chance to block
fn try_block(d_shield_rating: f64) -> bool {
    let chance_to_block = (d_shield_rating * 100.0) as u64;
    let random_block_num = rand::thread_rng().gen_range(0..100);

    let successfully_blocked = chance_to_block > random_block_num;

    successfully_blocked
}
