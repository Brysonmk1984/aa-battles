#![allow(warnings)]
use color_eyre::eyre::Result;
use service::query;
use std::{collections::HashMap, error::Error, fs::File, io::Write};
use types::Battalion;

use crate::{
    battle::do_battle::run_battle,
    format_results::format_battle_state,
    match_up::{create_mocks::create_mock_army_defaults, match_up::get_battle_tuple},
    types::{Army, ArmyName, BattleState},
    util::{create_hash_of_defaults, set_weapon_armor_hash, WEAPON_ARMOR_CELL},
};
mod battle;
mod format_results;
mod match_up;
mod service;
mod types;
mod util;

pub const MIN_RANGE_ATTACK_AIR: i32 = 20;
pub const IS_MARCHING_AGILITY_MOD: f64 = 0.15;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let weapon_armor_defaults = set_weapon_armor_hash();

    let mut army_defaults = query::get_all_armies().await.unwrap();
    army_defaults.sort_by(|a, b| a.id.cmp(&b.id));

    let mut army_defaults_hash: HashMap<ArmyName, Army> = create_hash_of_defaults(army_defaults);

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
