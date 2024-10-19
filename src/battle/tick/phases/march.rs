use crate::{entities::battalion::battalion::Battalion, enums::StartingDirection};

pub fn march_phase(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        println!("{} is marching {}", a.name, a.is_marching.get());
        if a.is_marching.get() && a.count.get() > 0 {
            let opposite_direction = if *starting_direction == StartingDirection::EAST {
                StartingDirection::WEST
            } else {
                StartingDirection::EAST
            };

            let marching_direction = if a.is_reverse_direction {
                opposite_direction
            } else {
                *starting_direction
            };
            a.march(marching_direction)
        }
    })
}

/**
 * Determines if fliers are passed all enemies, if so, it reverses the direction
 */
pub fn handle_direction_check(
    army: &mut Vec<Battalion>,
    enemy_army: &mut Vec<Battalion>,
    starting_direction: StartingDirection,
) {
    let mut fliers = army
        .iter_mut()
        .filter(|b| b.flying)
        .collect::<Vec<&mut Battalion>>();

    if fliers.len() > 0 {
        if starting_direction == StartingDirection::EAST {
            // determine if past all enemy battalions
            fliers.iter_mut().for_each(|b| {
                // find furthest defender position
                // enemy position will be going down
                let enemy_position = enemy_army.iter().fold(-150, |acc, cur| {
                    if cur.position > acc {
                        return cur.position;
                    }
                    acc
                });
                println!("{enemy_position}");

                if b.position > enemy_position && b.is_reverse_direction == false {
                    b.set_is_reverse_direction(true);
                }
            });
        } else {
            // determine if past all enemy battalions
            // enemy position will be going up
            fliers.iter_mut().for_each(|b| {
                // find furthest defender position
                let enemy_position = enemy_army.iter().fold(150, |acc, cur| {
                    if cur.position < acc {
                        return cur.position;
                    }
                    acc
                });

                if b.position < enemy_position && b.is_reverse_direction == false {
                    b.set_is_reverse_direction(true);
                }
            });
        }
    }
}

#[cfg(test)]
mod test {

    use crate::battle::tick::phases::attack::attack_phase::attack_phase;
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
    use crate::util::{map_army_defaults, WEAPON_ARMOR_CELL};

    use std::{collections::HashMap, env};

    use super::handle_direction_check;

    /**
     * march_phase
     * Should stop marching in order to attack when there's a defender in range
     */
    #[test]
    fn test_direction_change() {
        let mut attacker_map: HashMap<ArmyName, Vec<ArmyName>> = HashMap::new();
        let army_defaults = map_army_defaults(None);

        let mut attacker = create_mock_army(
            StartingDirection::EAST,
            &army_defaults,
            vec![AvianCliffDwellers],
        )
        .unwrap();
        let mut defender = create_mock_army(
            StartingDirection::WEST,
            &army_defaults,
            vec![HighbornCavalry],
        )
        .unwrap();
        attacker[0].position = 0;
        defender[0].position = -20;

        assert_eq!(attacker[0].is_reverse_direction, false);
        handle_direction_check(&mut attacker, &mut defender, StartingDirection::EAST);
        assert_eq!(attacker[0].is_reverse_direction, true);
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
}
