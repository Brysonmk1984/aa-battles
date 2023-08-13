use std::collections::HashMap;

use crate::service::query::Army;

pub fn create_hash_of_defaults(army_defaults: Vec<Army>) -> HashMap<&'static str, Army> {
    let mut army_defaults_hash: HashMap<&str, Army> = HashMap::new();
    army_defaults_hash.insert("amazonian_huntresses", army_defaults[0].to_owned());
    army_defaults_hash.insert("avian_cliff_dwellers", army_defaults[1].to_owned());
    army_defaults_hash.insert("highborn_cavalry", army_defaults[2].to_owned());
    army_defaults_hash.insert("imperial_legionnaires", army_defaults[3].to_owned());
    army_defaults_hash.insert("magi_enforcers", army_defaults[4].to_owned());
    army_defaults_hash.insert("north_watch_longbowmen", army_defaults[5].to_owned());
    army_defaults_hash.insert("peacekeeper_monks", army_defaults[6].to_owned());
    army_defaults_hash.insert("ronin_immortals", army_defaults[7].to_owned());
    army_defaults_hash.insert("shinobi_assassins", army_defaults[8].to_owned());
    army_defaults_hash.insert("skull_clan_death_cultists", army_defaults[9].to_owned());

    army_defaults_hash
}

use std::sync::OnceLock;

// static AOE_MAP_1M_SPREAD_CELL: OnceLock<HashMap<f64, i8>> = OnceLock::new();

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
