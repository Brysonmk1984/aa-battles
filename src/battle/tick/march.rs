use crate::match_up::match_up::Battalion;

pub fn march(non_attacking_battalions: &mut Vec<Battalion>) {
    let non_attacking_battalions: &mut Vec<Battalion> = in_range_map_1
        .into_iter()
        .filter(|(_, defender_vec)| defender_vec.is_empty())
        .map(|(b_name, _)| {
            *battle_state
                .army_1_state
                .iter()
                .find(|battalion| battalion.name == b_name)
                .unwrap()
        })
        .collect();

    non_attacking_battalions.iter_mut().for_each(|battalion| {
        let n = battalion.name.clone();
        let p = battalion.position;
        let s = battalion.speed;
        //println!("{n} is at {p}");

        battalion.set_is_marching(true);
        battalion.march();

        let n = battalion.name.clone();
        let p = battalion.position;
        let s = battalion.speed;
        println!("{n} marched to {p} with a speed of {s}");
    });
}
