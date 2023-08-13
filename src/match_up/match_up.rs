use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::create_mocks::{create_mock_army, MockError};
use crate::{service::query::Army, util::determine_aoe_effect};
use strum_macros::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum StartingDirection {
    #[default]
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
    pub aoe: f64,
    pub spread: f64,
    pub weapon_type: String,
    pub armor_type: String,
    pub agility: f64,
    pub speed: i32,
    pub is_marching: bool,
    pub starting_direction: StartingDirection,
    pub is_reverse_direction: bool,
}

impl Battalion {
    pub fn decrement(&mut self, attacker_aoe: f64) {
        // if attacker_aoe > 0.0 {
        //     println!(
        //         "HAS AOE - {attacker_aoe}, vs spread {} = hits {}",
        //         self.spread,
        //         determine_aoe_effect(attacker_aoe, self.spread)
        //     );
        // }
        let hits = determine_aoe_effect(attacker_aoe, self.spread);
        self.count -= hits as i32;
    }

    pub fn set_is_marching(&mut self, value: bool) {
        self.is_marching = value;
    }

    pub fn set_is_reverse_direction(&mut self, value: bool) {
        //println!("{value} HAS REVERSED ");
        self.is_reverse_direction = value;
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

        if self.position < -150 || self.position > 150 {
            panic!(
                "{} are out of battlefield bounds - {}!",
                self.name, self.position
            );
        }
    }
}

#[derive(Display)]
pub enum ArmyNames {
    #[strum(serialize = "amazonian_huntresses")]
    AmazonianHuntresses,
    #[strum(serialize = "avian_cliff_dwellers")]
    AvianCliffDwellers,
    #[strum(serialize = "amazonian_huntresses")]
    HighbornCavalry,
    #[strum(serialize = "imperial_legionnaires")]
    ImperialLegionnaires,
    #[strum(serialize = "magi_enforcers")]
    MagiEnforcers,
    #[strum(serialize = "north_watch_longbowmen")]
    NorthWatchLongbowmen,
    #[strum(serialize = "peacekeeper_monks")]
    PeacekeeperMonks,
    #[strum(serialize = "ronin_immortals")]
    RoninImmortals,
    #[strum(serialize = "shinobi_assassins")]
    ShinobiAssassins,
    #[strum(serialize = "skull_clan_death_cultists")]
    SkullClanDeathCultists,
}

// Full Army a user will use to battle
#[derive(Debug, Clone)]
pub struct BattleArmy {
    nation_id: i32,
    pub full_army: Vec<Battalion>,
}

/**
*  fn get_battle_tuple -
   Get all battalions belonging to both nations & return as  full armies (BattleArmy)
* params - id_1 (nation Id), id_2, army_defaults (hashmap of army types, to be converted to Battalion)
*/
pub fn get_battle_tuple(
    id_1: i32,
    id_2: i32,
    army_defaults: HashMap<&str, Army>,
) -> Result<(BattleArmy, BattleArmy), MockError> {
    // TODO: In the future, we need to replace this with the user's army saved in a new db table
    let full_army_west = create_mock_army(
        StartingDirection::WEST,
        &army_defaults,
        vec![
            "imperial_legionnaires",
            // "shinobi_assassins",
            // "amazonian_huntresses",
            // "peacekeeper_monks",
            // "ronin_immortals",
        ],
    )?;

    // TODO: In the future, we need to replace this with the user's army saved in a new db table
    let full_army_east = create_mock_army(
        StartingDirection::EAST,
        &army_defaults,
        vec![
            // "avian_cliff_dwellers",
            // "north_watch_longbowmen",
            "skull_clan_death_cultists",
        ],
    )?;

    Ok((
        BattleArmy {
            nation_id: id_1,
            full_army: full_army_west,
        },
        BattleArmy {
            nation_id: id_2,
            full_army: full_army_east,
        },
    ))
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
            spread: a.spread,
            weapon_type: a.weapon_type.clone(),
            armor_type: a.armor_type.clone(),
            agility: a.agility,
            speed: a.speed,
            is_marching: true,
            starting_direction: StartingDirection::EAST,
            is_reverse_direction: false,
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
        let original_position = test_battalion_ref.position;
        assert_eq!(original_position, 150);
        test_battalion_ref.march(super::StartingDirection::EAST);
        assert_eq!(
            test_battalion_ref.position,
            original_position - test_battalion_ref.speed
        );
    }

    #[test]
    fn should_march_east_distance_based_on_speed() {
        let partial_mock_battalion: PartialBattalionForTests = Default::default();
        let mut test_army = vec![create_mock_generic_battalion(partial_mock_battalion)];
        let test_battalion_ref = test_army.get_mut(0).unwrap();
        test_battalion_ref.position = -150;
        let original_position = test_battalion_ref.position;
        assert_eq!(original_position, -150);
        test_battalion_ref.march(super::StartingDirection::WEST);
        assert_eq!(
            test_battalion_ref.position,
            original_position + test_battalion_ref.speed
        );
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
