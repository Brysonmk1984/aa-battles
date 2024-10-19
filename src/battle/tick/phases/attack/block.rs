use crate::{
    enums::StartingDirection,
    util::{push_log, push_stat_block},
};

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

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{battle::tick::phases::attack::block::try_block, enums::StartingDirection};

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
