use serde::Deserialize;

use crate::enums::ArmyName;

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub struct NationArmy {
    pub id: i32,
    pub nation_id: i32,
    pub army_id: i32,
    pub count: i32,
    pub army_name: ArmyName,
}
