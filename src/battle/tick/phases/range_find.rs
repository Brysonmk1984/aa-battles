use std::{collections::HashMap, env};

use rand::seq::SliceRandom;

use crate::{entities::battalion::battalion::Battalion, enums::ArmyName, MIN_RANGE_ATTACK_AIR};

pub fn update_in_range_map<'a>(
    attacker_map: &mut HashMap<ArmyName, Vec<ArmyName>>,
    attacker: &'a Vec<Battalion>,
    defender: &'a Vec<Battalion>,
) {
    // loop through army_1 and figure out which of army_2 is in range
    for (battalion_key, in_range_vec) in attacker_map {
        let mut flyer_vec: Vec<ArmyName> = Vec::new();
        let mut ground_vec: Vec<ArmyName> = Vec::new();

        // For each battalion in the defender's army, determine which are in range of the attacker
        defender.iter().for_each(|battalion| {
            let defender_position = battalion.position;
            let attacker_battalion = attacker
                .iter()
                .find(|battalion| battalion.name == *battalion_key)
                .unwrap();
            let attacker_position = attacker_battalion.position;
            let attacker_range = attacker_battalion.range;

            let distance_between_battalions = attacker_position - defender_position;

            let in_range = distance_between_battalions.abs() - attacker_range <= 0;

            // TODO: Consider a more elaborate check for range finding when both are marching and march past each other rather than attack
            // For now, resolved this by adjusting speed down and range up.
            if in_range && battalion.count.get() > 0 {
                let battalion_name = battalion.name.clone();
                // insert defenders flyers in the flyer vec, otherwise the ground vec
                if attacker_range >= MIN_RANGE_ATTACK_AIR && battalion.flying {
                    // println!(
                    //     "{} IN RANGE OF {}, CAN HIT FLYER",
                    //     attacker_battalion.name, battalion.name
                    // );
                    // In range, can hit air, and enemy is flying
                    flyer_vec.push(battalion_name);
                } else if !battalion.flying {
                    // println!(
                    //     "{} IN RANGE OF {}, A GROUND FORCE",
                    //     attacker_battalion.name, battalion.name
                    // );
                    // In range, enemy is non-flyer
                    ground_vec.push(battalion_name)
                } else {
                    // println!(
                    //     "{} IN RANGE OF {}, BUT CANNOT HIT FLYER",
                    //     attacker_battalion.name, battalion.name
                    // );
                    // In range, can't hit enemy flyer
                }
            }
        });

        // Randomly shuffle the two vecs, this will dictate priority with attacks
        flyer_vec.shuffle(&mut rand::thread_rng());
        ground_vec.shuffle(&mut rand::thread_rng());

        // Flyers will be prioritized over ground enemies
        let combined_vec = [flyer_vec, ground_vec].concat();
        println!("{combined_vec:?}");
        // push arranged, combined vec items into the in_range vec on the attacker
        combined_vec.into_iter().for_each(|b_name| {
            in_range_vec.push(b_name);
        });

        if (in_range_vec.len() == 0) {
            let attacker_battalion = attacker
                .iter()
                .find(|battalion| battalion.name == *battalion_key)
                .unwrap();
            attacker_battalion.is_marching.set(true);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::battle::tick::phases::attack::attack_phase::attack_phase;
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
}
