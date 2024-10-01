use std::{collections::HashMap, env, ops::Deref, thread};

use crate::{
    types::{ArmorType, ArmyName, Battalion, Battle, Belligerent, StartingDirection, WeaponType},
    util::{push_log, push_stat_armor, push_stat_block, push_stat_dodge, WEAPON_ARMOR_CELL},
    IS_MARCHING_AGILITY_MOD,
};
use rand::Rng;
use std::string::ToString;

use super::attack::{try_armor_defense, try_block, try_dodge};

pub fn attack_phase_new(
    attacker_map: &HashMap<ArmyName, Vec<ArmyName>>,
    attackers: &Vec<Battalion>,
    defenders: &Vec<Battalion>,
    thread_num: u8,
) {
    // For each map key
    attacker_map.iter().enumerate().for_each(|(i, entry)| {
        println!("ATTACKER: {:?}", entry.0);
        // get first in vec of values
        let defending_b_name = entry.1.get(i).unwrap();

        // get A battalion from a param based on entry key
        let attacking_battalion = attackers.iter().find(|b| b.name == *entry.0).unwrap();

        // Currently isn't working as intended
        // each attack runs the attack sequence FOR EACH defender. so attacker[0] gets n*defenderCount (5) attacks in one tick
        run_attack_sequence(&attacking_battalion, defenders, thread_num);
    });
}

/**
* fn run_attack_sequence -
   Parent function for running functions related to an attack: try_dodge, try_block, decrement
*/
fn run_attack_sequence(
    attacker: &Battalion,
    combined_active_defenders: &Vec<Battalion>,
    thread_num: u8,
) {
    let attacker_count = attacker.count.load(Ordering::Acquire);
    for n in 0..attacker_count {
        // Pick a defender
        let defender_index = rand::thread_rng().gen_range(0..(combined_active_defenders.len() - 1));
        let defender = combined_active_defenders.get(defender_index).unwrap();

        // Run engagement steps multiple times depending on attack speed
        for a in 0..attacker.attack_speed {
            let result = run_engagement_steps(attacker, defender);

            // DO ATOMIC OPERATIONS

            // if result == EngagementOutcome::Hit {
            //     // Defending battalion loses a member or more depending on aoe
            //     defender.decrement(attacker.aoe, attacker.starting_direction);
            // } else if result == EngagementOutcome::Dodged
            //     && env::var("ENVIRONMENT").unwrap() == "test".to_string()
            // {
            //     test_only_count_dodges += 1;
            // }
        }
    }
}

#[derive(Debug, PartialEq)]
enum EngagementOutcome {
    Dodged,
    Blocked,
    ArmorSaved,
    Hit,
}

fn run_engagement_steps(attacker: &Battalion, defender: &Battalion) -> EngagementOutcome {
    let has_dodged_attack = try_dodge(
        attacker.accuracy,
        defender.agility,
        defender.is_marching,
        defender.starting_direction,
        || rand::thread_rng().gen_range(0..100),
    );
    if has_dodged_attack {
        return EngagementOutcome::Dodged;
    }

    let has_blocked_attack = try_block(
        defender.shield_rating,
        attacker.weapon_type != WeaponType::Magic,
        defender.starting_direction,
        || rand::thread_rng().gen_range(0..100),
    );
    if has_blocked_attack {
        return EngagementOutcome::Blocked;
    }

    let saved_by_armor = try_armor_defense(
        defender.armor_type,
        attacker.weapon_type,
        defender.starting_direction,
    );

    if saved_by_armor {
        return EngagementOutcome::ArmorSaved;
    }

    return EngagementOutcome::Hit;
}
