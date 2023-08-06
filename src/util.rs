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
