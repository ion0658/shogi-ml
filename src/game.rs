use crate::{
    board::{create_initial_board, create_move_range, Board},
    piece::{Color, PieceType},
};
use anyhow::Result;

pub struct Game {
    board: Board,
    turn: Color,
    _hand_black: Vec<PieceType>,
    _hand_white: Vec<PieceType>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: create_initial_board(),
            turn: Color::Black,
            _hand_black: Vec::new(),
            _hand_white: Vec::new(),
        }
    }
    pub fn print(&self) {
        self.board.iter().for_each(|row| {
            row.iter().for_each(|p| match p {
                Some(piece) => print!("| {} ", piece),
                None => print!("|{: >4}", ""),
            });
            println!("|")
        });
    }

    pub fn next(&mut self) -> Result<()> {
        let _ = create_move_range(&self.board, self.turn);
        {
            self.turn = self.turn.opponent();
        }
        Ok(())
    }

    pub fn checkmate(&self) -> bool {
        true
    }
}
