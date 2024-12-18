use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, AtomicU32},
};

use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        army::Army, battalion::battalion::Battalion, battle_army::battle_army::BattleArmy,
        nation::Nation, nation_army::nation_army::NationArmy,
    },
    enums::{
        ArmyName::{
            self, AmazonianHuntresses, AvianCliffDwellers, BarbariansOfTheOuterSteppe,
            CastlegateCrossbowmen, DeathDealerAssassins, ElvenArchers, HighbornCavalry,
            ImperialLegionnaires, MagiEnforcers, MinuteMenMilitia, NorthWatchLongbowmen,
            OathSwornKnights, PeacekeeperMonks, RoninImmortals, ShinobiMartialArtists,
            SkullClanDeathCultists,
        },
        StartingDirection,
    },
};

use super::create_mocks::{create_mock_army, MockError};

/**
*  fn get_battle_tuple -
   Get all battalions belonging to both nations & return as  full armies (BattleArmy)
* params - id_1 (nation Id), id_2, army_defaults (hashmap of army types, to be converted to Battalion)
*/
pub fn get_battle_tuple(
    competitors: ((Nation, Vec<NationArmy>), (Nation, Vec<NationArmy>)),
    army_defaults: HashMap<ArmyName, Army>,
    battalion_merge_func: impl Fn(
        (Nation, Vec<NationArmy>),
        &HashMap<ArmyName, Army>,
        StartingDirection,
    ) -> BattleArmy,
) -> Result<(BattleArmy, BattleArmy), MockError> {
    let (east_competitor, west_competitor) = competitors;

    Ok((
        battalion_merge_func(east_competitor, &army_defaults, StartingDirection::EAST),
        battalion_merge_func(west_competitor, &army_defaults, StartingDirection::WEST),
    ))
}

/**
 * Used to merge the nation details, the nation_army details, and the default army stats together
 * into a BattleArmy, which is used throughout the battle algorithm
 */
pub fn create_battle_army(
    competitor: (Nation, Vec<NationArmy>),
    army_defaults: &HashMap<ArmyName, Army>,
    starting_direction: StartingDirection,
) -> BattleArmy {
    let (nation_details, nation_army_details) = competitor;

    nation_army_details.iter().fold(
        BattleArmy {
            nation_id: nation_details.id,
            full_army: vec![],
        },
        |mut acc, nation_army| {
            let name = nation_army.army_name;

            let army_default = army_defaults.get(&name).unwrap();

            let merged_battalion = Battalion {
                name,
                count: (nation_army.count as u32).into(),
                position: if starting_direction == StartingDirection::EAST {
                    -150
                } else {
                    150
                },
                starting_direction,
                ..Battalion::from(army_default)
            };
            acc.full_army.push(merged_battalion);
            acc
        },
    )
}

impl From<&Army> for Battalion {
    fn from(a: &Army) -> Self {
        Self {
            position: 0,
            name: a.name.clone(),
            count: (a.count as u32).into(),
            shield_rating: a.shield_rating,
            flying: a.flying,
            range: a.range,
            attack_speed: a.attack_speed,
            accuracy: a.accuracy,
            aoe: a.aoe,
            spread: a.spread,
            weapon_type: a.weapon_type,
            armor_type: a.armor_type,
            agility: a.agility,
            speed: a.speed,
            is_marching: true.into(),
            starting_direction: StartingDirection::EAST,
            is_reverse_direction: false,
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::atomic::{AtomicBool, Ordering};

    use crate::{
        entities::testing_entities::partial_battalion_for_testing::PartialBattalionForTests,
        enums::StartingDirection, match_up::create_mocks::create_mock_generic_battalion,
        mocks::game_defaults::GameDefaultsMocks, util::AOE_SPREAD_CELL,
    };

    use super::Battalion;

    #[test]
    fn should_march_west_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = -150;
        let original_position = test_battalion_ref.position;
        assert_eq!(original_position, -150);

        test_battalion_ref.march(super::StartingDirection::EAST);
        assert_eq!(
            test_battalion_ref.position,
            original_position + test_battalion_ref.speed
        );
    }

    #[test]
    fn should_march_east_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];
        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = 150;
        let original_position = test_battalion_ref.position;
        assert_eq!(original_position, 150);
        test_battalion_ref.march(super::StartingDirection::WEST);
        assert_eq!(
            test_battalion_ref.position,
            original_position - test_battalion_ref.speed
        );
    }

    #[test]
    fn should_set_is_marching() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();

        assert_eq!(test_battalion_ref.is_marching.get(), true);
        test_battalion_ref.set_is_marching(false, None);
        assert_eq!(test_battalion_ref.is_marching.get(), false);
        test_battalion_ref.set_is_marching(true, None);
        assert_eq!(test_battalion_ref.is_marching.get(), true);
    }
}

pub fn create_mock_battle_army(
    competitor: (Nation, Vec<NationArmy>),
    army_defaults: &HashMap<ArmyName, Army>,
    starting_direction: StartingDirection,
) -> BattleArmy {
    let full_army = create_mock_army(
        starting_direction,
        &army_defaults,
        /**
         * Enter Belligerents Here
         */
        if starting_direction == StartingDirection::WEST {
            vec![
                HighbornCavalry,
                ImperialLegionnaires,
                ShinobiMartialArtists,
                DeathDealerAssassins,
                AmazonianHuntresses,
            ]
        } else {
            vec![
                NorthWatchLongbowmen,
                ElvenArchers,
                CastlegateCrossbowmen,
                BarbariansOfTheOuterSteppe,
            ]
        },
    )
    .unwrap();

    BattleArmy {
        full_army,
        nation_id: competitor.0.id,
    }
}
