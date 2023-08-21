use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::battle::tick::run_tick::run_tick;
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
                //dbg!(&winner_by_position);
                battle_result.win_type = Some(WinType::KingCaptured);
                battle_result.loser =
                    if winner_by_position.as_ref().unwrap() == &Belligerent::WesternArmy {
                        Some(Belligerent::EasternArmy)
                    } else {
                        Some(Belligerent::WesternArmy)
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
            //println!("WEST ARMY COUNT: {a1_count}, EAST ARMY COUNT: {a2_count}");

            battle_result.tick_count += 1;
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
    pub fn format_battle_state(&mut self, battle_result: &BattleResult) -> String {
        let mut winning_army: (Belligerent, String);
        let mut losing_army: (Belligerent, String);
        if let Belligerent::WesternArmy = battle_result.winner.as_ref().unwrap() {
            winning_army = (
                Belligerent::WesternArmy,
                self.format_army_state(Belligerent::WesternArmy),
            );
            losing_army = (
                Belligerent::EasternArmy,
                self.format_army_state(Belligerent::EasternArmy),
            );
        } else {
            winning_army = (
                Belligerent::EasternArmy,
                self.format_army_state(Belligerent::WesternArmy),
            );
            losing_army = (
                Belligerent::WesternArmy,
                self.format_army_state(Belligerent::EasternArmy),
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
    fn format_army_state(&mut self, belligerent: Belligerent) -> String {
        let mut formatted_vec = if belligerent == Belligerent::WesternArmy {
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

        format!("{formatted_vec}")
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
    pub fn decrement(&mut self, attacker_aoe: f64) {
        let hits = determine_aoe_effect(attacker_aoe, self.spread) as i32;
        let new_count = self.count - hits;
        if new_count > 0 {
            self.count = new_count;
        } else {
            self.count = 0;
        }
    }

    pub fn set_is_marching(&mut self, value: bool) {
        //println!("setting is marching {value}");
        self.is_marching = value;
    }

    pub fn set_is_reverse_direction(&mut self, value: bool) {
        //println!("{value} HAS REVERSED ");
        self.is_reverse_direction = value;
    }

    /**
     * If Starting direction is west, army starts at -150 and marches east, west starts at 150 and marches east
     */
    pub fn march(&mut self, starting_direction: StartingDirection) {
        if let StartingDirection::WEST = starting_direction {
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
#[derive(Debug, Clone)]
pub struct BattleArmy {
    pub nation_id: i32,
    pub full_army: Vec<Battalion>,
}

// NOTE: in order to use .sort(), these four traits are required.
// Otherwise, you can skip these attributes and just use sort_by along with .cmp()
//#[derive(Eq, Ord, PartialEq, PartialOrd)]

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Army {
    pub id: i32,
    pub name: ArmyName,
    pub count: i32,
    #[serde(deserialize_with = "as_f64")]
    pub shield_rating: f64,
    pub flying: bool,
    pub range: i32,
    pub attack_speed: i32,
    #[serde(deserialize_with = "as_f64")]
    pub accuracy: f64,
    #[serde(deserialize_with = "as_f64")]
    pub aoe: f64,
    #[serde(deserialize_with = "as_f64")]
    pub spread: f64,
    pub weapon_type: WeaponType,
    pub armor_type: ArmorType,
    #[serde(deserialize_with = "as_f64")]
    pub agility: f64,
    pub speed: i32,
}

#[derive(
    Display, Debug, Clone, Default, Deserialize, Serialize, EnumString, PartialEq, Eq, Hash,
)]
pub enum ArmyName {
    #[serde(rename = "Amazonian Huntresses")]
    #[strum(serialize = "amazonian_huntresses")]
    AmazonianHuntresses,
    #[serde(rename = "Avian Cliff Dwellers")]
    #[strum(serialize = "avian_cliff_dwellers")]
    AvianCliffDwellers,
    #[serde(rename = "Highborn Cavalry")]
    #[strum(serialize = "highborn_cavalry")]
    HighbornCavalry,
    #[serde(rename = "Imperial Legionnaires")]
    #[strum(serialize = "imperial_legionnaires")]
    ImperialLegionnaires,
    #[serde(rename = "Magi Enforcers")]
    #[strum(serialize = "magi_enforcers")]
    MagiEnforcers,
    #[serde(rename = "North Watch Longbowmen")]
    #[strum(serialize = "north_watch_longbowmen")]
    NorthWatchLongbowmen,
    #[serde(rename = "Peacekeeper Monks")]
    #[strum(serialize = "peacekeeper_monks")]
    PeacekeeperMonks,
    #[serde(rename = "Rōnin Immortals")]
    #[strum(serialize = "ronin_immortals")]
    RoninImmortals,
    #[serde(rename = "Shinobi Martial Artists")]
    #[strum(serialize = "shinobi_martial_artists")]
    ShinobiMartialArtists,
    #[serde(rename = "Skull Clan Death Cultists")]
    #[strum(serialize = "skull_clan_death_cultists")]
    SkullClanDeathCultists,
    #[serde(rename = "Barbarians of the Outer Steppe")]
    #[strum(serialize = "Barbarians of the Outer Steppe")]
    BarbariansOfTheOuterSteppe,
    #[serde(rename = "Oath-Sworn Knights")]
    #[strum(serialize = "oath-sworn_knights")]
    OathSwornKnights,
    #[default]
    #[serde(rename = "Militia")]
    #[strum(serialize = "militia")]
    Militia,
    #[serde(rename = "Hooded Assassins")]
    #[strum(serialize = "hooded_assassins")]
    HoodedAssassins,
    #[serde(rename = "Elven Archers")]
    #[strum(serialize = "elven_archers")]
    ElvenArchers,
    #[serde(rename = "Castlegate Crossbowmen")]
    #[strum(serialize = "castlegate_crossbowmen")]
    CastlegateCrossbowmen,
}

#[serde(rename_all = "snake_case")]
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
}

#[derive(Debug, Display, PartialEq)]
pub enum Belligerent {
    #[strum(serialize = "Western Army")]
    WesternArmy,
    #[strum(serialize = "Eastern Army")]
    EasternArmy,
}

#[derive(Debug, Display, PartialEq)]
pub enum WinType {
    #[strum(serialize = "Army Conquered")]
    ArmyConquered,
    #[strum(serialize = "King Captured")]
    KingCaptured,
}

#[derive(Debug, PartialEq, Default)]
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
    pub fn format_outcome(self) -> String {
        let result = format!(
            "Battle ID: {}\n{} Wins\n{}\nTick Count: {}",
            self.id,
            self.winner.unwrap().to_string(),
            self.win_type.unwrap().to_string(),
            self.tick_count
        );
        format!("\nBATTLE RESULTS:\n-------------\n{result}\n")
    }
}
