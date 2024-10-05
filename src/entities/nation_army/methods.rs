use crate::mocks::nation_army::NationArmyMock;

use super::nation_army::NationArmy;

impl From<NationArmyMock> for NationArmy {
    fn from(m: NationArmyMock) -> Self {
        Self {
            id: m.id,
            nation_id: m.nation_id,
            army_id: m.army_id,
            count: m.count,
            army_name: m.army_name,
        }
    }
}
