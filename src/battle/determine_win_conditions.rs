use crate::{
    entities::{
        battalion::battalion::Battalion, battle_result::battle_result::BattleResult,
        ending_battalion_stats::EndingBattalionStats,
    },
    enums::{ArmyName, Belligerent, WinType},
    util::push_log,
    Battle,
};
use anyhow::Context;
use std::sync::atomic::{AtomicU32, Ordering};

pub fn check_for_king_captured_condition(battle_state: &Battle) -> Option<Belligerent> {
    let eastern_army_reached_enemy_king = battle_state
        .army_1_state
        .iter()
        .find(|b| (b.position >= 150) && b.flying == false);

    let western_army_reached_enemy_king = battle_state
        .army_2_state
        .iter()
        .find(|b| (b.position <= -150) && b.flying == false);

    if eastern_army_reached_enemy_king.is_some() {
        push_log(format!("THE BATTLE ENDS: The Eastern army has passed all enemies and captured the Western army's King with a battalion of {}!", eastern_army_reached_enemy_king.unwrap().name));
        Some(Belligerent::EasternArmy)
    } else if western_army_reached_enemy_king.is_some() {
        push_log(format!("THE BATTLE ENDS: Western army has passed all enemies and captured the Eastern army's King with a battalion of {}!", western_army_reached_enemy_king.unwrap().name));
        Some(Belligerent::WesternArmy)
    } else {
        None
    }
}

