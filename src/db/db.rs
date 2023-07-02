use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect_db() -> DatabaseConnection {
    Database::connect(env::var("DATABASE_URL").unwrap().to_owned())
        .await
        .expect("Database connection failed")
}
