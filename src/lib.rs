#![allow(warnings)]
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, error::Error, fs::File, io::Write};
use types::{Battalion, BattleArmy, BattleResult, NationArmy};
use util::Stats;

use crate::{
    match_up::{
        create_mocks::{create_default_competitor, create_mock_army_defaults},
        match_up::{create_battle_army, create_mock_battle_army, get_battle_tuple},
    },
    types::{Army, ArmyName, Battle, Nation},
    util::{
        create_hash_of_defaults, get_logs, get_stats, push_log, reset_stats, set_weapon_armor_hash,
        BattleLog, LOG_MUTEX, WEAPON_ARMOR_CELL,
    },
};

mod battle;
mod match_up;
pub mod types;
mod util;

pub const MIN_RANGE_ATTACK_AIR: i32 = 20;
pub const IS_MARCHING_AGILITY_MOD: f64 = 0.15;

type NationWithNationArmies = (Nation, Vec<NationArmy>);

pub fn do_battle(
    army_defaults: Vec<Army>,
    competitors: (NationWithNationArmies, NationWithNationArmies),
) -> Result<EndBattlePayload> {
    dotenvy::dotenv().ok();
    reset_stats();
    let weapon_armor_defaults = set_weapon_armor_hash();
    let mut battle_log = BattleLog::new();

    let mut army_defaults_hash: HashMap<ArmyName, Army> = create_hash_of_defaults(army_defaults);

    let mut battle_tuple;

    println!("** USING COMPETITORS FROM DB **\n\n");

    battle_tuple = get_battle_tuple(
        (competitors.0, competitors.1),
        create_mock_army_defaults(Some(army_defaults_hash)),
        create_battle_army,
    )
    .context("Couldn't create battle tuple")?;

    let battle_headline = format!(
        "{} \nVS\n{}",
        battle_tuple.0.log_prebattle_count(),
        battle_tuple.1.log_prebattle_count()
    );

    battle_log.headline = Some(battle_headline);

    let mut battle = Battle {
        army_1_state: battle_tuple.0.full_army.clone(),
        army_2_state: battle_tuple.1.full_army.clone(),
    };

    let battle_result = battle.run_battle();
    println!("'THRESULTS: {battle_result:?}");

    //let battle_stats = get_stats();

    //let eastern_stats_formatted = battle_stats.0.format_battle_stats();
    //let western_stats_formatted = battle_stats.1.format_battle_stats();

    // let final_battle_state_formatted = battle.format_battle_state(
    //     &battle_result,
    //     &eastern_stats_formatted,
    //     &western_stats_formatted,
    // );

    //battle_log.end_state = Some(final_battle_state_formatted);
    //battle_log.events = Some(get_logs());

    let end_battle_payload = EndBattlePayload {
        battle_result,
        army_compositions: battle_tuple,
        events: get_logs(),
        stats: get_stats(),
    };

    Ok(end_battle_payload)
}

#[derive(Serialize, Debug)]
pub struct EndBattlePayload {
    pub battle_result: BattleResult,
    pub army_compositions: (BattleArmy, BattleArmy),
    pub events: Vec<String>,
    pub stats: (Stats, Stats),
}
