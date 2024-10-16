use std::{
    collections::HashMap,
    env,
    num::Wrapping,
    ops::Deref,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    entities::{battalion::battalion::Battalion, battle::battle::Battle},
    enums::{ArmorType, ArmyName, Belligerent, StartingDirection, WeaponType},
    util::{
        determine_aoe_effect, push_log, push_stat_armor, push_stat_block, push_stat_dodge,
        push_stat_kill, WEAPON_ARMOR_CELL,
    },
    IS_MARCHING_AGILITY_MOD,
};
use rand::Rng;
use std::string::ToString;

pub fn attack_phase<'a>(
    attacker_map: &HashMap<ArmyName, Vec<ArmyName>>,
    attackers: &Vec<Battalion>,
    defenders: &'a Vec<Battalion>,
) -> &'a Vec<Battalion> {
    // For each ATTACKER key, run attack sequence
    attacker_map
        .iter()
        .for_each(|(attacker, matching_defenders)| {
            let attacking_battalion = attackers.iter().find(|b| b.name == *attacker).unwrap();

            let defending_battalions = matching_defenders
                .iter()
                .map(|a| defenders.iter().find(|d| d.name == *a).unwrap())
                .collect::<Vec<&Battalion>>();

            if matching_defenders.len() > 0 {
                run_attack_sequence(attacking_battalion, &defending_battalions);
            }

            let post_attack_matching_defender_count =
                defending_battalions.iter().fold(0, |mut acc, cur| {
                    acc += cur.count.get();
                    acc
                });
            println!("DEF COUNT{}", post_attack_matching_defender_count);
            println!(
                "AFTER ATTACK marching BEING SET TO {} for {:?}, it was {:?}",
                post_attack_matching_defender_count == 0,
                attacking_battalion.name,
                attacking_battalion.is_marching
            );
            if post_attack_matching_defender_count == 0 {
                attacking_battalion.set_is_marching(true, None);
            }
        });
    //println!("DEF COUNT{}", post_attack_matching_defender_count);
    return defenders;
}

