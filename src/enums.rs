use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum StartingDirection {
    #[default]
    EAST,
    WEST,
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
