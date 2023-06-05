use anyhow::Result;
use shogi_alg::{db::get_connection, game::*, inference::Inference, piece::Color};
use std::{io::Write, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> Result<()> {
    let inference = Arc::new(Inference::init()?);
    let pool = get_connection().await?;
    sqlx::migrate!().run(&pool).await?;

    let _ = game_task(pool, inference).await?;

    Ok(())
}

async fn game_task(pool: sqlx::SqlitePool, inf: Arc<Inference>) -> Result<()> {
    println!("Start Game");
    print!("Select Color (Black: 0, White: _): ");
    std::io::stdout().flush()?;
    let player_color = match get_input() {
        0 => Color::Black,
        _ => Color::White,
    };

    let mut game = Game::new(pool, inf);
    loop {
        if player_color == game.current_turn() {
            let moves = game.get_legal_moves();
            if let Ok(moves) = moves {
                println!("Your Turn");
                println!("Current Board");
                game.print();

                moves.iter().enumerate().for_each(|(i, m)| {
                    if m.1.from.z == 0 {
                        println!(
                            "[{}]: {} => to [{}, {}] with rev = {}",
                            i, m.0, m.1.to.x, m.1.to.y, m.1.revolute
                        )
                    } else {
                        println!("[{}]: {} => to [{}, {}] 打", i, m.0, m.1.to.x, m.1.to.y)
                    }
                });

                let index = loop {
                    print!("Select Move: ");
                    std::io::stdout().flush()?;
                    let selected_num = get_input();
                    if selected_num >= moves.len() as u64 {
                        println!("Allow Range = 0..{}", moves.len() - 1);
                        continue;
                    }
                    break selected_num;
                };
                game.play_next(&moves[index as usize].1);
            } else {
                println!("You Lose!");
                game.print();
                break;
            }
        } else {
            match game.next()? {
                GameState::Checkmate(color) => {
                    if color == player_color {
                        println!("You Win");
                    } else {
                        println!("You Lose");
                    }
                    game.print();
                    break;
                }
                _ => {}
            }
        }
    }

    game.save().await?;
    Ok(())
}

fn get_input() -> u64 {
    loop {
        let mut buff = String::new();
        std::io::stdin()
            .read_line(&mut buff)
            .expect("std read line error");
        let input = buff.trim().parse::<u64>();
        match input {
            Ok(i) => break i,
            Err(_) => {
                println!("Please Input Number");
                continue;
            }
        }
    }
}
