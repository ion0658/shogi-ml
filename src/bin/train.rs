use anyhow::Result;
use shogi_alg::{db::get_connection, game::*, inference::Inference};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;

    train_task(pool.clone()).await?;
    Ok(())
}

async fn train_task(pool: sqlx::sqlite::SqlitePool) -> Result<()> {
    let _elapsed = game_task(pool.clone()).await?;
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool) -> Result<Duration> {
    let inf: Arc<Inference> = Arc::new(Inference::init()?);
    let mut game = Game::new(pool, inf);
    let start = std::time::Instant::now();
    loop {
        match game.next()? {
            GameState::Checkmate(_color) => {
                break;
            }
            _ => {}
        }
    }
    let elapsed = start.elapsed();
    game.save().await?;

    Ok(elapsed)
}
