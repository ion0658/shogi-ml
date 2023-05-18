use anyhow::Result;
use shogi_alg::game::*;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::{env, path::Path};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let game_number = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };
    run(game_number).await?;
    Ok(())
}

async fn run(game_number: usize) -> Result<()> {
    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;
    let mut tasks = vec![];
    for _ in 0..game_number {
        tasks.push(tokio::spawn(game_task(pool.clone())));
    }
    let elapsed_list = futures::future::try_join_all(tasks)
        .await?
        .iter()
        .filter_map(|e| match e {
            Ok(elapsed) => Some(*elapsed),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    let avg = elapsed_list.iter().sum::<u128>() / elapsed_list.len() as u128;
    println!("Average time: {} (micro sec)/move", avg);
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool) -> Result<u128> {
    let mut game = Game::new(pool);
    let mut count = 0;
    let start = std::time::Instant::now();
    loop {
        match game.next() {
            GameState::Checkmate(_color) => {
                // println!("Checkmate! {:?} is Winner!", color);
                // game.print();
                break;
            }
            _ => {
                count += 1;
            }
        }
    }
    let elapsed = start.elapsed();
    println!(
        "Game finished in {:?} with {} moves {} [(micro sec)/move]",
        elapsed,
        count,
        elapsed.as_micros() / count
    );
    game.save().await?;

    Ok(elapsed.as_micros() / count)
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
