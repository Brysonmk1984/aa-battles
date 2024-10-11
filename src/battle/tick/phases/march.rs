use crate::{entities::battalion::battalion::Battalion, enums::StartingDirection};

pub fn march_phase(army: &mut Vec<Battalion>, starting_direction: &StartingDirection) {
    army.iter_mut().for_each(|a| {
        println!("{} is marching {}", a.name, a.is_marching.get());
        if a.is_marching.get() && a.count.get() > 0 {
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
