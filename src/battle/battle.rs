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
}

pub fn run_battle(battle_state: &mut BattleState) -> BattleResult {
    let a1_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let a2_count = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut total_army_count = a1_count + a2_count;

    let mut tick_count = 0;

    while total_army_count > 0 {
        //println!("{total_army_count}");
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

    // return results
    BattleResult {
        id: 1,
        winner: winner.to_string(),
        loser: loser.to_string(),
        tick_count,
    }
}
