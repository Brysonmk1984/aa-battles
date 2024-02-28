use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::battle::tick::run_tick::run_tick;
use crate::util::{push_log, push_stat_kill, Stats};
use crate::{
    battle::determine_win_conditions::{
        check_for_king_captured_condition, determine_army_conquered_condition,
    },
    util::determine_aoe_effect,
};
#[derive(Debug)]
pub struct Battle {
    pub army_1_state: Vec<Battalion>,
    pub army_2_state: Vec<Battalion>,
}

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
            sum += b.count;
            sum
        });
        let mut a2_count = self.army_2_state.iter().fold(0, |mut sum, b| {
            sum += b.count;
            sum
        });
        let mut total_army_count = a1_count + a2_count;

        let mut battle_result = BattleResult {
            id: 1,
            winner: None,
            loser: None,
            tick_count: 0,
            win_type: None,
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
                sum += b.count;
                sum
            });
            a2_count = self.army_2_state.iter().fold(0, |mut sum, b| {
                sum += b.count;
                sum
            });
            battle_result.tick_count += 1;
            push_log(format!("TICK {}", battle_result.tick_count));

            if battle_result.tick_count > 300 {
                panic!("Infinite loop detected!");
            }

            total_army_count = run_tick(self);
        }

        determine_army_conquered_condition(battle_result, a1_count, a2_count)
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
            self.army_1_state.sort_by(|a, b| b.count.cmp(&a.count));
            self.army_1_state
                .iter()
                .map(|b| format!("{} - {} at position {}", b.name, b.count, b.position))
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            self.army_2_state.sort_by(|a, b| b.count.cmp(&a.count));
            self.army_2_state
                .iter()
                .map(|b| format!("{} - {} at position {}", b.name, b.count, b.position))
                .collect::<Vec<String>>()
                .join("\n")
        };

        format!("{formatted_vec}{stats}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum StartingDirection {
    #[default]
    EAST,
    WEST,
}

// An Army Type with count belonging to a user. Forms a part of a whole nation's army
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Battalion {
    pub name: ArmyName,
    pub count: i32,
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
    pub is_marching: bool,
    pub starting_direction: StartingDirection,
    pub is_reverse_direction: bool,
}

impl Battalion {
    pub fn decrement(&mut self, attacker_aoe: f64, attacker_starting_direction: StartingDirection) {
        let hits = determine_aoe_effect(attacker_aoe, self.spread) as i32;
        push_stat_kill(hits as u32, attacker_starting_direction);
        let new_count = self.count - hits;
        if new_count > 0 {
            self.count = new_count;
        } else {
            self.count = 0;
        }
    }

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

// Full Army a user will use to battle
#[derive(Debug, Clone, Deserialize, Serialize, Default)]

pub struct BattleArmy {
    pub nation_id: i32,
    pub full_army: Vec<Battalion>,
}

impl BattleArmy {
    pub fn log_prebattle_count(&self) -> String {
        let count_by_battalion: Vec<String>;

        let result = self
            .full_army
            .iter()
            .fold(vec![], |mut acc: Vec<String>, b: &Battalion| {
                acc.push(format!("{} {}", b.count, b.name));
                acc
            })
            .join(", ");

        result
    }
}

// NOTE: in order to use .sort(), these four traits are required.
// Otherwise, you can skip these attributes and just use sort_by along with .cmp()
//#[derive(Eq, Ord, PartialEq, PartialOrd)]

#[derive(Debug, Clone, Default)]
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

#[derive(
    Display, Debug, Clone, Copy, Default, Deserialize, Serialize, EnumString, PartialEq, Eq, Hash,
)]
pub enum ArmyName {
    #[serde(rename = "Amazonian Huntresses")]
    #[strum(serialize = "Amazonian Huntresses")]
    AmazonianHuntresses,
    #[serde(rename = "Avian Cliff Dwellers")]
    #[strum(serialize = "Avian Cliff Dwellers")]
    AvianCliffDwellers,
    #[serde(rename = "Highborn Cavalry")]
    #[strum(serialize = "Highborn Cavalry")]
    HighbornCavalry,
    #[serde(rename = "Imperial Legionnaires")]
    #[strum(serialize = "Imperial Legionnaires")]
    ImperialLegionnaires,
    #[serde(rename = "Magi Enforcers")]
    #[strum(serialize = "Magi Enforcers")]
    MagiEnforcers,
    #[serde(rename = "North Watch Longbowmen")]
    #[strum(serialize = "North Watch Longbowmen")]
    NorthWatchLongbowmen,
    #[serde(rename = "Peacekeeper Monks")]
    #[strum(serialize = "Peacekeeper Monks")]
    PeacekeeperMonks,
    #[serde(rename = "Rōnin Immortals")]
    #[strum(serialize = "Rōnin Immortals")]
    RoninImmortals,
    #[serde(rename = "Shinobi Martial Artists")]
    #[strum(serialize = "Shinobi Martial Artists")]
    ShinobiMartialArtists,
    #[serde(rename = "Skull Clan Death Cultists")]
    #[strum(serialize = "Skull Clan Death Cultists")]
    SkullClanDeathCultists,
    #[serde(rename = "Barbarians of the Outer Steppe")]
    #[strum(serialize = "Barbarians of the Outer Steppe")]
    BarbariansOfTheOuterSteppe,
    #[serde(rename = "Oath-Sworn Knights")]
    #[strum(serialize = "Oath-Sworn Knights")]
    OathSwornKnights,
    #[default]
    #[serde(rename = "Minute Men Militia")]
    #[strum(serialize = "Minute Men Militia")]
    MinuteMenMilitia,
    #[serde(rename = "Death Dealer Assassins")]
    #[strum(serialize = "Death Dealer Assassins")]
    DeathDealerAssassins,
    #[serde(rename = "Elven Archers")]
    #[strum(serialize = "Elven Archers")]
    ElvenArchers,
    #[serde(rename = "Castlegate Crossbowmen")]
    #[strum(serialize = "Castlegate Crossbowmen")]
    CastlegateCrossbowmen,
}

