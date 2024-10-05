use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::battle::tick::run_tick::run_tick;
use crate::enums::{ArmyName, StartingDirection};
use crate::mocks::nation_army::NationArmyMock;
use crate::util::{push_log, push_stat_kill, Stats};
use crate::{
    battle::determine_win_conditions::{
        check_for_king_captured_condition, determine_army_conquered_condition,
    },
    util::determine_aoe_effect,
};

use super::battalion::Battalion;

impl Battalion {
    pub fn set_is_marching(&mut self, march: bool, enemy_engaging_with: Option<&ArmyName>) {
        if self.is_marching != march && march == true {
            push_log(format!("{} are now marching", self.name));
        } else if self.is_marching != march && march == false && enemy_engaging_with.is_some() {
            push_log(format!(
                "{} are now engaging with {} ",
                self.name,
                enemy_engaging_with.unwrap()
            ));
        }
        self.is_marching = march;
    }

    pub fn set_is_reverse_direction(&mut self, value: bool) {
        push_log(format!("A ground battalion has passed under the {} battalion, causing the fliers to reverse direction.", self.name));
        self.is_reverse_direction = value;
    }

    /**
     * If Starting direction is EAST, army starts at -150 and marches west, WEST starts at 150 and marches east
     */
    pub fn march(&mut self, starting_direction: StartingDirection) {
        if let StartingDirection::EAST = starting_direction {
            self.position += self.speed;
        } else {
            self.position -= self.speed;
        }

        if self.position < -150 || self.position > 150 {
            panic!(
                "{} are out of battlefield bounds - {}!",
                self.name, self.position
            );
        }
    }
}