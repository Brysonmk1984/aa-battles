use crate::{
    match_up::match_up::{Battalion, BattleArmy},
    service::query::Army,
    BattleState,
};

use super::tick::run_tick::run_tick;

#[derive(Debug)]
pub struct BattleResult {
    id: i32,
    winner: i32,
    loser: i32,
}

pub fn run_battle(battle_state: &mut BattleState) -> BattleResult {
    let a1 = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let a2 = battle_state.army_1_state.iter().fold(0, |mut sum, b| {
        sum += b.count;
        sum
    });
    let mut total_army_count = a1 + a2;

    let mut tick_count = 0;

    while total_army_count > 0 {
        println!("{total_army_count}");
        tick_count += 1;
        if tick_count > 1000 {
            panic!("Infinite loop detected!");
        }
        total_army_count = run_tick(battle_state, total_army_count);
    }

    // return results
    BattleResult {
        id: 1,
        winner: 1,
        loser: 2,
    }
}
