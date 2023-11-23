use std::{collections::HashMap, env, error::Error};

use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use serde_this_or_that::as_f64;
use strum_macros::{Display, EnumString};

use crate::types::{Army, BattleArmy, Nation, NationArmy};

pub async fn get_all_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    let api =
        env::var("API_URL").expect("API_URL environment variable should exist but is missing");
    let full_url = api + "/armies";

    println!("{full_url}");
    let body = reqwest::get(full_url).await?;

    let all_armies = body.json::<Vec<Army>>().await?;

    Ok(all_armies)
}

pub async fn get_competing_nations(
    nation_id_1: i32,
    nation_id_2: i32,
) -> Result<((Nation, Vec<NationArmy>), (Nation, Vec<NationArmy>)), Box<dyn Error>> {
    // Need a query to get all nation_armies by nation_id + join with armies
    let api_url =
        env::var("API_URL").expect("API_URL environment variable should exist but is missing");
    let full_url = "/battles";
    let url = api_url + full_url;
    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("east_competitor", nation_id_1);
    body.insert("west_competitor", nation_id_2);

    let response = client.post(url).json(&body).send().await?;

    let body = response.json::<Vec<(Nation, Vec<NationArmy>)>>().await?;
    let competitors = (body[0].to_owned(), body[1].to_owned());

    Ok(competitors)
}
