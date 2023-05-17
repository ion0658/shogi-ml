mod board;
mod game;
mod piece;

use anyhow::Result;
use game::Game;

fn main() -> Result<()> {
    let mut game = Game::new();
    game.print();
    loop {
        #[cfg(debug_assertions)]
        game.print();

        let state = game.next();
        match state {
            game::GameState::Checkmate(color) => {
                println!("Checkmate! {:?} is Winner!", color);
                game.print();
                break;
            }
            _ => {}
        }
    }

    // game.next();
    // game.print();
    // game.next();
    // game.print();
    // game.next();
    // game.print();

    Ok(())
}
