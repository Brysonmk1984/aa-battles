use std::default;
use strum_macros::{Display, EnumString};

use crate::{
    types::{BattleResult, Belligerent, WinType},
    BattleState,
};

use super::determine_win_conditions::{
    check_for_king_captured_condition, determine_army_conquered_condition,
};
use super::tick::run_tick::run_tick;

pub fn run_battle(battle_state: &mut BattleState) -> BattleResult {
    let mut a1_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut a2_count = battle_state.army_2_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut total_army_count = a1_count + a2_count;

    let mut battle_result = BattleResult {
        id: 1,
        winner: None,
        loser: None,
        tick_count: 0,
        win_type: None,
    };

    while a1_count > 0 && a2_count > 0 {
        let winner_by_position = check_for_king_captured_condition(&battle_state);
        if winner_by_position.is_some() {
            //dbg!(&winner_by_position);
            battle_result.win_type = Some(WinType::KingCaptured);
            battle_result.loser =
                if winner_by_position.as_ref().unwrap() == &Belligerent::WesternArmy {
                    Some(Belligerent::EasternArmy)
                } else {
                    Some(Belligerent::WesternArmy)
                };
            battle_result.winner = winner_by_position;
            return battle_result;
        }

        a1_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
            sum += b.count;
            sum
        });
        a2_count = battle_state.army_2_state.iter().fold(0, |mut sum, b| {
            sum += b.count;
            sum
        });
        //println!("WEST ARMY COUNT: {a1_count}, EAST ARMY COUNT: {a2_count}");

        battle_result.tick_count += 1;
        if battle_result.tick_count > 300 {
            panic!("Infinite loop detected!");
        }
        total_army_count = run_tick(battle_state);
    }

    determine_army_conquered_condition(battle_result, a1_count, a2_count)
}
