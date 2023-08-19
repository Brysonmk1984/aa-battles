use std::{collections::HashMap, ops::Deref};

use crate::{
    match_up::match_up::{Battalion, StartingDirection},
    service::query::{ArmorType, ArmyName, WeaponType},
    util::WEAPON_ARMOR_CELL,
};
use rand::Rng;
use std::string::ToString;
/**
* fn attack_phase -
    Loops through every available field in the attacker_map and for each one, checks the list of possible targets (the defender vec)
    If there is an available target, the attack sequence is ran
*/
pub fn attack_phase<'a, 'b>(
    attacker_map: &HashMap<ArmyName, Vec<ArmyName>>,
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

        let has_past_all_defenders = determine_past_all_defenders(&a_battalion, &defender);

        // If no valid targets, march and early return
        if defending_b_name.is_none() {
            transition_to_march(attacking_b_name, attacker, has_past_all_defenders);
            return;
        }

        let mut d_battalion = defender
            .iter_mut()
            .find(|battalion| battalion.name == *defending_b_name.unwrap())
            .unwrap();

        // If defending battalion is already dead, from previous attacker_map iteration, march instead of attack
        if d_battalion.count == 0 {
            transition_to_march(attacking_b_name, attacker, has_past_all_defenders);
            return;
        }

        // If any valid targets for the attacker, run attack sequence
        a_battalion.set_is_marching(false);
        run_attack_sequence(&mut a_battalion, &mut d_battalion);
    });
}

fn determine_past_all_defenders(attacker: &Battalion, defenders: &Vec<Battalion>) -> bool {
    let attacker_is_west = attacker.starting_direction == StartingDirection::WEST;
    let defender_is_west = !attacker_is_west;
    let init = if defender_is_west { -150 } else { 150 };
    // Min position is the furthest point back on the numberline, so for west it's a negative number, east it's positive
    let defender_min_position = defenders.iter().fold(init, |mut min: i32, d| {
        if d.count > 0 {
            //println!("{} {}", d.name, d.position);
            if defender_is_west {
                if d.position > min {
                    min = d.position
                }
            } else {
                if d.position < min {
                    min = d.position
                }
            }
        }

        min
    });

    //println!("IS WEST={defender_is_west}, {defender_min_position}");

    if attacker_is_west {
        attacker.position > defender_min_position
    } else {
        attacker.position < defender_min_position
    }
}

fn transition_to_march(attacking_b_name: &ArmyName, attacker: &mut Vec<Battalion>, has_past: bool) {
    // If attacker had no valid targets (defenders), then army will march forward
    let mut a_battalion = attacker
        .iter_mut()
        .find(|battalion| battalion.name == *attacking_b_name)
        .unwrap();
    // Flyers need to back track to continue finding armies that may have passed them underneath
    if a_battalion.flying && has_past {
        a_battalion.set_is_reverse_direction(true);
    }

    a_battalion.set_is_marching(true);
}

#[derive(PartialEq)]
enum EngagementOutcome {
    Dodged,
    Blocked,
    ArmorSaved,
    Hit,
}

/**
* fn run_attack_sequence -
   Parent function for running functions related to an attack: try_dodge, try_block, decrement
*/
fn run_attack_sequence(attacker: &mut Battalion, defender: &mut Battalion) {
    // Do one attack attempt for each member of a battalion
    for n in 0..attacker.count {
        for a in 0..attacker.attack_speed {
            if defender.count == 0 {
                return;
            }

            let result = run_engagement_steps(attacker, defender);

            if result == EngagementOutcome::Hit {
                // Defending battalion loses a member or more depending on aoe
                defender.decrement(attacker.aoe);
            }
        }
    }
}

fn run_engagement_steps(attacker: &mut Battalion, defender: &mut Battalion) -> EngagementOutcome {
    let has_dodged_attack = try_dodge(
        attacker.accuracy,
        defender.agility,
        defender.is_marching,
        || rand::thread_rng().gen_range(0..100),
    );
    if has_dodged_attack {
        return EngagementOutcome::Dodged;
    }

    let has_blocked_attack = try_block(
        defender.shield_rating,
        attacker.weapon_type != WeaponType::Magic,
        || rand::thread_rng().gen_range(0..100),
    );
    if has_blocked_attack {
        return EngagementOutcome::Blocked;
    }

    let saved_by_armor = try_armor_defense(defender.armor_type, attacker.weapon_type);

    if saved_by_armor {
        return EngagementOutcome::ArmorSaved;
    }

    return EngagementOutcome::Hit;
}

