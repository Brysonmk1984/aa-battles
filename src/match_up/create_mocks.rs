use std::collections::HashMap;

use serde::de::Error;
use thiserror::Error;

use crate::types::{
    Army, ArmyName, Battalion, BattleArmy, Belligerent, Nation, NationArmy,
    PartialBattalionForTests, StartingDirection,
};

use super::mock_default_army_vec::get_mock_defaults;

#[derive(Error, Debug)]
pub enum MockError {
    #[error("Invalid army name, can't create mock!")]
    InvalidArmyName,
}

/**
 * Depending on if army defaults from the db were passed in (as a hashmap), either use those as the defaults
 * Or use the mock_defaults for testing purposes
 * TODO - In the future, use the actual DB values for tests
 */
pub fn create_mock_army_defaults(
    defaults_option: Option<HashMap<ArmyName, Army>>,
) -> HashMap<ArmyName, Army> {
    match defaults_option {
        Some(defaults_from_db) => defaults_from_db,
        None => get_mock_defaults(),
    }
}

/**
 * Temporary func for generating armies
 */
pub fn create_mock_army(
    army_direction: StartingDirection,
    army_defaults: &HashMap<ArmyName, Army>,
    army_selection: Vec<ArmyName>,
) -> Result<Vec<Battalion>, MockError> {
    let vec_to_return = army_selection
        .iter()
        .enumerate()
        .map(|(index, b_name)| -> Result<Battalion, MockError> {
            let army = army_defaults.get(&army_selection[index]);

            if army.is_none() {
                return Err(MockError::InvalidArmyName);
            }

            if army_direction == StartingDirection::WEST {
                Ok(Battalion {
                    position: -150,
                    starting_direction: StartingDirection::WEST,
                    ..Battalion::from(army.unwrap())
                })
            } else {
                Ok(Battalion {
                    position: 150,
                    starting_direction: StartingDirection::EAST,
                    ..Battalion::from(army.unwrap())
                })
            }
        })
        .collect();

    vec_to_return
}

pub fn create_mock_generic_battalion(partial_battalion: PartialBattalionForTests) -> Battalion {
    let mock_battalion = Battalion {
        range: partial_battalion.range.or(Some(5)).unwrap(),
        speed: partial_battalion.speed.or(Some(5)).unwrap(),
        count: partial_battalion.count.or(Some(1000)).unwrap(),
        position: partial_battalion.position.or(Some(150)).unwrap(),
        aoe: partial_battalion.aoe.or(Some(0.0)).unwrap(),
        is_marching: true,
        spread: partial_battalion.spread.or(Some(1.0)).unwrap(),
        flying: partial_battalion.flying.or(Some(false)).unwrap(),
        ..Default::default()
    };

    mock_battalion
}

pub fn create_default_competitor() -> (Nation, Vec<NationArmy>) {
    (
        Nation {
            ..Default::default()
        },
        vec![] as Vec<NationArmy>,
    )
}
