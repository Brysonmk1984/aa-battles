use std::collections::HashMap;

use crate::service::query::Army;

pub fn check_in_range(
    army_1_hash: HashMap<String, Vec<&str>>,
    army_2_hash: HashMap<String, Vec<&str>>,
) -> (HashMap<String, Vec<&str>>, HashMap<String, Vec<&str>>) {
    // loop through army_1 and figure out which of army_2 is in range
    // assign the names of the battalions that are in range of each property/battalion of army_1 as a vector of strings
    // repeat the above two steps for army_2
    // return the two hashmaps
}
