use crate::match_up::match_up::{Battalion, StartingDirection};

pub fn march_phase(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        if a.is_marching && a.count > 0 {
            a.march(*starting_direction)
        }
    })
}
