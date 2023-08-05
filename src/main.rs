#![allow(warnings)]
use color_eyre::eyre::Result;
use match_up::match_up::Battalion;
use service::query;
use std::{error::Error, fs::File, io::Write};

use crate::{
    battle::battle::run_battle, format_results::format_battle_state,
    match_up::match_up::get_battle_tuple,
};
mod battle;
mod format_results;
mod match_up;
mod service;

#[derive(Debug)]
pub struct BattleState {
    pub army_1_state: Vec<Battalion>,
    pub army_2_state: Vec<Battalion>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    let army_defaults = query::get_all_armies().await.unwrap();
    let mut battle_tuple = get_battle_tuple(1, 2, army_defaults);

    let mut battle_state = BattleState {
        army_1_state: battle_tuple.0.full_army,
        army_2_state: battle_tuple.1.full_army,
    };

    let battle_result = run_battle(&mut battle_state);
    let final_battle_state_formatted = format_battle_state(battle_state, &battle_result);
    println!("{final_battle_state_formatted}");

    let result = format_results::format_outcome(battle_result);
    println!("{result}");

    let path = "results.txt";
    let mut output = File::create(path)?;

    write!(output, "{}\n\n{}", final_battle_state_formatted, result);
    Ok(())
}
