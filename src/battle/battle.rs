use crate::{
    match_up::match_up::{Battalion, BattleArmy},
    service::query::Army,
    BattleState,
};

use super::tick::run_tick::run_tick;

#[derive(Debug)]
pub struct BattleResult {
    id: i32,
    winner: String,
    loser: String,
    tick_count: u16,
    //win_type: String,
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

    let mut tick_count = 0;

    let mut winner_by_position: Option<&str> = None;

    while a1_count > 0 && a2_count > 0 {
        //winner_by_position = check_for_positional_win(&battle_state);

        a1_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
            sum += b.count;
            sum
        });
        a2_count = battle_state.army_2_state.iter().fold(0, |mut sum, b| {
            sum += b.count;
            sum
        });
        println!("{a1_count} {a2_count}");

        break;

        tick_count += 1;
        if tick_count > 1000 {
            panic!("Infinite loop detected!");
        }
        total_army_count = run_tick(battle_state, total_army_count);
    }

    let winner = if a1_count > a2_count {
        "Army 1"
    } else {
        "Army 2"
    };
    let loser = if a1_count > a2_count {
        "Army 2"
    } else {
        "Army 1"
    };

    // let win_type = match winner_by_position {
    //     Some(_) => "positional".to_string(),
    //     None => "army defeated".to_string(),
    // };

    // return results
    BattleResult {
        id: 1,
        winner: winner.to_string(),
        loser: loser.to_string(),
        tick_count,
        //win_type,
    }
}

fn check_for_positional_win(battle_state: &BattleState) -> Option<&str> {
    let a1_battalion_passed_all_opponents = battle_state
        .army_1_state
        .iter()
        .find(|b| (b.position > 150 || b.position < 150) && b.flying == false);

    let a2_battalion_passed_all_opponents = battle_state
        .army_2_state
        .iter()
        .find(|b| (b.position > 150 || b.position < 150) && b.flying == false);

    if a1_battalion_passed_all_opponents.is_some() {
        Some("Army 1")
    } else if a2_battalion_passed_all_opponents.is_some() {
        Some("Army 2")
    } else {
        None
    }
}
