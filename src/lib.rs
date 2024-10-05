#![allow(warnings)]
use anyhow::{Context, Result};
use entities::{
    battle::battle::Battle, battle_army::battle_army::BattleArmy,
    battle_result::battle_result::BattleResult, game_defaults::GameDefaults, nation::Nation,
    nation_army::nation_army::NationArmy,
};
use enums::StartingDirection;
use mocks::game_defaults::GameDefaultsMocks;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, error::Error, fs::File, io::Write};
use util::{clear_logs, Stats, AOE_SPREAD_CELL};

use crate::{
    enums::ArmyName,
    match_up::{
        create_mocks::create_default_competitor,
        match_up::{create_battle_army, create_mock_battle_army, get_battle_tuple},
    },
    util::{
        create_hash_of_defaults, get_logs, get_stats, map_army_defaults, push_log, reset_stats,
        BattleLog, LOG_MUTEX, WEAPON_ARMOR_CELL,
    },
};

mod battle;
mod entities;
pub mod enums;
mod match_up;
mod mocks;
pub mod util;

pub const MIN_RANGE_ATTACK_AIR: i32 = 20;
pub const IS_MARCHING_AGILITY_MOD: f64 = 0.15;

type NationWithNationArmies = (Nation, Vec<NationArmy>);

pub fn do_battle(
    game_defaults: GameDefaults,
    competitors: (NationWithNationArmies, NationWithNationArmies),
) -> Result<EndBattlePayload> {
    dotenvy::dotenv().ok();

    match env::var("ENVIRONMENT") {
        Ok(_) => (),
        Err(e) => env::set_var("ENVIRONMENT", game_defaults.environment),
    }

    WEAPON_ARMOR_CELL.set(game_defaults.weapons_vs_armor);
    AOE_SPREAD_CELL.set(game_defaults.aoe_vs_spread);

    let mut battle_log = BattleLog::new();

    let army_defaults_hash = game_defaults.army_defaults;
    let army_defaults = map_army_defaults(Some(army_defaults_hash));

    let mut battle_tuple;

    let army_composition_for_logs = get_battle_tuple(
        (competitors.0.clone(), competitors.1.clone()),
        army_defaults.clone(),
        create_battle_army,
    )
    .unwrap();

    battle_tuple = get_battle_tuple(
        (competitors.0, competitors.1),
        army_defaults,
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
        army_1_state: battle_tuple.0.full_army,
        army_2_state: battle_tuple.1.full_army,
    };

    let battle_result = battle.run_battle();
    println!("'THRESULTS: {battle_result:?}");

    let battle_stats = get_stats();

    let eastern_stats_formatted = battle_stats.0.format_battle_stats();
    let western_stats_formatted = battle_stats.1.format_battle_stats();

    let final_battle_state_formatted = battle.format_battle_state(
        &battle_result,
        &eastern_stats_formatted,
        &western_stats_formatted,
    );

    println!("FINALFORMATTED {final_battle_state_formatted}");

    // battle_log.end_state = Some(final_battle_state_formatted);
    // battle_log.events = Some(get_logs());

    let end_battle_payload = EndBattlePayload {
        battle_result,
        army_compositions: army_composition_for_logs,
        events: get_logs(),
        stats: get_stats(),
    };

    reset_stats();
    clear_logs();

    Ok(end_battle_payload)
}

#[derive(Serialize, Debug)]
pub struct EndBattlePayload {
    pub battle_result: BattleResult,
    pub army_compositions: (BattleArmy, BattleArmy),
    pub events: Vec<String>,
    pub stats: (Stats, Stats),
}

mod tests {
    use std::collections::HashMap;

    use crate::do_battle;
    use crate::match_up::create_mocks::create_mock_army;

    use crate::mocks::game_defaults::{get_competitors, get_game_defaults, GameDefaultsMocks};
    use crate::util::{create_hash_of_defaults, map_army_defaults, WEAPON_ARMOR_CELL};

    /**
     * do_battle
     * Should run the battle as expected
     */
    #[test]
    fn test_do_battle() {
        let end_battle_payload =
            do_battle(get_game_defaults(), get_competitors(700, 1000)).unwrap();
        println!("{end_battle_payload:?}");
        assert_eq!(
            end_battle_payload.battle_result.winner,
            Some(crate::enums::Belligerent::EasternArmy)
        );
    }
}
