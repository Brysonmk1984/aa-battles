use std::sync::atomic::Ordering;

use crate::{entities::battalion::battalion::Battalion, enums::StartingDirection};

pub fn march_phase(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        if a.is_marching.load(Ordering::SeqCst) && a.count.load(Ordering::SeqCst) > 0 {
            let opposite_direction = if *starting_direction == StartingDirection::EAST {
                StartingDirection::WEST
            } else {
                StartingDirection::EAST
            };

            let marching_direction = if a.is_reverse_direction {
                opposite_direction
            } else {
                *starting_direction
            };
            a.march(marching_direction)
        }
    })
}
