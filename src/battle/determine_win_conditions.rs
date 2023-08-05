use crate::BattleState;

use super::battle::{BattleResult, Belligerent, WinType};

pub fn check_for_king_captured_condition(battle_state: &BattleState) -> Option<Belligerent> {
    let a1_battalion_passed_all_opponents = battle_state
        .army_1_state
        .iter()
        .find(|b| (b.position >= 150) && b.flying == false);

    let a2_battalion_passed_all_opponents = battle_state
        .army_2_state
        .iter()
        .find(|b| (b.position <= -150) && b.flying == false);

    if a1_battalion_passed_all_opponents.is_some() {
        Some(Belligerent::WesternArmy)
    } else if a2_battalion_passed_all_opponents.is_some() {
        Some(Belligerent::EasternArmy)
    } else {
        None
    }
}

pub fn determine_army_conquered_condition(
    mut battle_result: BattleResult,
    a1_count: i32,
    a2_count: i32,
) -> BattleResult {
    if a1_count > a2_count {
        battle_result.winner = Some(Belligerent::WesternArmy);
        battle_result.loser = Some(Belligerent::EasternArmy);
    } else {
        battle_result.winner = Some(Belligerent::EasternArmy);
        battle_result.loser = Some(Belligerent::WesternArmy);
    }
    battle_result.win_type = Some(WinType::ArmyConquered);
    battle_result
}

#[cfg(test)]
mod tests {
    use crate::{
        battle::battle::{BattleResult, Belligerent, WinType},
        match_up::create_mocks::{create_mock_generic_battalion, PartialBattalionForTests},
        BattleState,
    };

    use super::{check_for_king_captured_condition, determine_army_conquered_condition};

    #[test]
    fn test_determine_army_conquered_condition_west() {
        let mut battle_result: BattleResult = Default::default();

        let updated_battle_result = determine_army_conquered_condition(battle_result, 1000, 0);
        assert_eq!(updated_battle_result.winner, Some(Belligerent::WesternArmy));
        assert_eq!(updated_battle_result.loser, Some(Belligerent::EasternArmy));
        assert_eq!(updated_battle_result.win_type, Some(WinType::ArmyConquered));
    }

    #[test]
    fn test_determine_army_conquered_condition_east() {
        let mut battle_result: BattleResult = Default::default();

        let updated_battle_result = determine_army_conquered_condition(battle_result, 0, 1000);
        assert_eq!(updated_battle_result.winner, Some(Belligerent::EasternArmy));
        assert_eq!(updated_battle_result.loser, Some(Belligerent::WesternArmy));
        assert_eq!(updated_battle_result.win_type, Some(WinType::ArmyConquered));
    }

    #[test]
    fn test_check_for_king_captured_condition_west_win() {
        // Ground Army that can't hit air
        let mock_partial_battalion_1 = PartialBattalionForTests {
            count: Some(1000),
            position: Some(150),
            speed: None,
            flying: Some(false),
            range: None,
        };
        // Air Army
        let mock_partial_battalion_2 = PartialBattalionForTests {
            count: Some(1),
            position: Some(-100),
            speed: None,
            flying: Some(true),
            range: None,
        };

        let battle_state = BattleState {
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],

            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent = check_for_king_captured_condition(&battle_state).unwrap();
        assert_eq!(belligerent, Belligerent::WesternArmy);
    }

    #[test]
    fn test_check_for_king_captured_condition_east_win() {
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

        let battle_state = BattleState {
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],
            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent = check_for_king_captured_condition(&battle_state).unwrap();
        assert_eq!(belligerent, Belligerent::EasternArmy);
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

        let battle_state = BattleState {
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
        let battle_state = BattleState {
            // Air Army
            army_1_state: vec![create_mock_generic_battalion(mock_partial_battalion_1)],
            // Ground Army that can't hit air
            army_2_state: vec![create_mock_generic_battalion(mock_partial_battalion_2)],
        };

        let belligerent_option = check_for_king_captured_condition(&battle_state);
        assert_eq!(belligerent_option, None);
    }
}
