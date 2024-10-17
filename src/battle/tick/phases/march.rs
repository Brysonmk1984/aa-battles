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

/**
 * Determines if fliers are passed all enemies, if so, it reverses the direction
 */
pub fn handle_direction_check(
    army: &mut Vec<Battalion>,
    enemy_army: &mut Vec<Battalion>,
    starting_direction: StartingDirection,
) {
    let mut fliers = army
        .iter_mut()
        .filter(|b| b.flying)
        .collect::<Vec<&mut Battalion>>();

    if fliers.len() > 0 {
        if starting_direction == StartingDirection::EAST {
            // determine if past all enemy battalions
            fliers.iter_mut().for_each(|b| {
                // find furthest defender position
                // enemy position will be going down
                let enemy_position = enemy_army.iter().fold(-150, |acc, cur| {
                    if cur.position > acc {
                        return cur.position;
                    }
                    acc
                });
                println!("{enemy_position}");

                if b.position > enemy_position && b.is_reverse_direction == false {
                    b.set_is_reverse_direction(true);
                }
            });
        } else {
            // determine if past all enemy battalions
            // enemy position will be going up
            fliers.iter_mut().for_each(|b| {
                // find furthest defender position
                let enemy_position = enemy_army.iter().fold(150, |acc, cur| {
                    if cur.position < acc {
                        return cur.position;
                    }
                    acc
                });

                if b.position < enemy_position && b.is_reverse_direction == false {
                    b.set_is_reverse_direction(true);
                }
            });
        }
    }
}
