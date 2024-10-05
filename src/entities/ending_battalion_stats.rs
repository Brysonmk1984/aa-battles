use crate::enums::ArmyName;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct EndingBattalionStats {
    pub name: ArmyName,
    pub count: i32,
    pub position: i32,
}
