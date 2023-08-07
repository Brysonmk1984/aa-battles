use std::collections::HashMap;

use serde::de::Error;
use thiserror::Error;

use crate::{match_up::match_up::StartingDirection, service::query::Army};

use super::{match_up::Battalion, mock_default_army_vec::get_mock_defaults};

#[derive(Error, Debug)]
pub enum MockError {
    #[error("Invalid army name, can't create mock!")]
    InvalidArmyName,
}

pub fn create_mock_army_defaults(
    defaults_option: Option<HashMap<&str, Army>>,
) -> HashMap<&str, Army> {
    let mut defaults: HashMap<&str, Army>;

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
    army_defaults: &HashMap<&str, Army>,
    army_selection: Vec<&str>,
) -> Result<Vec<Battalion>, MockError> {
    let vec_to_return = army_selection
        .iter()
        .enumerate()
        .map(|(index, b_name)| -> Result<Battalion, MockError> {
            let army = army_defaults.get(army_selection[index]);

            if army.is_none() {
                return Err(MockError::InvalidArmyName);
            }

            Ok(Battalion {
                position: if army_direction == StartingDirection::WEST {
                    -150
                } else {
                    150
                },
                ..Battalion::from(army.unwrap())
            })
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
}

pub fn create_mock_generic_battalion(partial_battalion: PartialBattalionForTests) -> Battalion {
    let mock_battalion = Battalion {
        range: partial_battalion.range.or(Some(5)).unwrap(),
        speed: partial_battalion.speed.or(Some(5)).unwrap(),
        count: partial_battalion.count.or(Some(1000)).unwrap(),
        position: partial_battalion.position.or(Some(150)).unwrap(),
        is_marching: true,
        flying: partial_battalion.flying.or(Some(false)).unwrap(),
        ..Default::default()
    };

    mock_battalion
}
