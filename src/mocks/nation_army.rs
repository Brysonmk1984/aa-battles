use crate::{entities::nation_army::nation_army::NationArmy, enums::ArmyName};

#[derive(Default)]
pub struct NationArmyMock {
    pub id: i32,
    pub nation_id: i32,
    pub army_id: i32,
    pub count: i32,
    pub army_name: ArmyName,
}

impl NationArmyMock {
    pub fn new(count: i32) -> Self {
        NationArmyMock {
            count,
            ..Default::default()
        }
    }

    fn from(n: NationArmy) -> Self {
        Self {
            id: n.id,
            nation_id: n.nation_id,
            army_id: n.army_id,
            count: n.count,
            army_name: n.army_name,
        }
    }
}
