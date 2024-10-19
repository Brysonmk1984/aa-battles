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

#[cfg(test)]
mod tests {
    use crate::{battle::tick::phases::attack::dodge::try_dodge, enums::StartingDirection};
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
}