#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
#[derive(Debug, Clone, Copy, Default, Display, Deserialize, Serialize, EnumString, PartialEq)]
pub enum WeaponType {
    Blunt,
    #[default]
    Edged,
    Piercing,
    Crushing,
    Magic,
}

#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
#[derive(Debug, Display, Clone, Copy, Default, Deserialize, Serialize, EnumString, PartialEq)]
pub enum ArmorType {
    #[default]
    Unarmored,
    Leather,
    Chain,
    Plate,
}

#[derive(Default)]
pub struct PartialBattalionForTests {
    pub count: Option<i32>,
    pub position: Option<i32>,
    pub speed: Option<i32>,
    pub flying: Option<bool>,
    pub range: Option<i32>,
    pub aoe: Option<f64>,
    pub spread: Option<f64>,
    pub starting_direction: Option<StartingDirection>,
}

#[derive(Serialize, Debug, Display, PartialEq)]
pub enum Belligerent {
    #[strum(serialize = "Western Army")]
    WesternArmy,
    #[strum(serialize = "Eastern Army")]
    EasternArmy,
}

#[derive(Serialize, Debug, Display, PartialEq)]
pub enum WinType {
    #[strum(serialize = "Army Conquered")]
    ArmyConquered,
    #[strum(serialize = "King Captured")]
    KingCaptured,
}

#[derive(Serialize, Debug, PartialEq, Default)]
pub struct BattleResult {
    pub id: i32,
    pub winner: Option<Belligerent>,
    pub loser: Option<Belligerent>,
    pub tick_count: u16,
    pub win_type: Option<WinType>,
}

impl BattleResult {
    /**
     * Formats the final tally and outcome to be printed to the command line and the log
     */
    pub fn format_outcome(&self) -> String {
        let result = format!(
            "Battle ID: {}\n{} Wins\n{}\nTick Count: {}",
            self.id,
            self.winner.as_ref().unwrap().to_string(),
            self.win_type.as_ref().unwrap().to_string(),
            self.tick_count
        );
        format!("\nBATTLE RESULTS:\n-------------\n{result}\n")
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Nation {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub gold: i32,
    pub is_npc: bool,
}
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct NationArmy {
    pub id: i32,
    pub nation_id: i32,
    pub army_id: i32,
    pub count: i32,
    pub army_name: ArmyName,
}

pub struct GameDefaults {
    pub weapons_vs_armor: &'static HashMap<&'static str, f64>,
    pub army_defaults: &'static HashMap<ArmyName, Army>,
    pub environment: String,
}
