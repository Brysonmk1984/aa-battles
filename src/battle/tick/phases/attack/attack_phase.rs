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

use super::{armor_defense::try_armor_defense, block::try_block, dodge::try_dodge};

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
            println!("ENGAGEMENT OUTCOME {:?}", result);
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

#[cfg(test)]
mod tests {

    use crate::battle::tick::phases::attack::attack_phase::attack_phase;
    use crate::battle::tick::phases::march::march_phase;
    use crate::battle::tick::phases::range_find::update_in_range_map;
    use crate::match_up::create_mocks::create_mock_army;
    use crate::mocks::game_defaults::{self, GameDefaultsMocks};

    use crate::enums::ArmyName::{
        self, AmazonianHuntresses, AvianCliffDwellers, BarbariansOfTheOuterSteppe,
        CastlegateCrossbowmen, DeathDealerAssassins, ElvenArchers, HighbornCavalry,
        ImperialLegionnaires, MagiEnforcers, MinuteMenMilitia, NorthWatchLongbowmen,
        OathSwornKnights, PeacekeeperMonks, RoninImmortals, ShinobiMartialArtists,
        SkullClanDeathCultists,
    };
    use crate::enums::{ArmorType, StartingDirection};
    use crate::util::{map_army_defaults, AOE_SPREAD_CELL, WEAPON_ARMOR_CELL};

    use std::{collections::HashMap, env};

    /**
     * attack_phase
     * The attacking battalion should hit all defenders : magic vs chain w/1 spread
     */
    #[test]
    fn test_aoe_attack() {
        dotenvy::dotenv().ok();
        WEAPON_ARMOR_CELL.set(GameDefaultsMocks::generate_weapon_armor_hash());
        AOE_SPREAD_CELL.set(GameDefaultsMocks::generate_aoe_spread_hash());
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![SkullClanDeathCultists],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![CastlegateCrossbowmen],
        )
        .unwrap();

        attacker[0].position = -20;
        attacker[0].count.set(1);
        attacker[0].accuracy = 1.0;
        defender[0].position = 0;
        defender[0].count.set(34);
        defender[0].armor_type = ArmorType::Chain;
        defender[0].is_marching.set(false);
        defender[0].agility = 0.0;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());

        assert_eq!(defender[0].count.get(), 34);
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        // Still 1 in 100 chance to dodge
        assert!(cloned_defender[0].count.get() == 1 || cloned_defender[0].count.get() == 34);
    }

    /**
     * attack_phase
     * The attacking battalion should hit multiple times
     */
    #[test]
    fn test_attack_speed() {
        dotenvy::dotenv().ok();
        WEAPON_ARMOR_CELL.set(GameDefaultsMocks::generate_weapon_armor_hash());
        AOE_SPREAD_CELL.set(GameDefaultsMocks::generate_aoe_spread_hash());
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![ShinobiMartialArtists],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![MinuteMenMilitia],
        )
        .unwrap();

        attacker[0].attack_speed = 3;
        attacker[0].position = -5;
        defender[0].position = 0;
        attacker[0].count.set(10);
        defender[0].count.set(100);
        defender[0].is_marching.set(false);
        defender[0].agility = 0.0;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());

        assert_eq!(defender[0].count.get(), 100);
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        assert!(cloned_defender[0].count.get() < 90);
    }

    /**
     * attack_phase
     * Will panic due to .75 agility + .25 marching bonus making defender never hittable
     * Removed panic from too many dodges. Don't want to actually trigger a panic in
     */
    #[test]
    //#[should_panic]
    fn test_attack_phase_should_dodge_all_with_max_agility_and_marching() {
        dotenvy::dotenv().ok();
        WEAPON_ARMOR_CELL.set(GameDefaultsMocks::generate_weapon_armor_hash());

        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![NorthWatchLongbowmen],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![PeacekeeperMonks],
        )
        .unwrap();
        attacker[0].is_marching.set(false);
        attacker[0].position = -50;
        attacker[0].count.set(50);
        defender[0].position = 0;
        defender[0].count.set(50);
        defender[0].agility = 0.75;
        defender[0].is_marching.set(true);
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);
    }

    /**
     * attack_phase
     * The army count of the defender should be equal to what it started with 1.0 shield_rating, after being attacked by 50 men
     */
    #[test]
    fn test_attack_phase_should_block_all_with_max_shield_rating() {
        dotenvy::dotenv().ok();

        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![NorthWatchLongbowmen],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![PeacekeeperMonks],
        )
        .unwrap();
        attacker[0].is_marching.set(false);
        attacker[0].position = -50;
        attacker[0].count.set(50);
        defender[0].position = 0;
        defender[0].count.set(50);
        defender[0].shield_rating = 1.0;
        defender[0].is_marching.set(false);
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        assert_eq!(defender[0].count.get(), cloned_defender[0].count.get());
    }

    /**
     * attack_phase
     * The army count of the defender should be equal to what it started with if not in range of attacker
     */
    #[test]
    fn test_attack_phase_defender_count_should_not_change_if_not_in_range() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![NorthWatchLongbowmen],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![PeacekeeperMonks],
        )
        .unwrap();
        attacker[0].is_marching.set(false);
        attacker[0].position = -150;
        attacker[0].count.set(50);
        defender[0].position = 150;
        defender[0].count.set(50);
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        assert_eq!(defender[0].count, cloned_defender[0].count);
    }

    /**
     * attack_phase
     * The army should march - changing position and is_marching if no defenders are in range
     */
    #[test]
    fn test_attack_phase_attacker_should_march_if_no_defender_in_range() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![PeacekeeperMonks],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![NorthWatchLongbowmen],
        )
        .unwrap();
        attacker[0].is_marching.set(false);
        attacker[0].position = -150;
        defender[0].position = 150;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());

        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        march_phase(&mut cloned_attacker, &StartingDirection::EAST);

        assert_eq!(cloned_attacker[0].is_marching.get(), true);
        assert_eq!(
            cloned_attacker[0].position,
            attacker[0].position + attacker[0].speed
        );
    }
}
