use crate::match_up::match_up::BattleArmy;

#[derive(Debug)]
pub struct BattleResult {
    id: i32,
    winner: i32,
    loser: i32,
}

#[derive(Debug)]
// Here we need to track EACH battalions'
struct BattalionState {
    name: String,
    count: i32,
    position: i32,
}

#[derive(Debug)]
struct Battle {
    army_1_state: Vec<BattalionState>,
    army_2_state: Vec<BattalionState>,
}

pub fn run_battle(battle_tuple: (BattleArmy, BattleArmy)) -> BattleResult {
    println!("{battle_tuple:?}");

    let mut battle = Battle {
        army_1_state: battle_tuple
            .0
            .full_army
            .iter()
            .map(|mut b| BattalionState {
                name: b.name.to_owned(),
                count: b.count.clone(),
                position: -150,
            })
            .collect(),
        army_2_state: battle_tuple
            .1
            .full_army
            .iter()
            .map(|b| BattalionState {
                name: b.name.to_owned(),
                count: b.count.to_owned(),
                position: -150,
            })
            .collect(),
    };

    run_battle_update(battle);

    // return results
    BattleResult {
        id: 1,
        winner: 1,
        loser: 2,
    }
}

fn run_battle_update(mut battle: Battle) {
    battle.army_1_state[0].position = 999;

    println!("{battle:?}")
}
