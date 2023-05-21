use anyhow::Result;
use shogi_alg::{db::get_connection, game::*, inference::Inference};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let game_number = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };

    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;
    let inference = Arc::new(Inference::init()?);
    train_task(pool.clone(), game_number, inference.clone()).await?;
    Ok(())
}

async fn train_task(
    pool: sqlx::sqlite::SqlitePool,
    game_number: usize,
    inference: Arc<Inference>,
) -> Result<()> {
    let mut elapsed_list = vec![];
    for _ in 0..game_number {
        elapsed_list.push(game_task(pool.clone(), inference.clone()).await?);
    }
    let avg = elapsed_list.iter().sum::<u128>() / elapsed_list.len() as u128;
    println!("Average time: {} (micro sec)/move", avg);
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool, inf: Arc<Inference>) -> Result<u128> {
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
    game.save().await?;

    Ok(elapsed.as_micros() / count)
}
