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
    run_tick(battle_state);

    // return results
    BattleResult {
        id: 1,
        winner: 1,
        loser: 2,
    }
}
