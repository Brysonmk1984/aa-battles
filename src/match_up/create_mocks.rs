use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use serde::de::Error;
use thiserror::Error;

use crate::{
    entities::{
        army::Army, battalion::battalion::Battalion, nation::Nation,
        nation_army::nation_army::NationArmy,
        testing_entities::partial_battalion_for_testing::PartialBattalionForTests,
    },
    enums::{ArmyName, Belligerent, StartingDirection},
};

#[derive(Error, Debug)]
pub enum MockError {
    #[error("Invalid army name, can't create mock!")]
    InvalidArmyName,
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
        count: partial_battalion
            .count
            .or(Some(AtomicU32::new(1000)))
            .unwrap(),
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
