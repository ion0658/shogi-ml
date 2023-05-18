use crate::{
    board::{
        create_initial_board, create_move_range, get_num_array, move_piece, print_boards,
        select_best_board, Board, Boards, BoardsAsNum, Position, BOARD_SIZE,
    },
    piece::{Color, PieceType},
};
use anyhow::Result;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub enum GameState {
    Playing,
    Checkmate(Color),
}

#[derive(Debug, Serialize, Deserialize)]
struct GameResult {
    winner: i8,
    records: Vec<BoardsAsNum>,
}

pub struct Game {
    boards: Boards,
    turn: Color,
    boards_record: Vec<BoardsAsNum>,
    pool: sqlx::SqlitePool,
}

impl Game {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        let boards = create_initial_board();
        Game {
            boards,
            turn: Color::Black,
            boards_record: vec![],
            pool,
        }
    }
    #[allow(unused)]
    pub fn print(&self) {
        print_boards(&self.boards)
    }

    pub async fn save(&self) -> Result<()> {
        let result = GameResult {
            winner: self.turn.opponent() as i8,
            records: self.boards_record.clone(),
        };
        let record = result.records.as_slice().concat().concat().concat();
        let query = sqlx::query("INSERT INTO KIFU (WINNER, RECORDS) VALUES (?, ?)")
            .bind(result.winner)
            .bind(&record);
        query.execute(&self.pool).await?;
        Ok(())
    }

    pub fn next(&mut self) -> GameState {
        let move_range = create_move_range(&self.boards, self.turn);
        let next_boards = move_range
            .par_iter()
            .filter_map(|range| {
                let boards = move_piece(self.boards, *range);
                let checked = Self::is_checked(&boards[0], self.turn);
                if checked {
                    None
                } else {
                    Some(boards)
                }
            })
            .collect::<Vec<_>>();
        if next_boards.len() == 0 {
            return GameState::Checkmate(self.turn.opponent());
        }
        let best_boards = select_best_board(&next_boards);
        self.boards = best_boards;
        self.boards_record.push(get_num_array(&best_boards));
        self.turn = self.turn.opponent();
        std::thread::yield_now();
        GameState::Playing
    }

    fn is_checked(board: &Board, color: Color) -> bool {
        // 王の位置を検索
        let king_position = Self::find_king_position(board, color);

        board.par_iter().enumerate().any(|(y, row)| {
            row.par_iter().enumerate().any(|(x, piece)| {
                if let Some(piece) = piece {
                    if piece.color != color {
                        return piece.can_capture_king(
                            Position::new(x as i32, y as i32, 0),
                            board,
                            king_position,
                        );
                    }
                }
                false
            })
        })
    }

    // 王の位置を検索するヘルパー関数
    fn find_king_position(board: &Board, color: Color) -> Position {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if let Some(piece) = board[y][x] {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Position::new(x as i32, y as i32, 0);
                    }
                }
            }
        }

        panic!("King not found on the board!");
    }
}
