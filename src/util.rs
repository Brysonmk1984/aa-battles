use std::{collections::HashMap, sync::Mutex};

pub fn create_hash_of_defaults(army_defaults: Vec<Army>) -> HashMap<ArmyName, Army> {
    let mut army_defaults_hash: HashMap<ArmyName, Army> = HashMap::new();
    army_defaults_hash.insert(ArmyName::PeacekeeperMonks, army_defaults[0].to_owned());
    army_defaults_hash.insert(ArmyName::ImperialLegionnaires, army_defaults[1].to_owned());
    army_defaults_hash.insert(ArmyName::NorthWatchLongbowmen, army_defaults[2].to_owned());
    army_defaults_hash.insert(ArmyName::HighbornCavalry, army_defaults[3].to_owned());
    army_defaults_hash.insert(ArmyName::RoninImmortals, army_defaults[4].to_owned());
    army_defaults_hash.insert(ArmyName::ShinobiMartialArtists, army_defaults[5].to_owned());

    army_defaults_hash.insert(ArmyName::AmazonianHuntresses, army_defaults[6].to_owned());
    army_defaults_hash.insert(ArmyName::AvianCliffDwellers, army_defaults[7].to_owned());

    army_defaults_hash.insert(ArmyName::MagiEnforcers, army_defaults[8].to_owned());
    army_defaults_hash.insert(
        ArmyName::SkullClanDeathCultists,
        army_defaults[9].to_owned(),
    );
    army_defaults_hash.insert(
        ArmyName::BarbariansOfTheOuterSteppe,
        army_defaults[10].to_owned(),
    );
    army_defaults_hash.insert(ArmyName::OathSwornKnights, army_defaults[11].to_owned());
    army_defaults_hash.insert(ArmyName::MinuteMenMilitia, army_defaults[12].to_owned());
    army_defaults_hash.insert(ArmyName::DeathDealerAssassins, army_defaults[13].to_owned());
    army_defaults_hash.insert(ArmyName::ElvenArchers, army_defaults[14].to_owned());
    army_defaults_hash.insert(
        ArmyName::CastlegateCrossbowmen,
        army_defaults[15].to_owned(),
    );

    army_defaults_hash
}

pub fn determine_aoe_effect(aoe: f64, spread: f64) -> i8 {
    if spread == 1.0 {
        if aoe == 0.0 {
            1
        } else if aoe == 0.5 {
            2
        } else if aoe == 1.0 {
            5
        } else if aoe == 1.5 {
            9
        } else if aoe == 2.0 {
            13
        } else if aoe == 2.5 {
            20
        } else if aoe == 3.0 {
            33
        } else {
            panic!("Unsupported AOE value! {} for spread {}", aoe, spread);
        }
    } else if spread == 2.0 {
        if aoe == 0.0 {
            1
        } else if aoe == 0.5 {
            1
        } else if aoe == 1.0 {
            2
        } else if aoe == 1.5 {
            3
        } else if aoe == 2.0 {
            5
        } else if aoe == 2.5 {
            7
        } else if aoe == 3.0 {
            9
        } else {
            panic!("Unsupported AOE value! {} for spread {}", aoe, spread);
        }
    } else if spread == 3.0 {
        if aoe == 0.0 {
            1
        } else if aoe == 0.5 {
            1
        } else if aoe == 1.0 {
            1
        } else if aoe == 1.5 {
            2
        } else if aoe == 2.0 {
            2
        } else if aoe == 2.5 {
            3
        } else if aoe == 3.0 {
            5
        } else {
            panic!("Unsupported AOE value! {} for spread {}", aoe, spread);
        }
    } else {
        panic!("Unsupported Spread value! - {spread}");
    }
}

use std::sync::{OnceLock, RwLock};

use num_format::{Locale, ToFormattedString};
use serde::Serialize;

use crate::types::{Army, ArmyName, Belligerent, StartingDirection};
/**
 * WEAPON_ARMOR_CELL
 * stores a hash map of f64s for weapon type against armor type
 */
pub static WEAPON_ARMOR_CELL: OnceLock<HashMap<&str, f64>> = OnceLock::new();
/**
 * LOG_MUTEX
 * Stores a vec of Strings that is added to throughout the battle with information to report
 * To the end user and developer
 */
