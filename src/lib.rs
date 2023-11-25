#![allow(warnings)]
use std::{collections::HashMap, env, error::Error, fs::File, io::Write};
use types::{Battalion, NationArmy};

use crate::{
    match_up::{
        create_mocks::{create_default_competitor, create_mock_army_defaults},
        match_up::{create_battle_army, create_mock_battle_army, get_battle_tuple},
    },
    types::{Army, ArmyName, Battle, Nation},
    util::{
        create_hash_of_defaults, get_logs, get_stats, push_log, set_weapon_armor_hash, BattleLog,
        LOG_MUTEX, WEAPON_ARMOR_CELL,
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
) -> String {
    dotenvy::dotenv().ok();

    let weapon_armor_defaults = set_weapon_armor_hash();
    let mut battle_log = BattleLog::new();

    let mut army_defaults_hash: HashMap<ArmyName, Army> = create_hash_of_defaults(army_defaults);

    // let args: Vec<_> = env::args().collect();

    let mut battle_tuple;

    // if args.len() > 1 && args[1] == "test" {
    //     println!("** USING COMPETITORS FROM MATCHUP FILE ** \n\n");
    //     // Generate BattleArmy without nation merging (for manual tests)
    //     battle_tuple = get_battle_tuple(
    //         (create_default_competitor(), create_default_competitor()),
    //         create_mock_army_defaults(Some(army_defaults_hash)),
    //         create_mock_battle_army,
    //     )
    //     .unwrap();
    // } else {
    println!("** USING COMPETITORS FROM DB **\n\n");
    // Generate BattleArmy for both competitors
    battle_tuple = get_battle_tuple(
        competitors,
        create_mock_army_defaults(Some(army_defaults_hash)),
        create_battle_army,
    )
    .unwrap();
    // }

    let battle_headline = format!(
        "{} \nVS\n{}",
        battle_tuple.0.log_prebattle_count(),
        battle_tuple.1.log_prebattle_count()
    );

    battle_log.headline = Some(battle_headline);

    let mut battle = Battle {
        army_1_state: battle_tuple.0.full_army,
        army_2_state: battle_tuple.1.full_army,
    };

    let battle_result = battle.run_battle();

    let battle_stats = get_stats();
    let western_stats_formatted = battle_stats.0.format_battle_stats();
    let eastern_stats_formatted = battle_stats.1.format_battle_stats();

    let western_stats_formatted = battle_stats.0.format_battle_stats();
    let final_battle_state_formatted = battle.format_battle_state(
        &battle_result,
        &western_stats_formatted,
        &eastern_stats_formatted,
    );

    battle_log.end_state = Some(final_battle_state_formatted);

    let outcome = battle_result.format_outcome();
    battle_log.outcome = Some(outcome);

    //let path = "results.txt";
    //let mut output = File::create(path)?;

    // println!("{}", battle_log.headline.as_ref().unwrap());
    // println!("{}", &battle_log.end_state.as_ref().unwrap());
    // println!("{}", &battle_log.outcome.as_ref().unwrap());

    battle_log.events = Some(get_logs());

    let result = format!(
        // "{output:?}",
        "{} \n\n{} \n\n{} \n\n{}",
        battle_log.headline.unwrap(),
        get_logs(),
        battle_log.end_state.unwrap(),
        battle_log.outcome.unwrap()
    );

    result
}
