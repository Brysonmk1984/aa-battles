use std::{env, error::Error};

use serde::Deserialize;
use serde_this_or_that::as_f64;

pub async fn get_all_armies() -> Result<(), Box<dyn Error>> /*Vec<Army>*/ {
    let body = reqwest::get(
        env::var("API_URL").expect("API_URL environment variable should exist but is missing"),
    )
    .await?;

    let all_armies = body.json::<Vec<Army>>().await?;

    println!("TEST {all_armies:?}");

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Army {
    id: i32,
    name: String,
    lore: String,
    size: i32,
    #[serde(deserialize_with = "as_f64")]
    shield_rating: f64,
    flying: bool,
    range: i32,
    #[serde(deserialize_with = "as_f64")]
    attack_speed: f64,
    #[serde(deserialize_with = "as_f64")]
    accuracy: f64,
    aoe: bool,
    weapon_type: String,
    armor_type: String,
    #[serde(deserialize_with = "as_f64")]
    agility: f64,
    speed: i32,
}
