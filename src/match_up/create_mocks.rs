use crate::{match_up::match_up::get_db_battalion_properties, service::query::Army};

use super::match_up::Battalion;

pub fn create_mock_army(id: i32, army_defaults: &Vec<Army>) -> Vec<Battalion> {
    let mut db_battalion_templates = army_defaults.clone();

    db_battalion_templates.sort_by(|a, b| a.name.cmp(&b.name));

    let amazonian_huntresses = db_battalion_templates[0].to_owned();
    let avian_cliff_dwellers = db_battalion_templates[1].to_owned();
    let highborn_cavalry = db_battalion_templates[2].to_owned();
    let imperial_legionnaires = db_battalion_templates[3].to_owned();
    let magi_enforcers = db_battalion_templates[4].to_owned();
    let north_watch_longbowmen = db_battalion_templates[5].to_owned();
    let peacekeeper_monks = db_battalion_templates[6].to_owned();
    let ronin_immortals = db_battalion_templates[7].to_owned();
    let shinobi_assassins = db_battalion_templates[8].to_owned();
    let skull_clan_death_cultists = db_battalion_templates[9].to_owned();

    if id == 1 {
        // WESTERN ARMY
        vec![
            get_db_battalion_properties(&imperial_legionnaires, 1000, 150),
            get_db_battalion_properties(&avian_cliff_dwellers, 1000, -150),
            get_db_battalion_properties(&highborn_cavalry, 1000, -150),
        ]
    } else {
        // EASTER ARMY
        vec![
            get_db_battalion_properties(&amazonian_huntresses, 1000, -150),
            get_db_battalion_properties(&magi_enforcers, 1000, 150),
            get_db_battalion_properties(&north_watch_longbowmen, 1000, 150),
        ]
    }
}

pub fn create_mock_generic_battalion() -> Battalion {
    Battalion {
        name: String::from("Generic Fighters"),
        count: 1000,
        position: 150,
        shield_rating: 0.00,
        flying: false,
        range: 150,
        attack_speed: 0.00,
        accuracy: 0.75,
        aoe: false,
        weapon_type: String::from("piercing"),
        armor_type: String::from("piercing"),
        agility: 0.5,
        speed: 50,
        is_marching: true,
    }
}
