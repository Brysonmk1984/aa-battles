use crate::types::{Battalion, StartingDirection};

pub fn march_phase(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        if a.is_marching && a.count > 0 {
            let opposite_direction = if *starting_direction == StartingDirection::WEST {
                StartingDirection::EAST
            } else {
                StartingDirection::WEST
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
