#![allow(warnings)]
use color_eyre::eyre::Result;
use match_up::match_up::Battalion;
use service::query;
use std::{collections::HashMap, error::Error, fs::File, io::Write};

use crate::{
    battle::battle::run_battle,
    format_results::format_battle_state,
    match_up::{create_mocks::create_mock_army_defaults, match_up::get_battle_tuple},
    service::query::Army,
    util::{create_hash_of_defaults, set_weapon_armor_hash, WEAPON_ARMOR_CELL},
};
mod battle;
mod format_results;
mod match_up;
mod service;
mod util;

pub const MIN_RANGE_ATTACK_AIR: i32 = 20;

#[derive(Debug)]
pub struct BattleState {
    pub army_1_state: Vec<Battalion>,
    pub army_2_state: Vec<Battalion>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let weapon_armor_defaults = set_weapon_armor_hash();

    let mut army_defaults = query::get_all_armies().await.unwrap();

    army_defaults.sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
    let mut army_defaults_hash: HashMap<&str, Army> = create_hash_of_defaults(army_defaults);

    let mut battle_tuple =
        get_battle_tuple(1, 2, create_mock_army_defaults(Some(army_defaults_hash)))?;

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