/**
* fn run_attack_sequence -
   Parent function for running functions related to an attack: try_dodge, try_block, decrement
*/
fn run_attack_sequence(attacker: &Battalion, combined_active_defenders: &Vec<&Battalion>) {
    // todo: not always going to be accurate
    attacker.set_is_marching(false, Some(&combined_active_defenders[0].name));
    //println!("RUNNING ATTACK SEQUENCE ");
    for n in 0..attacker.count.get() {
        //println!("COMBINED DEFENDERS:{}", combined_active_defenders.len());
        // Pick a defender
        let defender_index = rand::thread_rng().gen_range(0..(combined_active_defenders.len()));
        let defender = combined_active_defenders.get(defender_index).unwrap();
        let mut test_only_count_dodges = 0;

        // Still need to log this:
        // attacker.set_is_marching(false, Some(&defender.name));

        // Run engagement steps multiple times depending on attack speed
        for a in 0..attacker.attack_speed {
            // Defending battalion loses a member or more depending on aoe
            let result = run_engagement_steps(attacker, defender);

            if result == EngagementOutcome::Hit {
                let defender_hit_x_times =
                    determine_aoe_effect(&attacker.aoe, defender.spread as i32) as u32;

                let defender_count = defender.count.get();

                if defender_count.checked_sub(defender_hit_x_times).is_some() {
                    defender.count.set(defender_count - defender_hit_x_times);
                    push_stat_kill(defender_hit_x_times as u32, attacker.starting_direction);
                } else {
                    let count_copy = defender.count.get();
                    push_stat_kill(count_copy, attacker.starting_direction);
                    defender.count.set(0);
                    return;
                }
            } else if result == EngagementOutcome::Dodged
                && env::var("ENVIRONMENT").unwrap() == "test".to_string()
            {
                println!("IN DODGED, SHOULDNT HAPPEN");
                test_only_count_dodges += 1;
            }
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
        defender.is_marching.get(),
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

pub fn try_dodge(
    a_accuracy: f64,
    d_agility: f64,
    d_is_marching: bool,
    starting_direction: StartingDirection,
    randomizer_func: impl Fn() -> u64,
) -> bool {
    let is_marching_mod = if d_is_marching {
        IS_MARCHING_AGILITY_MOD
    } else {
        0.0
    };
    // 1.0 accuracy = 100% chance to hit - (agility + is_marching)
    let chance_to_dodge = d_agility + is_marching_mod;
    //println!("DODGE DETAILS {a_accuracy} {chance_to_dodge} {is_marching_mod}");
    let chance_to_hit = ((a_accuracy - chance_to_dodge) * 100.0) as u64;

    if chance_to_hit == 0 {
        push_log(
            "Defender is unhittable. Agility is too high for the attacking battalion!".to_string(),
        );
        // panic!(
        //     "Chance to hit in try_dodge is {chance_to_hit} and chance to dodge is {chance_to_dodge}. Is this intentional?"
        // );
    }

    let random_dodge_num = randomizer_func();

    let has_dodged = chance_to_hit <= random_dodge_num;
    if has_dodged {
        push_stat_dodge(starting_direction);
    }

    has_dodged
}

/**
* fn try_block -
   Checks if an attack is dodged. Only shielded defenders have a chance to block
*/
pub fn try_block(
    d_shield_rating: f64,
    is_valid_attack_to_block: bool,
    starting_direction: StartingDirection,
    randomizer_func: impl Fn() -> u64,
) -> bool {
    if !is_valid_attack_to_block {
        return false;
    }

    let chance_to_block = (d_shield_rating * 100.0) as u64;
    let random_block_num = randomizer_func();

    if chance_to_block == 1 {
        push_log("Defender is too heavily shielded for the attacking battalion!".to_string());
        //panic!("Chance to block in try_block is {chance_to_block}. Is this intentional?");
    }

    let has_blocked = chance_to_block > random_block_num;

    if has_blocked {
        println!(" BLOCKED {chance_to_block} {random_block_num}");
        push_stat_block(starting_direction);
    }
    has_blocked
}

pub fn try_armor_defense(
    armor: ArmorType,
    weapon: WeaponType,
    starting_direction: StartingDirection,
) -> bool {
    let weapon_armor_map = WEAPON_ARMOR_CELL.get().unwrap();
    let weapon_armor_combo = weapon.to_string() + "-" + armor.to_string().as_str();

    let chance_to_hit_option = weapon_armor_map.get(weapon_armor_combo.as_str());

    if let Some(hit_float) = chance_to_hit_option {
        let random_defense_num = rand::thread_rng().gen_range(0..100);
        if random_defense_num < (*hit_float * 100.0).round() as i64 {
            // Successful hit, unsuccessful armor defense
            return false;
        } else {
            push_stat_armor(starting_direction);
            return true;
        }

        return false;
    } else {
        panic!("WeaponType-ArmorType not supported! {weapon_armor_combo}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        battle::tick::phases::attack::{try_block, try_dodge},
        enums::StartingDirection,
    };
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
        let starting_direction = StartingDirection::EAST;
        let successfully_dodged = try_dodge(
            a_accuracy,
            d_agility,
            d_is_marching,
            starting_direction,
            randomizer_func,
        );
        assert!(successfully_dodged);
    }

    #[test]
    fn try_dodge_fail_no_march() {
        let a_accuracy = 0.8;
        let d_agility = 0.2;
        let d_is_marching = false;
        let random_dodge_num = 59;
        let randomizer_func = || random_dodge_num;
        let starting_direction = StartingDirection::EAST;
        let successfully_dodged = try_dodge(
            a_accuracy,
            d_agility,
            d_is_marching,
            starting_direction,
            randomizer_func,
        );
        assert!(!successfully_dodged);
    }

    #[test]
    fn try_dodge_pass_march() {
        let a_accuracy = 0.7;
        let d_agility = 0.3;
        let d_is_marching = true;
        let random_dodge_num = 25;
        let randomizer_func = || random_dodge_num;
        let starting_direction = StartingDirection::EAST;
        let successfully_dodged = try_dodge(
            a_accuracy,
            d_agility,
            d_is_marching,
            starting_direction,
            randomizer_func,
        );
        assert!(successfully_dodged);
    }

    #[test]
    fn try_dodge_fail_march() {
        let a_accuracy = 0.7;
        let d_agility = 0.2;
        let d_is_marching = true;
        let random_dodge_num = 23;
        let randomizer_func = || random_dodge_num;
        let starting_direction = StartingDirection::EAST;
        let successfully_dodged = try_dodge(
            a_accuracy,
            d_agility,
            d_is_marching,
            starting_direction,
            randomizer_func,
        );
        assert!(!successfully_dodged);
    }

    #[test]
    fn try_block_pass() {
        let d_shield_rating = 0.4;
        let randomizer_func = || 39;
        let starting_direction = StartingDirection::EAST;
        let successfully_blocked =
            try_block(d_shield_rating, true, starting_direction, randomizer_func);
        assert!(successfully_blocked);
    }

    #[test]
    fn try_block_fail() {
        let d_shield_rating = 0.4;
        let randomizer_func = || 41;
        let starting_direction = StartingDirection::EAST;
        let successfully_blocked =
            try_block(d_shield_rating, true, starting_direction, randomizer_func);
        assert_eq!(successfully_blocked, false);
    }
}