/**
* fn try_dodge -
   Checks if an attack is dodged. All attacks have a chance to miss, which is higher if they're marching
*/
pub fn try_dodge(
    a_accuracy: f64,
    d_agility: f64,
    d_is_marching: bool,
    randomizer_func: impl Fn() -> u64,
) -> bool {
    let is_marching_modifier = if d_is_marching { 0.25 } else { 0.0 };
    // 1.0 accuracy = 100% chance to hit - (agility + is_marching)
    let chance_to_dodge = d_agility + is_marching_modifier;
    let chance_to_hit = ((a_accuracy - chance_to_dodge) * 100.0) as u64;

    let random_dodge_num = randomizer_func();

    chance_to_hit <= random_dodge_num
}

/**
* fn try_block -
   Checks if an attack is dodged. Only shielded defenders have a chance to block
*/
pub fn try_block(
    d_shield_rating: f64,
    is_valid_attack_to_block: bool,
    randomizer_func: impl Fn() -> u64,
) -> bool {
    if !is_valid_attack_to_block {
        return false;
    }

    let chance_to_block = (d_shield_rating * 100.0) as u64;
    let random_block_num = randomizer_func();

    chance_to_block > random_block_num
}

pub fn try_armor_defense(armor: ArmorType, weapon: WeaponType) -> bool {
    let weapon_armor_map = WEAPON_ARMOR_CELL.get().unwrap();
    let weapon_armor_combo = weapon.to_string() + "-" + armor.to_string().as_str();

    let chance_to_hit_option = weapon_armor_map.get(weapon_armor_combo.as_str());

    if let Some(hit_float) = chance_to_hit_option {
        let random_attack_num = rand::thread_rng().gen_range(0..100);
        if random_attack_num > (*hit_float * 100.0).round() as i64 {
            // Successful hit, unsuccessful armor defense
            return false;
        } else {
            return true;
        }

        return false;
    } else {
        panic!("WeaponType-ArmorType not supported! {weapon_armor_combo}")
    }
}

#[cfg(test)]
mod tests {
    use crate::battle::tick::phases::attack::{try_block, try_dodge};
    use rand::Rng;

    #[test]
    fn try_dodge_pass_no_march() {
        let a_accuracy = 0.8;
        let d_agility = 0.2;
        let d_is_marching = false;
        let random_dodge_num = 60;
        // chance_to_dodge = (d_agility + (d_is_marching ? .25 : 0)
        // chance_to_hit = a_accuracy - chance_to_dodge) * 100
        // .8 - (.2 + 0) = .6 * 100 = 60
        // chance_to_hit < random_dodge_num
        let randomizer_func = || random_dodge_num;
        let successfully_dodged = try_dodge(a_accuracy, d_agility, d_is_marching, randomizer_func);
        assert!(successfully_dodged);
    }
    #[test]
    fn try_dodge_fail_no_march() {
        let a_accuracy = 0.8;
        let d_agility = 0.2;
        let d_is_marching = false;
        let random_dodge_num = 59;
        let randomizer_func = || random_dodge_num;
        let successfully_dodged = try_dodge(a_accuracy, d_agility, d_is_marching, randomizer_func);
        assert!(!successfully_dodged);
    }

    #[test]
    fn try_dodge_pass_march() {
        let a_accuracy = 0.7;
        let d_agility = 0.2;
        let d_is_marching = true;
        let random_dodge_num = 24;
        let randomizer_func = || random_dodge_num;
        let successfully_dodged = try_dodge(a_accuracy, d_agility, d_is_marching, randomizer_func);
        assert!(successfully_dodged);
    }

    #[test]
    fn try_dodge_fail_march() {
        let a_accuracy = 0.7;
        let d_agility = 0.2;
        let d_is_marching = true;
        let random_dodge_num = 23;
        let randomizer_func = || random_dodge_num;
        let successfully_dodged = try_dodge(a_accuracy, d_agility, d_is_marching, randomizer_func);
        assert!(!successfully_dodged);
    }

    fn try_block_pass() {
        let d_shield_rating = 0.4;
        let randomizer_func = || 39;
        let successfully_blocked = try_block(d_shield_rating, true, randomizer_func);
        assert!(successfully_blocked);
    }

    #[test]
    fn try_block_fail() {
        let d_shield_rating = 0.4;
        let randomizer_func = || 41;
        let successfully_blocked = try_block(d_shield_rating, true, randomizer_func);
        assert_eq!(successfully_blocked, false);
    }
}
