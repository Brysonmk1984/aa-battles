use crate::enums::{ArmorType, ArmyName, WeaponType};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Army {
    pub id: i32,
    pub name: ArmyName,
    pub count: i32,
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
}
