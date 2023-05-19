use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::path::Path;

pub async fn get_connection() -> Result<sqlx::sqlite::SqlitePool> {
    let db_url = "sqlite:db/data.db";

    if !Path::new("./db").exists() {
        std::fs::create_dir("./db")?;
    }
    if !sqlx::Sqlite::database_exists(db_url).await? {
        println!("Creating database {}", db_url);
        Sqlite::create_database(db_url).await?;
    }

    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_lazy(db_url)?;
    Ok(pool)
}
