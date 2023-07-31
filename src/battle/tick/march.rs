use crate::match_up::match_up::{Battalion, StartingDirection};

pub fn march(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        let which_army = match starting_direction {
            StartingDirection::WEST => "1",
            StartingDirection::EAST => "2",
        };

        if a.is_marching && a.count > 0 {
            a.march(*starting_direction)
        }
    })
}
