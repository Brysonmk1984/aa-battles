use crate::match_up::match_up::Battalion;

pub fn march(army: &mut Vec<Battalion>) {
    army.iter_mut().for_each(|a| {
        if a.is_marching {
            a.march()
        }
    })
}
