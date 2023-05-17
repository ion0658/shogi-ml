mod board;
mod game;
mod piece;

use anyhow::Result;
use game::Game;

fn main() -> Result<()> {
    let mut game = Game::new();
    loop {
        game.print();
        let state = game.next();
        match state {
            game::GameState::Checkmate(color) => {
                println!("Checkmate! {:?} is Winner!", color);
                break;
            }
            _ => {}
        }
        std::thread::yield_now();
    }

    Ok(())
}
