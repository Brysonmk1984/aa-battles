use crate::{battle::battle::run_battle, db::db::connect_db, match_up::match_up::get_battle_tuple};
mod battle;
mod db;
mod match_up;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conn = connect_db().await;
    println!("Hello, world! {:?}", conn);

    let battle_result = run_battle(get_battle_tuple(1, 2));

    //println!("{battle_result:?}");
}
