use super::phases::march::march_phase;
use super::phases::range_find::update_in_range_map;

use crate::battle::tick::sync_attack::sync_attack;
use crate::enums::{ArmyName, StartingDirection};
use crate::Battle;
use std::collections::HashMap;
use std::thread::spawn;

pub fn run_tick(battle_state: &mut Battle) -> u32 {
    let mut in_range_map_east: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
    let mut in_range_map_west: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();

    battle_state.army_1_state.iter().for_each(|army| {
        in_range_map_east.insert(army.name.clone(), Vec::new());
    });

    battle_state.army_2_state.iter().for_each(|army| {
        in_range_map_west.insert(army.name.clone(), Vec::new());
    });

    // STEP 1: Check for range
    let east_marchers = update_in_range_map(
        &mut in_range_map_east,
        &battle_state.army_1_state,
        &battle_state.army_2_state,
    );
    let west_marchers = update_in_range_map(
        &mut in_range_map_west,
        &battle_state.army_2_state,
        &battle_state.army_1_state,
    );

    let cloned_state = battle_state.clone();

    // STEP 2: Do attacks (both sides)
    let new_state = sync_attack(&in_range_map_east, &in_range_map_west, cloned_state);
    let is_m = (
        &new_state.army_1_state[0].is_marching.get(),
        &new_state.army_2_state[0].is_marching.get(),
    );

    // why are they marching already here?
    println!("{is_m:?}");

    // Update state with result of sync_attack
    battle_state.army_1_state = new_state.army_1_state;
    battle_state.army_2_state = new_state.army_2_state;

    // STEP 3: Adjust Counts
    let mut eastern_army_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count.get();
        sum
    });
    let mut western_army_count = battle_state.army_2_state.iter().fold(0, |mut sum, b| {
        sum += b.count.get();
        sum
    });
    println!("{western_army_count}, {eastern_army_count}");
    println!("------- March Phase ------");
    // STEP 4: March forward
    if western_army_count > 0 && eastern_army_count >= 0 {
        march_phase(&mut battle_state.army_1_state, &StartingDirection::EAST);
        march_phase(&mut battle_state.army_2_state, &StartingDirection::WEST);
    }

    let total_combined_count = western_army_count + eastern_army_count;
    total_combined_count.into()
}

#[cfg(test)]
mod tests {

    use crate::battle::tick::phases::attack::attack_phase;
    use crate::battle::tick::phases::march::march_phase;
    use crate::battle::tick::phases::range_find::update_in_range_map;
    use crate::match_up::create_mocks::create_mock_army;
    use crate::mocks::game_defaults::GameDefaultsMocks;

    use crate::enums::ArmyName::{
        self, AmazonianHuntresses, AvianCliffDwellers, BarbariansOfTheOuterSteppe,
        CastlegateCrossbowmen, DeathDealerAssassins, ElvenArchers, HighbornCavalry,
        ImperialLegionnaires, MagiEnforcers, MinuteMenMilitia, NorthWatchLongbowmen,
        OathSwornKnights, PeacekeeperMonks, RoninImmortals, ShinobiMartialArtists,
        SkullClanDeathCultists,
    };
    use crate::enums::StartingDirection;
    use crate::util::{map_army_defaults, AOE_SPREAD_CELL, WEAPON_ARMOR_CELL};

    use std::sync::OnceLock;
    use std::{collections::HashMap, env};

    #[test]
    fn test_update_in_range_map_in_range() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![PeacekeeperMonks],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![HighbornCavalry],
        )
        .unwrap();
        attacker[0].position = 0;
        defender[0].position = 0;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);

        assert_eq!(attacker_map.get(&PeacekeeperMonks).unwrap().len(), 1);
    }

    #[test]
    fn test_update_in_range_map_none_in_range() {
        WEAPON_ARMOR_CELL.set(GameDefaultsMocks::generate_weapon_armor_hash());
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![HighbornCavalry],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![NorthWatchLongbowmen],
        )
        .unwrap();
        attacker[0].position = -150;
        defender[0].position = 150;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        assert_eq!(attacker_map.get(&HighbornCavalry).unwrap().len(), 0);
    }

    #[test]
    fn test_update_in_range_map_air_not_in_range_of_melee() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![HighbornCavalry],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![AvianCliffDwellers],
        )
        .unwrap();
        attacker[0].position = 0;
        defender[0].position = 0;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        assert_eq!(attacker_map.get(&HighbornCavalry).unwrap().len(), 0);
    }

    #[test]
    fn test_update_in_range_map_zero_count_excluded() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);
        let mut attacker = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![HighbornCavalry],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![PeacekeeperMonks, ImperialLegionnaires],
        )
        .unwrap();
        attacker[0].position = 0;
        defender[0].position = 0;
        defender[0].count.set(0);
        defender[1].position = 0;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        assert_eq!(attacker_map.get(&HighbornCavalry).unwrap().len(), 1);
    }

    #[test]
    fn test_update_in_range_map_flying_in_front() {
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
            vec![
                NorthWatchLongbowmen,
                PeacekeeperMonks,
                AvianCliffDwellers,
                ImperialLegionnaires,
            ],
        )
        .unwrap();
        attacker[0].position = 0;
        defender[0].position = 0;
        defender[1].position = 0;
        defender[2].position = 0;
        defender[3].position = 0;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let vec_of_in_range = attacker_map.get(&NorthWatchLongbowmen).unwrap();
        assert_eq!(vec_of_in_range.len(), 4);
        assert!(vec_of_in_range[0] == AvianCliffDwellers);
    }

    /**
     * attack_phase
     * Should stop marching in order to attack when there's a defender in range
     */
    #[test]
    fn test_attack_phase_no_march() {
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
            vec![ImperialLegionnaires],
        )
        .unwrap();
        attacker[0].is_marching.set(true);
        attacker[0].position = -50;
        defender[0].position = 0;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);
        assert!(cloned_attacker[0].is_marching.get() == false);
    }

    /**
     * attack_phase
     * Should start marching when there's no defender in range
     */
    #[test]
    fn test_attack_phase_march() {
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
            vec![ImperialLegionnaires],
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
        assert!(cloned_attacker[0].is_marching.get() == true);
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
