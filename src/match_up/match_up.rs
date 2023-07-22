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

impl Battalion {
    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}

// impl Marching for Battalion {
//     fn march() {}
// }

// Full Army a user will use to battle
#[derive(Debug)]
pub struct BattleArmy {
    nation_id: i32,
    pub full_army: Vec<Battalion>,
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

fn get_mock(id: i32, army_defaults: &Vec<Army>) -> Vec<Battalion> {
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
        vec![
            get_db_battalion_properties(&imperial_legionnaires, 1001, -150),
            get_db_battalion_properties(&avian_cliff_dwellers, 500, -150),
            get_db_battalion_properties(&highborn_cavalry, 250, -150),
        ]
    } else {
        vec![
            get_db_battalion_properties(&amazonian_huntresses, 500, 150),
            get_db_battalion_properties(&ronin_immortals, 2000, 150),
            get_db_battalion_properties(&north_watch_longbowmen, 1000, 150),
        ]
    }
}

fn get_db_battalion_properties(
    db_battalion_template: &Army,
    count: i32,
    position: i32,
) -> Battalion {
    Battalion {
        count: count,
        position,
        ..Battalion::from(db_battalion_template)
    }
}

impl From<&Army> for Battalion {
    fn from(a: &Army) -> Self {
        Self {
            position: 0,
            name: a.name.clone(),
            count: a.count,
            shield_rating: a.shield_rating,
            flying: a.flying,
            range: a.range,
            attack_speed: a.attack_speed,
            accuracy: a.accuracy,
            aoe: a.aoe,
            weapon_type: a.weapon_type.clone(),
            armor_type: a.armor_type.clone(),
            agility: a.agility,
            speed: a.speed,
        }
    }
}
