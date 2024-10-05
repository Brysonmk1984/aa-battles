use std::sync::atomic::Ordering;

use crate::{
    battle::{
        determine_win_conditions::{
            check_for_king_captured_condition, determine_army_conquered_condition,
        },
        tick::run_tick::run_tick,
    },
    entities::{battalion::battalion::Battalion, battle_result::battle_result::BattleResult},
    enums::{Belligerent, WinType},
    util::push_log,
};

use super::battle::Battle;

impl Battle {
    /**
     * Keeps tally of the 2 army counts & the battle result
     * As long as the counts are not zero it loops, checking if there's been a check_for_king_captured_condition
     * If not, runs ticks to increment the army positions and attacks along
     * Finally, checks if the determine_army_conquered_condition is met
     */
    pub fn run_battle(&mut self) -> BattleResult {
        push_log(
            "THE BATTLE BEGINS: Both Eastern & Western Army are marching towards each other"
                .to_string(),
        );
        let mut a1_count = self.army_1_state.iter().fold(0, |mut sum, b| {
            sum += b.count.load(Ordering::Acquire);
            sum
        });
        let mut a2_count = self.army_2_state.iter().fold(0, |mut sum, b| {
            sum += b.count.load(Ordering::Acquire);
            sum
        });
        let mut total_army_count = a1_count + a2_count;

        let mut battle_result = BattleResult {
            id: 1,
            winner: None,
            loser: None,
            tick_count: 0,
            win_type: None,
            eastern_battalions: vec![],
            western_battalions: vec![],
        };

        while a1_count > 0 && a2_count > 0 {
            let winner_by_position = check_for_king_captured_condition(&self);
            if winner_by_position.is_some() {
                battle_result.win_type = Some(WinType::KingCaptured);
                battle_result.loser =
                    if winner_by_position.as_ref().unwrap() == &Belligerent::EasternArmy {
                        Some(Belligerent::WesternArmy)
                    } else {
                        Some(Belligerent::EasternArmy)
                    };
                battle_result.winner = winner_by_position;
                return battle_result;
            }

            a1_count = self.army_1_state.iter().fold(0, |mut sum, b| {
                sum += b.count.load(Ordering::Acquire);
                sum
            });
            a2_count = self.army_2_state.iter().fold(0, |mut sum, b| {
                sum += b.count.load(Ordering::Acquire);
                sum
            });
            battle_result.tick_count += 1;
            push_log(format!("TICK {}", battle_result.tick_count));

            if battle_result.tick_count > 300 {
                panic!("Infinite loop detected!");
            }
            // should return more info about the tick, including how to update the battle state, rather than updating from internally
            total_army_count = run_tick(self);
        }

        let ending_army_states = (&self.army_1_state, &self.army_2_state);
        determine_army_conquered_condition(ending_army_states, battle_result, a1_count, a2_count)
    }

    /**
     * Formats a string to reflect the final battle state
     */
    pub fn format_battle_state(
        &mut self,
        battle_result: &BattleResult,
        eastern_stats: &String,
        western_stats: &String,
    ) -> String {
        let mut winning_army: (Belligerent, String);
        let mut losing_army: (Belligerent, String);
        if let Belligerent::EasternArmy = battle_result.winner.as_ref().unwrap() {
            winning_army = (
                Belligerent::EasternArmy,
                self.format_army_state(Belligerent::EasternArmy, eastern_stats),
            );
            losing_army = (
                Belligerent::WesternArmy,
                self.format_army_state(Belligerent::WesternArmy, western_stats),
            );
        } else {
            winning_army = (
                Belligerent::WesternArmy,
                self.format_army_state(Belligerent::WesternArmy, western_stats),
            );
            losing_army = (
                Belligerent::EasternArmy,
                self.format_army_state(Belligerent::EasternArmy, eastern_stats),
            );
        }

        format!(
          "\nWinner ({}):\n----------------------\n{}\n\nLoser ({}):\n----------------------\n{}\n",
          winning_army.0, winning_army.1, losing_army.0, losing_army.1
      )
    }

    /**
     * Helps format the final string of the battle state bu formatting each of the two army states
     */
    fn format_army_state(&mut self, belligerent: Belligerent, stats: &String) -> String {
        let mut formatted_vec = if belligerent == Belligerent::EasternArmy {
            self.army_1_state.sort_by(|a, b| {
                let a_count = a.count.load(Ordering::Acquire);
                let b_count = b.count.load(Ordering::Acquire);
                return b_count.cmp(&a_count);
            });
            self.army_1_state
                .iter()
                .map(|b| {
                    format!(
                        "{} - {} at position {}",
                        b.name,
                        b.count.load(Ordering::Acquire),
                        b.position
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            self.army_2_state.sort_by(|a, b| {
                let a_count = a.count.load(Ordering::Acquire);
                let b_count = b.count.load(Ordering::Acquire);
                return b_count.cmp(&a_count);
            });
            self.army_2_state
                .iter()
                .map(|b| {
                    format!(
                        "{} - {} at position {}",
                        b.name,
                        b.count.load(Ordering::Acquire),
                        b.position
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        };

        format!("{formatted_vec}{stats}")
    }
}
