use std::sync::atomic::AtomicU32;

use crate::enums::StartingDirection;

#[derive(Default)]
pub struct PartialBattalionForTests {
    pub count: Option<AtomicU32>,
    pub position: Option<i32>,
    pub speed: Option<i32>,
    pub flying: Option<bool>,
    pub range: Option<i32>,
    pub aoe: Option<f64>,
    pub spread: Option<f64>,
    pub starting_direction: Option<StartingDirection>,
}
