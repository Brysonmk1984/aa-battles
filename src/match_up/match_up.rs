use serde::{Deserialize, Serialize};

use crate::service::query::Army;

// Just like how AvatarItem can have many different types of items,
// ArmyNation can have many different armies.
// They are represented as different rows
struct ArmyNation {
    id: i32,
    army_id: i32,
    nation_id: i32,
    count: u32,
}

// An Army Type with count belonging to a user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Battalion {
    pub name: String,
    pub count: i32,
    pub position: i32,
    pub shield_rating: f64,
    pub flying: bool,
    pub range: i32,
    pub attack_speed: f64,
    pub accuracy: f64,
    pub aoe: bool,
    pub weapon_type: String,
    pub armor_type: String,
    pub agility: f64,
    pub speed: i32,
}

// impl Marching for Battalion {
//     fn march() {}
// }

// Full Army a user will use to battle
#[derive(Debug)]
pub struct BattleArmy {
    nation_id: i32,
    pub full_army: Vec<Army>,
}

pub fn get_battle_tuple(
    id_1: i32,
    id_2: i32,
    army_defaults: Vec<Army>,
) -> (BattleArmy, BattleArmy) {
    (
        get_full_army(id_1, &army_defaults),
        get_full_army(id_2, &army_defaults),
    )
}

// Get all battalions belonging to a particular nation & return as a full army (BattleArmy)
// Takes in nation_id
pub fn get_full_army(id: i32, army_defaults: &Vec<Army>) -> BattleArmy {
    let whole_army = BattleArmy {
        nation_id: id,
        full_army: get_mock(id, army_defaults),
    };

    whole_army
}

fn get_mock(id: i32, army_defaults: &Vec<Army>) -> Vec<Army> {
    let db_battalion_templates = army_defaults;

    let imperial_legionnaires = db_battalion_templates
        .iter()
        .find(|a| a.name == "Imperial Legionnaires")
        .unwrap()
        .to_owned();
    let peacekeeper_monks = db_battalion_templates
        .iter()
        .find(|a| a.name == "Peacekeeper Monks")
        .unwrap()
        .to_owned();

    let highborn_cavalry = db_battalion_templates
        .iter()
        .find(|a| a.name == "Highborn Cavalry")
        .unwrap()
        .to_owned();

    let amazonian_huntress = db_battalion_templates
        .iter()
        .find(|a| a.name == "Amazonian Huntresses")
        .unwrap()
        .to_owned();

    let ronin_immortals = db_battalion_templates
        .iter()
        .find(|a| a.name == "Rōnin Immortals")
        .unwrap()
        .to_owned();

    let north_watch_longbowmen = db_battalion_templates
        .iter()
        .find(|a| a.name == "North Watch Longbowmen")
        .unwrap()
        .to_owned();

    if id == 1 {
        vec![
            get_db_battalion_properties(imperial_legionnaires, 1000, -150),
            get_db_battalion_properties(peacekeeper_monks, 2000, -150),
            get_db_battalion_properties(highborn_cavalry, 250, -150),
        ]
    } else {
        vec![
            get_db_battalion_properties(amazonian_huntress, 500, 150),
            get_db_battalion_properties(ronin_immortals, 2000, 150),
            get_db_battalion_properties(north_watch_longbowmen, 1000, 150),
        ]
    }
}

fn get_db_battalion_properties(db_battalion_template: Army, count: i32, position: i32) -> Army {
    Army {
        count: count,
        position: Some(position),
        ..db_battalion_template
    }
}

// When we try to call from or into, this doesn't work if even one property isn't the same! (position doesn't exist in DB)

// impl From<Army> for Battalion {
//     fn from(a: Army) -> Self {
//         let serialized = serde_json::to_string(&a).unwrap();
//         serde_json::from_str(&serialized).unwrap()
//     }
// }
