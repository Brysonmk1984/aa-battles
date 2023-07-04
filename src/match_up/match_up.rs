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
#[derive(Debug)]
pub struct Battalion {
    pub name: String,
    pub count: i32,
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

pub fn get_battle_tuple(id_1: i32, id_2: i32) -> (BattleArmy, BattleArmy) {
    (get_full_army(id_1), get_full_army(id_2))
}

// Get all battalions belonging to a particular nation & return as a full army (BattleArmy)
// Takes in nation_id
pub fn get_full_army(id: i32) -> BattleArmy {
    let whole_army = BattleArmy {
        nation_id: id,
        full_army: get_mock(id),
    };

    whole_army
}

fn get_mock(id: i32) -> Vec<Battalion> {
    if id == 1 {
        vec![
            Battalion {
                name: "Imperial Legionnaires".to_string(),
                count: 1000,
            },
            Battalion {
                name: "Peacekeeper Monks".to_string(),
                count: 2000,
            },
            Battalion {
                name: "Highborn Cavalry".to_string(),
                count: 250,
            },
        ]
    } else {
        vec![
            Battalion {
                name: "Amazonian Huntresses".to_string(),
                count: 500,
            },
            Battalion {
                name: "Rōnin Immortals".to_string(),
                count: 2000,
            },
            Battalion {
                name: "North Glade Longbowmen".to_string(),
                count: 1000,
            },
        ]
    }
}
