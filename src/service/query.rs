use std::{env, error::Error};

use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

pub async fn get_all_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    let body = reqwest::get(
        env::var("API_URL").expect("API_URL environment variable should exist but is missing"),
    )
    .await?;

    let all_armies = body.json::<Vec<Army>>().await?;

    Ok(all_armies)
}

// NOTE: in order to use .sort(), these four traits are required.
// Otherwise, you can skip these attributes and just use sort_by along with .cmp()
//#[derive(Eq, Ord, PartialEq, PartialOrd)]

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Army {
    pub id: i32,
    pub name: ArmyName,
    pub lore: String,
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
    #[serde(rename = "Outer Steppe Barbarians")]
    #[strum(serialize = "outer_steppe_barbarians")]
    OuterSteppeBarbarians,
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
