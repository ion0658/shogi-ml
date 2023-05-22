use anyhow::Result;
use shogi_alg::{
    db::get_connection,
    game::*,
    inference::{GameMode, Inference},
};
use std::{env, sync::Arc, time::Duration};

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

    train_task(pool.clone(), game_number).await?;
    Ok(())
}

async fn train_task(pool: sqlx::sqlite::SqlitePool, game_number: usize) -> Result<()> {
    let mut game_count: usize = 0;
    for _ in 0..game_number {
        println!("game {} start", game_count);
        let elapsed = game_task(pool.clone()).await?;
        println!("game {} finish({:?})", game_count, elapsed);
        game_count += 1;
    }
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool) -> Result<Duration> {
    let inf: Arc<Inference> = Arc::new(Inference::init(GameMode::Train)?);
    let mut game = Game::new(pool, inf);
    let start = std::time::Instant::now();
    loop {
        match game.next()? {
            GameState::Checkmate(color) => {
                println!("{:?} win", color);
                game.print();
                break;
            }
            _ => {}
        }
    }
    let elapsed = start.elapsed();
    game.save().await?;

    Ok(elapsed)
}
