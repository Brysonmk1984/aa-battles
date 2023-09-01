use std::{env, error::Error};

use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::types::{Army, BattleArmy};

pub async fn get_all_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    let body = reqwest::get(
        env::var("API_URL").expect("API_URL environment variable should exist but is missing"),
    )
    .await?;

    let all_armies = body.json::<Vec<Army>>().await?;

    Ok(all_armies)
}

pub async fn get_competing_nations(
    nation_id_1: i32,
    nation_id_2: i32,
) -> Result<(BattleArmy, BattleArmy), Box<dyn Error>> {
    // Need a query to get all nation_armies by nation_id + join with armies
}
