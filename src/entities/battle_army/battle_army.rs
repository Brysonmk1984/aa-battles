use serde::{Deserialize, Serialize};

use crate::entities::battalion::battalion::Battalion;

/**
 * A full_army along with the nation it's associated with
 */
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BattleArmy {
    pub nation_id: i32,
    pub full_army: Vec<Battalion>,
}
