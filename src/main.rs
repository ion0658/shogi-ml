mod board;
mod game;
mod piece;

use anyhow::Result;
use game::Game;

fn main() -> Result<()> {
    println!("Game Start");
    let mut game = Game::new();
    loop {
        #[cfg(debug_assertions)]
        game.print();
        match game.next() {
            game::GameState::Checkmate(color) => {
                println!("Checkmate! {:?} is Winner!", color);
                game.print();
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
