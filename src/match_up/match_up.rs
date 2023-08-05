use serde::{Deserialize, Serialize};

use crate::service::query::Army;

use super::create_mocks::create_mock_army;

#[derive(Clone, Copy, Debug)]
pub enum StartingDirection {
    EAST,
    WEST,
}

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
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
    pub is_marching: bool,
}

impl Battalion {
    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn set_is_marching(&mut self, value: bool) {
        self.is_marching = value;
    }

    /**
     * If Starting direction is west, army starts at -150 and marches east, west starts at 150 and marches east
     */
    pub fn march(&mut self, starting_direction: StartingDirection) {
        if let StartingDirection::WEST = starting_direction {
            self.position += self.speed;
        } else {
            self.position -= self.speed;
        }
    }
}

// Full Army a user will use to battle
#[derive(Debug, Clone)]
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

/**
*  fn get_full_army -
   Get all battalions belonging to a particular nation () & return as a full army (BattleArmy)
* params - id (nation Id), army_defaults (vector of army types, to be converted to Battalion)
*/
pub fn get_full_army(id: i32, army_defaults: &Vec<Army>) -> BattleArmy {
    let whole_army = BattleArmy {
        nation_id: id,
        full_army: create_mock_army(id, army_defaults),
    };

    whole_army
}

pub fn get_db_battalion_properties(
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
            is_marching: true,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::match_up::create_mocks::{create_mock_generic_battalion, PartialBattalionForTests};

    use super::Battalion;

    #[test]
    fn should_march_west_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = 150;
        assert_eq!(test_battalion_ref.position, 150);
        test_battalion_ref.march(super::StartingDirection::EAST);
        assert_eq!(test_battalion_ref.position, 100);
    }

    #[test]
    fn should_march_east_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];
        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = -150;
        assert_eq!(test_battalion_ref.position, -150);
        test_battalion_ref.march(super::StartingDirection::WEST);
        assert_eq!(test_battalion_ref.position, -100);
    }

    #[test]
    fn should_decrease_count_by_one() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();

        assert_eq!(test_battalion_ref.count, 1000);
        test_battalion_ref.decrement();
        assert_eq!(test_battalion_ref.count, 999);
    }

    #[test]
    fn should_set_is_marching() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];

        let test_battalion_ref = test_army.get_mut(0).unwrap();

        assert_eq!(test_battalion_ref.is_marching, true);
        test_battalion_ref.set_is_marching(false);
        assert_eq!(test_battalion_ref.is_marching, false);
        test_battalion_ref.set_is_marching(true);
        assert_eq!(test_battalion_ref.is_marching, true);
    }
}
