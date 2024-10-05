use super::army::Army;
use crate::enums::ArmyName;
use std::collections::HashMap;

pub struct GameDefaults {
    pub weapons_vs_armor: HashMap<String, f64>,
    pub aoe_vs_spread: HashMap<i32, Vec<(f64, i32)>>,
    pub army_defaults: HashMap<ArmyName, Army>,
    pub environment: String,
}
