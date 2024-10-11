use crate::entities::battalion::battalion::Battalion;
use std::sync::atomic::Ordering;

use super::battle_army::BattleArmy;

impl BattleArmy {
    pub fn log_prebattle_count(&self) -> String {
        let count_by_battalion: Vec<String>;

        let result = self
            .full_army
            .iter()
            .fold(vec![], |mut acc: Vec<String>, b: &Battalion| {
                acc.push(format!("{} {}", b.count.get(), b.name));
                acc
            })
            .join(", ");

        result
    }
}
