use super::phases::attack::attack_phase;
use super::phases::march::march_phase;
use super::phases::range_find::update_in_range_map;
use crate::types::{ArmyName, StartingDirection};
use crate::Battle;
use std::collections::HashMap;

pub fn run_tick(battle_state: &mut Battle) -> i32 {
    let mut in_range_map_1: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
    let mut in_range_map_2: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();

    battle_state.army_1_state.iter().for_each(|army| {
        in_range_map_1.insert(army.name.clone(), Vec::new());
    });

    battle_state.army_2_state.iter().for_each(|army| {
        in_range_map_2.insert(army.name.clone(), Vec::new());
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

    // STEP 3: Adjust Counts
    let mut western_army_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut eastern_army_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });

    // STEP 4: March forward
    if western_army_count > 0 && eastern_army_count >= 0 {
        march_phase(&mut battle_state.army_1_state, &StartingDirection::WEST);
        march_phase(&mut battle_state.army_2_state, &StartingDirection::EAST);
    }

    let total_combined_count = western_army_count + eastern_army_count;
    total_combined_count
}

#[cfg(test)]
mod tests {
    use crate::battle::tick::phases::attack::attack_phase;
    use crate::battle::tick::phases::march::march_phase;
    use crate::battle::tick::phases::range_find::update_in_range_map;
    use crate::match_up::create_mocks::{create_mock_army, create_mock_army_defaults};
    use crate::types::ArmyName::{
        self, AmazonianHuntresses, AvianCliffDwellers, BarbariansOfTheOuterSteppe,
        CastlegateCrossbowmen, DeathDealerAssassin, ElvenArchers, HighbornCavalry,
        ImperialLegionnaires, MagiEnforcers, MinuteMenMilitia, NorthWatchLongbowmen,
        OathSwornKnights, PeacekeeperMonks, RoninImmortals, ShinobiMartialArtists,
        SkullClanDeathCultists,
    };
    use crate::types::StartingDirection;
    use crate::util::set_weapon_armor_hash;
    use std::sync::OnceLock;
    use std::{collections::HashMap, env};
    pub static WEAPON_ARMOR_CELL: OnceLock<HashMap<&str, f64>> = OnceLock::new();

    #[test]
    fn test_update_in_range_map_in_range() {
        set_weapon_armor_hash();
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        set_weapon_armor_hash();
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        let army_defaults = create_mock_army_defaults(None);
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
        let army_defaults = create_mock_army_defaults(None);
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
        defender[0].count = 0;
        defender[1].position = 0;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        assert_eq!(attacker_map.get(&HighbornCavalry).unwrap().len(), 1);
    }

    #[test]
    fn test_update_in_range_map_flying_in_front() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = true;
        attacker[0].position = -50;
        defender[0].position = 0;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);
        assert!(cloned_attacker[0].is_marching == false);
    }

    /**
     * attack_phase
     * Should start marching when there's no defender in range
     */
    #[test]
    fn test_attack_phase_march() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = false;
        attacker[0].position = -150;
        defender[0].position = 150;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);
        assert!(cloned_attacker[0].is_marching == true);
    }

    /**
     * attack_phase
     * The army count of the defender should be less than what it started at after being attacked by 50 men
     */
    #[test]
    fn test_attack_phase_count_change() {
        set_weapon_armor_hash();
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = false;
        attacker[0].position = -50;
        attacker[0].count = 50;
        defender[0].position = 0;
        defender[0].count = 50;

        attacker_map.insert(attacker[0].name.clone(), Vec::new());

        update_in_range_map(&mut attacker_map, &attacker, &defender);

        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();

        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        assert!(defender[0].count > cloned_defender[0].count);
    }

    /**
     * attack_phase
     * Will panic due to .75 agility + .25 marching bonus making defender never hittable
     */
    #[ignore]
    #[test]
    #[should_panic]
    fn test_attack_phase_should_dodge_all_with_max_agility_and_marching() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = false;
        attacker[0].position = -50;
        attacker[0].count = 50;
        defender[0].position = 0;
        defender[0].count = 50;
        defender[0].agility = 0.75;
        defender[0].is_marching = true;
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
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = false;
        attacker[0].position = -50;
        attacker[0].count = 50;
        defender[0].position = 0;
        defender[0].count = 50;
        defender[0].shield_rating = 1.0;
        defender[0].is_marching = false;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        assert_eq!(defender[0].count, cloned_defender[0].count);
    }

    /**
     * attack_phase
     * The army count of the defender should be equal to what it started with if not in range of attacker
     */
    #[test]
    fn test_attack_phase_defender_count_should_not_change_if_not_in_range() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = false;
        attacker[0].position = -150;
        attacker[0].count = 50;
        defender[0].position = 150;
        defender[0].count = 50;
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
        let army_defaults = create_mock_army_defaults(None);
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
        attacker[0].is_marching = false;
        attacker[0].position = -150;
        defender[0].position = 150;
        attacker_map.insert(attacker[0].name.clone(), Vec::new());
        update_in_range_map(&mut attacker_map, &attacker, &defender);
        let mut cloned_attacker = attacker.clone();
        let mut cloned_defender = defender.clone();
        attack_phase(&attacker_map, &mut cloned_attacker, &mut cloned_defender);

        march_phase(&mut cloned_attacker, &StartingDirection::WEST);
        assert_eq!(cloned_attacker[0].is_marching, true);
        assert_eq!(
            cloned_attacker[0].position,
            attacker[0].position + attacker[0].speed
        );
    }
}
