mod board;
mod game;
mod piece;

use anyhow::Result;
use game::Game;

fn main() -> Result<()> {
    let mut game = Game::new();
    game.print();
    //while {
    game.next()?;
    game.print();
    game.next()?;
    game.print();

    //    !game.checkmate()
    //} {}
    Ok(())
}
