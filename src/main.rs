use std::error::Error;

use crate::{battle::battle::run_battle, match_up::match_up::get_battle_tuple};
mod battle;
mod match_up;
mod service;

use service::query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let result = query::get_all_armies().await?;

    //let battle_result = run_battle(get_battle_tuple(1, 2));

    //println!("{battle_result:?}");

    Ok(())
}
