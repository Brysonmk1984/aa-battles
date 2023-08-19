use std::collections::HashMap;

use serde::de::Error;
use thiserror::Error;

use crate::{
    match_up::match_up::StartingDirection,
    service::query::{Army, ArmyName},
};

use super::{match_up::Battalion, mock_default_army_vec::get_mock_defaults};

#[derive(Error, Debug)]
pub enum MockError {
    #[error("Invalid army name, can't create mock!")]
    InvalidArmyName,
}

pub fn create_mock_army_defaults(
    defaults_option: Option<HashMap<ArmyName, Army>>,
) -> HashMap<ArmyName, Army> {
    let mut defaults: HashMap<ArmyName, Army>;

    match defaults_option {
        Some(defaults_from_db) => defaults = defaults_from_db,
        None => defaults = get_mock_defaults(),
    }

    defaults
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

#[derive(Default)]
pub struct PartialBattalionForTests {
    pub count: Option<i32>,
    pub position: Option<i32>,
    pub speed: Option<i32>,
    pub flying: Option<bool>,
    pub range: Option<i32>,
    pub aoe: Option<f64>,
    pub spread: Option<f64>,
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
