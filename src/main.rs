use service::query;
use std::error::Error;

use crate::{battle::battle::run_battle, match_up::match_up::get_battle_tuple};
mod battle;
mod match_up;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let army_defaults = query::get_all_armies().await.unwrap();
    let tuple = get_battle_tuple(1, 2, army_defaults);
    let battle_result = run_battle(tuple);

    //println!("{battle_result:?}");

    Ok(())
}
