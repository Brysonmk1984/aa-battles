use std::{env, error::Error};

use serde::{Deserialize, Serialize};
use serde_this_or_that::as_f64;

pub async fn get_all_armies() -> Result<Vec<Army>, Box<dyn Error>> /*Vec<Army>*/ {
    let body = reqwest::get(
        env::var("API_URL").expect("API_URL environment variable should exist but is missing"),
    )
    .await?;

    let all_armies = body.json::<Vec<Army>>().await?;

    //println!("TEST {all_armies:?}");

    Ok(all_armies)
}

// NOTE: in order to use .sort(), these four traits are required.
// Otherwise, you can skip these attributes and just use sort_by along with .cmp()
//#[derive(Eq, Ord, PartialEq, PartialOrd)]

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Army {
    pub id: i32,
    pub name: String,
    pub lore: String,
    pub count: i32,
    #[serde(deserialize_with = "as_f64")]
    pub shield_rating: f64,
    pub flying: bool,
    pub range: i32,
    #[serde(deserialize_with = "as_f64")]
    pub attack_speed: f64,
    #[serde(deserialize_with = "as_f64")]
    pub accuracy: f64,
    #[serde(deserialize_with = "as_f64")]
    pub aoe: f64,
    #[serde(deserialize_with = "as_f64")]
    pub spread: f64,
    pub weapon_type: String,
    pub armor_type: String,
    #[serde(deserialize_with = "as_f64")]
    pub agility: f64,
    pub speed: i32,
}
