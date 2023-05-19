use anyhow::Result;
use shogi_alg::{game::*, inference::Inference};
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::{env, path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let game_number = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };
    let generation = if args.len() > 2 {
        args[2].parse::<i32>().unwrap_or_default()
    } else {
        0
    };
    let para = if args.len() > 3 {
        let para = args[3].parse::<usize>().unwrap_or(1);
        match para {
            0 => 1,
            _ => para,
        }
    } else {
        num_cpus::get()
    };

    let game_number = game_number / para;
    let mut tasks = vec![];
    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;
    for _ in 0..para {
        tasks.push(tokio::spawn(train_task(
            pool.clone(),
            game_number,
            generation,
        )));
    }
    futures::future::try_join_all(tasks).await?;
    Ok(())
}

async fn train_task(
    pool: sqlx::sqlite::SqlitePool,
    game_number: usize,
    generation: i32,
) -> Result<()> {
    let inference = Arc::new(Inference::init(generation)?);

    let mut elapsed_list = vec![];
    for _ in 0..game_number {
        elapsed_list.push(game_task(pool.clone(), generation, inference.clone()).await?);
    }
    let avg = elapsed_list.iter().sum::<u128>() / elapsed_list.len() as u128;
    println!("Average time: {} (micro sec)/move", avg);
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool, generation: i32, inf: Arc<Inference>) -> Result<u128> {
    let mut game = Game::new(true, pool, inf);
    let mut count = 0;
    let start = std::time::Instant::now();
    loop {
        match game.next()? {
            GameState::Checkmate(_color) => {
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
    game.save(generation).await?;

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
