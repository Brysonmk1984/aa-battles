use std::{env, error::Error};

use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::types::Army;

pub async fn get_all_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    let body = reqwest::get(
        env::var("API_URL").expect("API_URL environment variable should exist but is missing"),
    )
    .await?;

    let all_armies = body.json::<Vec<Army>>().await?;

    Ok(all_armies)
}
