use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::{
    Army,
    ArmyName::{
        self, AmazonianHuntresses, AvianCliffDwellers, BarbariansOfTheOuterSteppe,
        CastlegateCrossbowmen, ElvenArchers, HighbornCavalry, HoodedAssassins,
        ImperialLegionnaires, MagiEnforcers, Militia, NorthWatchLongbowmen, OathSwornKnights,
        PeacekeeperMonks, RoninImmortals, ShinobiMartialArtists, SkullClanDeathCultists,
    },
    Battalion, BattleArmy, StartingDirection,
};

use super::create_mocks::{create_mock_army, MockError};

/**
*  fn get_battle_tuple -
   Get all battalions belonging to both nations & return as  full armies (BattleArmy)
* params - id_1 (nation Id), id_2, army_defaults (hashmap of army types, to be converted to Battalion)
*/
pub fn get_battle_tuple(
    id_1: i32,
    id_2: i32,
    army_defaults: HashMap<ArmyName, Army>,
) -> Result<(BattleArmy, BattleArmy), MockError> {
    // TODO: In the future, we need to replace this with the user's army saved in a new db table
    let full_army_west = create_mock_army(
        StartingDirection::WEST,
        &army_defaults,
        /**
         * Enter Belligerents Here
         */
        vec![PeacekeeperMonks, RoninImmortals, BarbariansOfTheOuterSteppe],
    )?;

    // TODO: In the future, we need to replace this with the user's army saved in a new db table
    let full_army_east = create_mock_army(
        StartingDirection::EAST,
        &army_defaults,
        /**
         * Enter Belligerents Here
         */
        vec![NorthWatchLongbowmen, HighbornCavalry, OathSwornKnights],
    )?;

    Ok((
        BattleArmy {
            nation_id: id_1,
            full_army: full_army_west,
        },
        BattleArmy {
            nation_id: id_2,
            full_army: full_army_east,
        },
    ))
}

impl From<&Army> for Battalion {
    fn from(a: &Army) -> Self {
        Self {
            position: 0,
            name: a.name.clone(),
            count: a.count,
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
            is_marching: true,
            starting_direction: StartingDirection::EAST,
            is_reverse_direction: false,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        match_up::create_mocks::create_mock_generic_battalion, types::PartialBattalionForTests,
    };

    use super::Battalion;

    #[test]
    fn should_march_west_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = 150;
        let original_position = test_battalion_ref.position;
        assert_eq!(original_position, 150);
        test_battalion_ref.march(super::StartingDirection::EAST);
        assert_eq!(
            test_battalion_ref.position,
            original_position - test_battalion_ref.speed
        );
    }

    #[test]
    fn should_march_east_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];
        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = -150;
        let original_position = test_battalion_ref.position;
        assert_eq!(original_position, -150);
        test_battalion_ref.march(super::StartingDirection::WEST);
        assert_eq!(
            test_battalion_ref.position,
            original_position + test_battalion_ref.speed
        );
    }

    #[test]
    fn should_decrease_count_by_one_normal_attack() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();
        let attacking_army_aoe = 0.0;
        assert_eq!(test_battalion_ref.count, 1000);
        test_battalion_ref.decrement(attacking_army_aoe);
        assert_eq!(test_battalion_ref.count, 999);
    }

    #[test]
    fn should_decrease_count_by_five_aoe_attack_normal_spread() {
        let partial_mock_battalion = PartialBattalionForTests {
            aoe: None,
            count: None,
            position: None,
            speed: None,
            flying: None,
            range: None,
            spread: Some(1.0),
        };

        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();

        assert_eq!(test_battalion_ref.count, 1000);
        test_battalion_ref.decrement(1.0);
        assert_eq!(test_battalion_ref.count, 995);
    }

    #[test]
    fn should_decrease_count_by_two_aoe_attack_extra_spread() {
        let partial_mock_battalion = PartialBattalionForTests {
            aoe: None,
            count: None,
            position: None,
            speed: None,
            flying: None,
            range: None,
            spread: Some(2.0),
        };

        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();

        assert_eq!(test_battalion_ref.count, 1000);
        test_battalion_ref.decrement(1.0);
        assert_eq!(test_battalion_ref.count, 998);
    }

    #[test]
    fn should_set_is_marching() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();

        assert_eq!(test_battalion_ref.is_marching, true);
        test_battalion_ref.set_is_marching(false);
        assert_eq!(test_battalion_ref.is_marching, false);
        test_battalion_ref.set_is_marching(true);
        assert_eq!(test_battalion_ref.is_marching, true);
    }
}
