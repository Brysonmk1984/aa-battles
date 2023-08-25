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
    army_defaults_hash.insert(ArmyName::Militia, army_defaults[12].to_owned());
    army_defaults_hash.insert(ArmyName::HoodedAssassins, army_defaults[13].to_owned());
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

use std::sync::OnceLock;

use crate::types::{Army, ArmyName};

/**
 * WEAPON_ARMOR_CELL
 * stores a hash map of f64s for weapon type against armor type
 */
pub static WEAPON_ARMOR_CELL: OnceLock<HashMap<&str, f64>> = OnceLock::new();

/**
 * fn set_weapon_armor_map
 * used for initializing the chance to hit given weapon type against armor type
 */
pub fn set_weapon_armor_hash() {
    let map = HashMap::from([
        ("Piercing-Unarmored", 1.0),
        ("Piercing-Leather", 0.75),
        ("Piercing-Chain", 0.6),
        ("Piercing-Plate", 0.1),
        ("Crushing-Unarmored", 0.25),
        ("Crushing-Leather", 0.50),
        ("Crushing-Chain", 0.75),
        ("Crushing-Plate", 1.0),
        ("Blunt-Unarmored", 0.75),
        ("Blunt-Leather", 0.75),
        ("Blunt-Chain", 0.5),
        ("Blunt-Plate", 0.25),
        ("Edged-Unarmored", 1.0),
        ("Edged-Leather", 0.75),
        ("Edged-Chain", 0.5),
        ("Edged-Plate", 0.25),
        ("Magic-Unarmored", 0.25),
        ("Magic-Leather", 0.50),
        ("Magic-Chain", 1.0),
        ("Magic-Plate", 0.75),
    ]);
    WEAPON_ARMOR_CELL.set(map);
}

/**
 * LOG_MUTEX
 * Stores a vec of Strings that is added to throughout the battle with information to report
 * To the end user and developer
 */
pub static LOG_MUTEX: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn push_logs(mut new_messages: Vec<String>) {
    let mut val = LOG_MUTEX.lock().unwrap();
    val.append(&mut new_messages);
}

pub fn push_log(message: String) {
    let mut val = LOG_MUTEX.lock().unwrap();
    val.push(message);
}

pub fn get_logs() -> Vec<String> {
    LOG_MUTEX.lock().unwrap().to_vec()
}

pub fn get_logs_as_string() -> String {
    LOG_MUTEX.lock().unwrap().to_vec().join(", ")
}

#[derive(Debug)]
pub struct BattleLog {
    pub headline: Option<String>,
    pub events: Option<Vec<String>>,
    pub end_state: Option<String>,
    pub outcome: Option<String>,
}

impl BattleLog {
    pub fn new() -> BattleLog {
        BattleLog {
            headline: None,
            events: None,
            end_state: None,
            outcome: None,
        }
    }
}