pub fn determine_army_conquered_condition(
    ending_army_states: (&Vec<Battalion>, &Vec<Battalion>),
    mut battle_result: BattleResult,
    eastern_count: u32,
    western_count: u32,
) -> BattleResult {
    if eastern_count > western_count {
        push_log(
            "THE BATTLE ENDS: Eastern Army has defeated all of the Western Army's forces!"
                .to_string(),
        );
        battle_result.winner = Some(Belligerent::EasternArmy);
        battle_result.loser = Some(Belligerent::WesternArmy);
    } else if eastern_count < western_count {
        push_log(
            "THE BATTLE ENDS: Western Army has defeated all of the Eastern Army's forces!"
                .to_string(),
        );
        battle_result.winner = Some(Belligerent::WesternArmy);
        battle_result.loser = Some(Belligerent::EasternArmy);
    } else {
        panic!("Need to figure this out. A tie where both armies are zero");
    }

    battle_result.eastern_battalions = ending_army_states
        .0
        .iter()
        .map(|battalion| EndingBattalionStats {
            name: battalion.name,
            count: battalion.count.get() as i32,
            position: battalion.position,
        })
        .collect();

    battle_result.western_battalions = ending_army_states
        .1
        .iter()
        .map(|battalion| EndingBattalionStats {
            name: battalion.name,
            count: battalion.count.get() as i32,
            position: battalion.position,
        })
        .collect();

    battle_result.win_type = Some(WinType::ArmyConquered);
    battle_result
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicU32;

    use super::{check_for_king_captured_condition, determine_army_conquered_condition};
    use crate::{
        entities::{
            battle::battle::Battle, battle_result::battle_result::BattleResult,
            testing_entities::partial_battalion_for_testing::PartialBattalionForTests,
        },
        enums::{Belligerent, StartingDirection, WinType},
        match_up::create_mocks::create_mock_generic_battalion,
    };

    #[test]
    fn test_determine_army_conquered_condition_east() {
        let mut battle_result: BattleResult = Default::default();
        let east = vec![create_mock_generic_battalion(PartialBattalionForTests {
            count: Some(1000),
            starting_direction: Some((StartingDirection::EAST)),
            ..Default::default()
        })];
        let west = vec![create_mock_generic_battalion(PartialBattalionForTests {
            count: Some(0),
            starting_direction: Some((StartingDirection::EAST)),
            ..Default::default()
        })];

        let updated_battle_result =
            determine_army_conquered_condition((&east, &west), battle_result, 1000, 0);
        assert_eq!(updated_battle_result.winner, Some(Belligerent::EasternArmy));
        assert_eq!(updated_battle_result.loser, Some(Belligerent::WesternArmy));
        assert_eq!(updated_battle_result.win_type, Some(WinType::ArmyConquered));
    }

    #[test]
    fn test_determine_army_conquered_condition_west() {
        let mut battle_result: BattleResult = Default::default();
        let east = vec![create_mock_generic_battalion(PartialBattalionForTests {
            count: Some(0),
            starting_direction: Some((StartingDirection::EAST)),
            ..Default::default()
        })];
        let west = vec![create_mock_generic_battalion(PartialBattalionForTests {
            count: Some(1000),
            starting_direction: Some((StartingDirection::EAST)),
            ..Default::default()
        })];

        let updated_battle_result =
            determine_army_conquered_condition((&east, &west), battle_result, 0, 1000);
        assert_eq!(updated_battle_result.winner, Some(Belligerent::WesternArmy));
        assert_eq!(updated_battle_result.loser, Some(Belligerent::EasternArmy));
        assert_eq!(updated_battle_result.win_type, Some(WinType::ArmyConquered));
    }

    #[test]
    fn test_check_for_king_captured_condition_east_win() {
        // Ground Army that can't hit air
        let mock_partial_battalion_1 = PartialBattalionForTests {
            count: Some(1000),
            position: Some(150),
            speed: None,
            flying: Some(false),
            range: None,
            aoe: None,
            spread: None,
            starting_direction: None,
        };
        // Air Army
        let mock_partial_battalion_2 = PartialBattalionForTests {
            count: Some(1),
            position: Some(-100),
            speed: None,
            flying: Some(true),
            range: None,
            aoe: None,
            spread: None,
            starting_direction: None,
        };

        let battle_state = Battle {
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],

            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent = check_for_king_captured_condition(&battle_state).unwrap();

        assert_eq!(belligerent, Belligerent::EasternArmy);
    }

    #[test]
    fn test_check_for_king_captured_condition_west_win() {
        // Air Army
        let mock_partial_battalion_1 = PartialBattalionForTests {
            count: Some(1),
            position: Some(-100),
            flying: Some(true),
            ..Default::default()
        };
        // Ground Army that can't hit air
        let mock_partial_battalion_2 = PartialBattalionForTests {
            count: Some(1000),
            position: Some(-150),
            flying: Some(false),
            ..Default::default()
        };

        let battle_state = Battle {
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],
            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent = check_for_king_captured_condition(&battle_state).unwrap();
        assert_eq!(belligerent, Belligerent::WesternArmy);
    }

    #[test]
    fn test_check_for_king_captured_condition_none() {
        // Air Army
        let mock_partial_battalion_1 = PartialBattalionForTests {
            count: Some(1),
            position: Some(-100),
            flying: Some(true),
            ..Default::default()
        };
        // Ground Army that can't hit air
        let mock_partial_battalion_2 = PartialBattalionForTests {
            count: Some(1000),
            position: Some(-120),
            flying: Some(false),
            ..Default::default()
        };

        let battle_state = Battle {
            // Air Army
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],
            // Ground Army that can't hit air
            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent_option = check_for_king_captured_condition(&battle_state);
        assert_eq!(belligerent_option, None);
    }

    #[test]
    fn test_check_for_king_captured_condition_air_no_capture() {
        // Air Army
        let mock_partial_battalion_1 = PartialBattalionForTests {
            count: Some(1),
            position: Some(-150),
            flying: Some(false),
            ..Default::default()
        };
        // Ground Army that can't hit air
        let mock_partial_battalion_2 = PartialBattalionForTests {
            count: Some(1000),
            position: Some(-120),
            flying: Some(false),
            ..Default::default()
        };
        let battle_state = Battle {
            // Air Army
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],
            // Ground Army that can't hit air
            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent_option = check_for_king_captured_condition(&battle_state);
        assert_eq!(belligerent_option, None);
    }
}
