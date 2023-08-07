use super::attack_phase::attack::attack_phase;
use super::march::march_phase;
use super::range_find::update_in_range_map;
use crate::match_up::match_up::{Battalion, StartingDirection};
use crate::BattleState;
use std::collections::HashMap;

pub fn run_tick(battle_state: &mut BattleState, total_combined_count: i32) -> i32 {
    let mut in_range_map_1: HashMap<String, Vec<&str>> = HashMap::new();
    let mut in_range_map_2: HashMap<String, Vec<&str>> = HashMap::new();

    battle_state.army_1_state.iter().for_each(|army| {
        in_range_map_1.insert(army.name.clone(), Vec::new());
    });

    battle_state.army_2_state.iter().for_each(|army| {
        in_range_map_2.insert(army.name.clone(), vec![]);
    });

    // TODO: Figure out way to handle this where cloning isn't needed to satisfy borrow checker
    let army_1_clone = battle_state.army_1_state.clone();
    let army_2_clone = battle_state.army_2_state.clone();

    // STEP 1: Check for range
    update_in_range_map(&mut in_range_map_1, &army_1_clone, &army_2_clone);
    update_in_range_map(&mut in_range_map_2, &army_2_clone, &army_1_clone);

    // STEP 2: Attack Battalions within range
    // STEP 2a: army_1 Attacks army_2 (Concurrently with step 2b)
    attack_phase(
        &in_range_map_1,
        &mut battle_state.army_1_state,
        &mut battle_state.army_2_state,
    );
    // STEP 2b: army_2 Attacks army_1 (Concurrently with step 2a)
    attack_phase(
        &in_range_map_2,
        &mut battle_state.army_2_state,
        &mut battle_state.army_1_state,
    );

    // STEP 3: March forward
    march_phase(&mut battle_state.army_1_state, &StartingDirection::WEST);
    march_phase(&mut battle_state.army_2_state, &StartingDirection::EAST);

    //println!("{in_range_map_1:?} \n\n {in_range_map_2:?}");

    let mut a1 = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut a2 = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let new_total = a1 + a2;

    new_total
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use crate::{
        battle::tick::range_find::update_in_range_map,
        match_up::{
            create_mocks::{create_mock_army, create_mock_army_defaults},
            match_up::StartingDirection,
        },
    };

    #[test]
    fn test_update_in_range_map_in_range() {
        temp_env::with_var("MIN_RANGE_ATTACK_AIR", Some("15"), || {
            let mut attacker_map: HashMap<String, Vec<&str>> = HashMap::new();
            let army_defaults = create_mock_army_defaults(None);
            let mut attacker = create_mock_army(
                StartingDirection::WEST,
                &army_defaults,
                vec!["highborn_cavalry"],
            );
            let mut defender = create_mock_army(
                StartingDirection::EAST,
                &army_defaults,
                vec!["north_watch_longbowmen"],
            );
            attacker[0].position = 0;
            defender[0].position = 0;

            attacker_map.insert(attacker[0].name.clone(), Vec::new());
            update_in_range_map(&mut attacker_map, &attacker, &defender);
            assert_eq!(attacker_map.get("Highborn Cavalry").unwrap().len(), 1);
        });
    }

    #[test]
    fn test_update_in_range_map_none_in_range() {
        temp_env::with_var("MIN_RANGE_ATTACK_AIR", Some("15"), || {
            let mut attacker_map: HashMap<String, Vec<&str>> = HashMap::new();
            let army_defaults = create_mock_army_defaults(None);
            let mut attacker = create_mock_army(
                StartingDirection::WEST,
                &army_defaults,
                vec!["highborn_cavalry"],
            );
            let mut defender = create_mock_army(
                StartingDirection::EAST,
                &army_defaults,
                vec!["north_watch_longbowmen"],
            );
            attacker[0].position = -150;
            defender[0].position = 150;

            attacker_map.insert(attacker[0].name.clone(), Vec::new());
            update_in_range_map(&mut attacker_map, &attacker, &defender);
            assert_eq!(attacker_map.get("Highborn Cavalry").unwrap().len(), 0);
        });
    }

    #[test]
    fn test_update_in_range_map_air_not_in_range_of_melee() {
        temp_env::with_var("MIN_RANGE_ATTACK_AIR", Some("15"), || {
            let mut attacker_map: HashMap<String, Vec<&str>> = HashMap::new();
            let army_defaults = create_mock_army_defaults(None);
            let mut attacker = create_mock_army(
                StartingDirection::WEST,
                &army_defaults,
                vec!["highborn_cavalry"],
            );
            let mut defender = create_mock_army(
                StartingDirection::EAST,
                &army_defaults,
                vec!["avian_cliff_dwellers"],
            );
            attacker[0].position = 0;
            defender[0].position = 0;

            attacker_map.insert(attacker[0].name.clone(), Vec::new());
            update_in_range_map(&mut attacker_map, &attacker, &defender);
            assert_eq!(attacker_map.get("Highborn Cavalry").unwrap().len(), 0);
        });
    }

    #[test]
    fn test_update_in_range_map_zero_count_excluded() {
        temp_env::with_var("MIN_RANGE_ATTACK_AIR", Some("15"), || {
            let mut attacker_map: HashMap<String, Vec<&str>> = HashMap::new();
            let army_defaults = create_mock_army_defaults(None);
            let mut attacker = create_mock_army(
                StartingDirection::WEST,
                &army_defaults,
                vec!["highborn_cavalry"],
            );
            let mut defender = create_mock_army(
                StartingDirection::EAST,
                &army_defaults,
                vec!["peacekeeper_monks", "imperial_legionnaires"],
            );
            attacker[0].position = 0;
            defender[0].position = 0;
            defender[0].count = 0;
            defender[1].position = 0;

            attacker_map.insert(attacker[0].name.clone(), Vec::new());
            update_in_range_map(&mut attacker_map, &attacker, &defender);
            assert_eq!(attacker_map.get("Highborn Cavalry").unwrap().len(), 1);
        });
    }

    #[test]
    fn test_update_in_range_map_flying_in_front() {
        temp_env::with_var("MIN_RANGE_ATTACK_AIR", Some("15"), || {
            let mut attacker_map: HashMap<String, Vec<&str>> = HashMap::new();
            let army_defaults = create_mock_army_defaults(None);
            let mut attacker = create_mock_army(
                StartingDirection::WEST,
                &army_defaults,
                vec!["north_watch_longbowmen"],
            );
            let mut defender = create_mock_army(
                StartingDirection::EAST,
                &army_defaults,
                vec![
                    "north_watch_longbowmen",
                    "peacekeeper_monks",
                    "avian_cliff_dwellers",
                    "imperial_legionnaires",
                ],
            );
            attacker[0].position = 0;
            defender[0].position = 0;
            defender[1].position = 0;
            defender[2].position = 0;
            defender[3].position = 0;

            attacker_map.insert(attacker[0].name.clone(), Vec::new());
            update_in_range_map(&mut attacker_map, &attacker, &defender);
            let vec_of_in_range = attacker_map.get("North Watch Longbowmen").unwrap();
            assert_eq!(vec_of_in_range.len(), 4);
            assert!(vec_of_in_range[0] == "Avian Cliff Dwellers");
        });
    }
}
