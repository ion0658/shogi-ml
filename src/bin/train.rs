use anyhow::Result;
use shogi_alg::{db::get_connection, game::*, inference::Inference, piece::Color};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;

    train_task(pool.clone()).await?;
    Ok(())
}

async fn train_task(pool: sqlx::sqlite::SqlitePool) -> Result<()> {
    game_task(pool.clone()).await?;
    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool) -> Result<()> {
    let inf: Arc<Inference> = Arc::new(Inference::init()?);
    let mut game = Game::new(pool, inf);
    let mut hand_count: usize = 1;
    let mut black_hand_time = Duration::from_secs(100);
    let mut white_hand_time = Duration::from_secs(100);
    let mut start_black = std::time::Instant::now();
    let mut start_white = std::time::Instant::now();
    loop {
        let result = game.next()?;
        match game.current_turn() {
            Color::Black => {
                let elapsed: Duration = start_white.elapsed();
                if white_hand_time < elapsed {
                    println!("white handtime is out!");
                    break;
                }
                white_hand_time -= elapsed;
                start_black = std::time::Instant::now();
            }
            Color::White => {
                let elapsed = start_black.elapsed();
                if black_hand_time < elapsed {
                    println!("black handtime is out!");
                    break;
                }
                black_hand_time -= elapsed;
                start_white = std::time::Instant::now();
            }
        }
        match result {
            GameState::Checkmate(_color) => {
                break;
            }
            _ => {
                hand_count += 1;
            }
        }
    }
    game.print();
    println!("{:?} win.({} hands)", game.current_turn(), hand_count);
    game.save().await?;

    Ok(())
}
