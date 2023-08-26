use crate::{
    types::{BattleResult, Belligerent, WinType},
    util::push_log,
    Battle,
};

pub fn check_for_king_captured_condition(battle_state: &Battle) -> Option<Belligerent> {
    let western_army_reached_enemy_king = battle_state
        .army_1_state
        .iter()
        .find(|b| (b.position >= 150) && b.flying == false);

    let eastern_army_reached_enemy_king = battle_state
        .army_2_state
        .iter()
        .find(|b| (b.position <= -150) && b.flying == false);

    if western_army_reached_enemy_king.is_some() {
        push_log(format!("The Western army has passed all enemies and captured the Eastern army's King with a battalion of {}!", western_army_reached_enemy_king.unwrap().name));
        Some(Belligerent::WesternArmy)
    } else if eastern_army_reached_enemy_king.is_some() {
        push_log(format!("The Eastern army has passed all enemies and captured the Western army's King with a battalion of {}!", eastern_army_reached_enemy_king.unwrap().name));
        Some(Belligerent::EasternArmy)
    } else {
        None
    }
}

pub fn determine_army_conquered_condition(
    mut battle_result: BattleResult,
    western_count: i32,
    eastern_count: i32,
) -> BattleResult {
    if western_count > eastern_count {
        push_log("Western Army has defeater all of the Eastern Army's forces!".to_string());
        battle_result.winner = Some(Belligerent::WesternArmy);
        battle_result.loser = Some(Belligerent::EasternArmy);
    } else {
        push_log("Eastern Army has defeater all of the Western Army's forces!".to_string());
        battle_result.winner = Some(Belligerent::EasternArmy);
        battle_result.loser = Some(Belligerent::WesternArmy);
    }
    battle_result.win_type = Some(WinType::ArmyConquered);
    battle_result
}

#[cfg(test)]
mod tests {
    use super::{check_for_king_captured_condition, determine_army_conquered_condition};
    use crate::{
        match_up::create_mocks::create_mock_generic_battalion,
        types::{Battle, BattleResult, Belligerent, PartialBattalionForTests, WinType},
    };

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
            aoe: None,
            spread: None,
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
        };

        let battle_state = Battle {
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

        let battle_state = Battle {
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
