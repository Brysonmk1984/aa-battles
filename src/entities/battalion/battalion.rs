use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use serde::{Deserialize, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::battle::tick::run_tick::run_tick;
use crate::enums::{ArmorType, ArmyName, StartingDirection, WeaponType};
use crate::mocks::nation_army::NationArmyMock;
use crate::util::{push_log, push_stat_kill, Stats};
use crate::{
    battle::determine_win_conditions::{
        check_for_king_captured_condition, determine_army_conquered_condition,
    },
    util::determine_aoe_effect,
};

// An Army Type with count belonging to a user. Forms a part of a whole nation's army
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Battalion {
    pub name: ArmyName,
    pub count: AtomicU32,
    pub position: i32,
    pub shield_rating: f64,
    pub flying: bool,
    pub range: i32,
    pub attack_speed: i32,
    pub accuracy: f64,
    pub aoe: f64,
    pub spread: f64,
    pub weapon_type: WeaponType,
    pub armor_type: ArmorType,
    pub agility: f64,
    pub speed: i32,
    pub is_marching: AtomicBool,
    pub starting_direction: StartingDirection,
    pub is_reverse_direction: bool,
}
