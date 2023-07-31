use crate::{
    match_up::match_up::{Battalion, BattleArmy},
    service::query::Army,
    BattleState,
};

use super::tick::run_tick::run_tick;

#[derive(Debug, PartialEq)]
enum Belligerent {
    WesternArmy,
    EasternArmy,
}

#[derive(Debug)]
enum WinType {
    ArmyConquered,
    KingCaptured,
}

#[derive(Debug)]
pub struct BattleResult {
    id: i32,
    winner: Option<Belligerent>,
    loser: Option<Belligerent>,
    tick_count: u16,
    win_type: Option<WinType>,
}

pub fn run_battle(battle_state: &mut BattleState) -> BattleResult {
    let mut a1_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut a2_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut total_army_count = a1_count + a2_count;

    let (mut winner, mut loser, mut condition): (
        Option<Belligerent>,
        Option<Belligerent>,
        Option<WinType>,
    ) = (None, None, None);

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
                    Some(Belligerent::WesternArmy)
                } else {
                    Some(Belligerent::EasternArmy)
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
        println!("{a1_count} {a2_count}");

        battle_result.tick_count += 1;
        if battle_result.tick_count > 1000 {
            panic!("Infinite loop detected!");
        }
        total_army_count = run_tick(battle_state, total_army_count);
    }

    determine_army_conquered_condition(battle_result, a1_count, a2_count)
}

fn check_for_king_captured_condition(battle_state: &BattleState) -> Option<Belligerent> {
    let a1_battalion_passed_all_opponents = battle_state
        .army_1_state
        .iter()
        .find(|b| (b.position > 150) && b.flying == false);

    let a2_battalion_passed_all_opponents = battle_state
        .army_2_state
        .iter()
        .find(|b| (b.position < -150) && b.flying == false);

    if a1_battalion_passed_all_opponents.is_some() {
        Some(Belligerent::WesternArmy)
    } else if a2_battalion_passed_all_opponents.is_some() {
        Some(Belligerent::EasternArmy)
    } else {
        None
    }
}

fn determine_army_conquered_condition(
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
