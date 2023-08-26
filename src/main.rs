#![allow(warnings)]
use color_eyre::eyre::Result;
use service::query;
use std::{collections::HashMap, error::Error, fs::File, io::Write};
use types::Battalion;

use crate::{
    match_up::{create_mocks::create_mock_army_defaults, match_up::get_battle_tuple},
    types::{Army, ArmyName, Battle},
    util::{
        create_hash_of_defaults, get_logs, get_logs_as_string, push_log, push_logs,
        set_weapon_armor_hash, BattleLog, LOG_MUTEX, WEAPON_ARMOR_CELL,
    },
};
mod battle;
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
    let mut battle_log = BattleLog::new();

    let mut army_defaults = query::get_all_armies().await.unwrap();
    army_defaults.sort_by(|a, b| a.id.cmp(&b.id));

    let mut army_defaults_hash: HashMap<ArmyName, Army> = create_hash_of_defaults(army_defaults);

    let mut battle_tuple =
        get_battle_tuple(1, 2, create_mock_army_defaults(Some(army_defaults_hash)))?;

    let battle_headline = format!(
        "{} \nVS\n{}",
        battle_tuple.0.log_prebattle_count(),
        battle_tuple.1.log_prebattle_count()
    );

    battle_log.headline = Some(battle_headline);

    //println!("{:#?}", LOG_MUTEX.lock().unwrap().join(""));

    let mut battle = Battle {
        army_1_state: battle_tuple.0.full_army,
        army_2_state: battle_tuple.1.full_army,
    };

    let battle_result = battle.run_battle();
    let final_battle_state_formatted = battle.format_battle_state(&battle_result);

    battle_log.end_state = Some(final_battle_state_formatted);

    let outcome = battle_result.format_outcome();
    battle_log.outcome = Some(outcome);

    let path = "results.txt";
    let mut output = File::create(path)?;

    println!("{battle_log:#?}");
    battle_log.events = Some(get_logs_as_string());
    write!(output, "{}", get_logs_as_string());
    Ok(())
}
