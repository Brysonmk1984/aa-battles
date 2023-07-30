use crate::match_up::match_up::{Battalion, StartingDirection};

pub fn march(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        println!("{} {} ", a.position, a.is_marching);

        if a.is_marching {
            a.march(*starting_direction)
        }
    })
}
