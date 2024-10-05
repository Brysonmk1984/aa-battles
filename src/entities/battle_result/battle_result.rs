use crate::{
    entities::ending_battalion_stats::EndingBattalionStats,
    enums::{Belligerent, WinType},
};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Default)]
pub struct BattleResult {
    pub id: i32,
    pub winner: Option<Belligerent>,
    pub loser: Option<Belligerent>,
    pub tick_count: u16,
    pub win_type: Option<WinType>,
    pub eastern_battalions: Vec<EndingBattalionStats>,
    pub western_battalions: Vec<EndingBattalionStats>,
}
