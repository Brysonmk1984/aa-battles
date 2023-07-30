#![allow(warnings)]
use match_up::match_up::Battalion;
use service::query;
use std::error::Error;

use crate::{battle::battle::run_battle, match_up::match_up::get_battle_tuple};
mod battle;
mod match_up;
mod service;

#[derive(Debug)]
pub struct BattleState {
    pub army_1_state: Vec<Battalion>,
    pub army_2_state: Vec<Battalion>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let army_defaults = query::get_all_armies().await.unwrap();
    let mut battle_tuple = get_battle_tuple(1, 2, army_defaults);

    let mut battle_state = BattleState {
        army_1_state: battle_tuple.0.full_army,
        army_2_state: battle_tuple.1.full_army,
    };
    println!("START : {battle_state:?}");
    let battle_result = run_battle(&mut battle_state);
    println!("FINAL : {battle_state:?}");
    println!("BATTLE RESULTS : {battle_result:?}");
    Ok(())
}