pub static LOG_MUTEX: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn push_log(message: String) {
    let mut val = LOG_MUTEX.lock().unwrap();
    val.push(message);
}

pub fn get_logs() -> Vec<String> {
    LOG_MUTEX.lock().unwrap().to_vec()
}

/**
 * STATS_MUTEX
 * Stores a Stats Struct that tracks stats about battalion performance to report
 * To the end user and developer
 */
pub static STATS_RWLOCK: RwLock<(Stats, Stats)> = RwLock::new((
    get_stat_defaults(StartingDirection::EAST),
    get_stat_defaults(StartingDirection::WEST),
));

const fn get_stat_defaults(starting_direction: StartingDirection) -> Stats {
    Stats {
        starting_direction,
        dodge_count: 0,
        block_count: 0,
        armor_defense_count: 0,
        kill: 0,
    }
}

pub fn reset_stats() {
    let mut tuple = STATS_RWLOCK.write().unwrap();
    tuple.0 = get_stat_defaults(StartingDirection::EAST);
    tuple.1 = get_stat_defaults(StartingDirection::WEST);
}

pub fn push_stat_dodge(starting_direction: StartingDirection) {
    let mut tuple = STATS_RWLOCK.write().unwrap();

    if starting_direction == StartingDirection::WEST {
        tuple.0.dodge_count += 1;
    } else {
        tuple.1.dodge_count += 1;
    }
}

pub fn push_stat_block(starting_direction: StartingDirection) {
    let mut tuple = STATS_RWLOCK.write().unwrap();

    if starting_direction == StartingDirection::EAST {
        tuple.0.block_count += 1;
    } else {
        tuple.1.block_count += 1;
    }
}

pub fn push_stat_armor(starting_direction: StartingDirection) {
    let mut tuple = STATS_RWLOCK.write().unwrap();

    if starting_direction == StartingDirection::EAST {
        tuple.0.armor_defense_count += 1;
    } else {
        tuple.1.armor_defense_count += 1;
    }
}

pub fn push_stat_kill(kills: u32, starting_direction: StartingDirection) {
    let mut tuple = STATS_RWLOCK.write().unwrap();

    if starting_direction == StartingDirection::EAST {
        tuple.0.kill += kills;
    } else {
        tuple.1.kill += kills;
    }
}

pub fn get_stats() -> (Stats, Stats) {
    let tuple = STATS_RWLOCK.read().unwrap();
    (
        Stats {
            starting_direction: StartingDirection::EAST,
            dodge_count: tuple.0.dodge_count,
            block_count: tuple.0.block_count,
            armor_defense_count: tuple.0.armor_defense_count,
            kill: tuple.0.kill,
        },
        Stats {
            starting_direction: StartingDirection::WEST,
            dodge_count: tuple.1.dodge_count,
            block_count: tuple.1.block_count,
            armor_defense_count: tuple.1.armor_defense_count,
            kill: tuple.1.kill,
        },
    )
}

#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub starting_direction: StartingDirection,
    pub dodge_count: u32,
    pub block_count: u32,
    pub armor_defense_count: u32,
    pub kill: u32,
}

impl Stats {
    pub fn format_battle_stats(&self) -> String {
        let stats = format!(
            "\n\nKills: {}\nAttacks Dodged: {}\nAttacks Blocked by Shield: {}\nAttacks Blocked by Armor: {}\n",
            self.kill.to_formatted_string(&Locale::en), self.dodge_count.to_formatted_string(&Locale::en), self.block_count.to_formatted_string(&Locale::en), self.armor_defense_count.to_formatted_string(&Locale::en),
        );
        stats
    }
}

#[derive(Debug)]
pub struct BattleLog {
    pub headline: Option<String>,
    pub events: Option<Vec<String>>,
    pub stats: Stats,
    pub end_state: Option<String>,
    pub outcome: Option<String>,
}

impl BattleLog {
    pub fn new() -> BattleLog {
        BattleLog {
            headline: None,
            events: None,
            stats: Stats {
                starting_direction: StartingDirection::EAST,
                dodge_count: 0,
                block_count: 0,
                armor_defense_count: 0,
                kill: 0,
            },
            end_state: None,
            outcome: None,
        }
    }
}
