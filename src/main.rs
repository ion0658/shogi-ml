mod board;
mod game;
mod piece;

use anyhow::Result;
use game::Game;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::path::Path;

const TASK_NUMBER: usize = 1_000;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;
    let mut tasks = vec![];
    for _ in 0..TASK_NUMBER {
        tasks.push(tokio::spawn(game_task(pool.clone())));
    }
    futures::future::try_join_all(tasks).await?;
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool) -> Result<()> {
    let mut game = Game::new(pool);
    loop {
        match game.next() {
            game::GameState::Checkmate(_color) => {
                // println!("Checkmate! {:?} is Winner!", color);
                // game.print();
                break;
            }
            _ => {
                tokio::task::yield_now().await;
            }
        }
    }
    game.save().await?;
    Ok(())
}

async fn get_connection() -> Result<sqlx::sqlite::SqlitePool> {
    let db_url = "sqlite:db/data.db";

    if !Path::new("./db").exists() {
        std::fs::create_dir("./db")?;
    }
    if !sqlx::Sqlite::database_exists(db_url).await? {
        println!("Creating database {}", db_url);
        Sqlite::create_database(db_url).await?;
    }

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(100)
        .connect_lazy(db_url)?;
    Ok(pool)
}
