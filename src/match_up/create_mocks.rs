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
            //get_db_battalion_properties(&imperial_legionnaires, 1000, -150),
            // get_db_battalion_properties(&avian_cliff_dwellers, 1000, -150),
            get_db_battalion_properties(&highborn_cavalry, 1000, -150),
        ]
    } else {
        // EASTER ARMY
        vec![
            //get_db_battalion_properties(&amazonian_huntresses, 1000, 150),
            //get_db_battalion_properties(&magi_enforcers, 1000, 150),
            get_db_battalion_properties(&north_watch_longbowmen, 1000, 150),
        ]
    }
}

#[derive(Default)]
pub struct PartialBattalionForTests {
    pub count: Option<i32>,
    pub position: Option<i32>,
    pub speed: Option<i32>,
    pub flying: Option<bool>,
    pub range: Option<i32>,
}

pub fn create_mock_generic_battalion(partial_battalion: PartialBattalionForTests) -> Battalion {
    let mock_battalion = Battalion {
        range: partial_battalion.range.or(Some(0)).unwrap(),
        speed: partial_battalion.speed.or(Some(50)).unwrap(),
        count: partial_battalion.count.or(Some(1000)).unwrap(),
        position: partial_battalion.position.or(Some(150)).unwrap(),
        is_marching: true,
        flying: partial_battalion.flying.or(Some(false)).unwrap(),
        ..Default::default()
    };

    mock_battalion
}
