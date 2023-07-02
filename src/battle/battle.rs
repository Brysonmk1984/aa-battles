use crate::match_up::match_up::BattleArmy;

#[derive(Debug)]
pub struct BattleResult {
    id: i32,
    winner: i32,
    loser: i32,
}

// Here we need to track EACH battalions'
// 1. count
// 2. position
struct Army_1<'a> {
    count: &'a mut u32,
}

pub fn run_battle(battle_tuple: (BattleArmy, BattleArmy)) -> BattleResult {
    // return results
    BattleResult {
        id: 1,
        winner: 1,
        loser: 2,
    }
}
