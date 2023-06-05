use std::{sync::Arc, vec};

use crate::{
    board::{
        create_initial_board, create_move_range, get_num_array, is_checked, is_checkmate,
        move_piece, print_boards, Boards, LegalMove, BOARD_SIZE,
    },
    inference::Inference,
    piece::{Color, Piece},
};
use anyhow::Result;
use rayon::prelude::*;

pub enum GameState {
    Playing,
    Checkmate(Color),
}

pub struct Game {
    boards: Boards,
    turn: Color,
    inference: Arc<Inference>,
    boards_record: Vec<Boards>,
    pool: sqlx::SqlitePool,
}

impl Game {
    // mode true: train, false: play
    pub fn new(pool: sqlx::SqlitePool, inference: Arc<Inference>) -> Self {
        let boards = create_initial_board();
        Game {
            boards,
            turn: Color::Black,
            inference,
            boards_record: vec![],
            pool,
        }
    }
    #[allow(unused)]
    pub fn print(&self) {
        print_boards(&self.boards)
    }

    pub fn current_turn(&self) -> Color {
        self.turn
    }

    pub async fn save(&self) -> Result<()> {
        let records = self
            .boards_record
            .iter()
            .map(|boards| get_num_array(boards))
            .collect::<Vec<_>>();
        let record = records.as_slice().concat().concat().concat();
        let query = sqlx::query("INSERT INTO KIFU (WINNER, RECORDS) VALUES (?, ?)")
            .bind(self.turn as i8)
            .bind(&record);
        query.execute(&self.pool).await?;
        Ok(())
    }

    pub fn next(&mut self) -> Result<GameState> {
        let move_range = create_move_range(&self.boards, self.turn);
        let legal_boards = move_range
            .par_iter()
            .filter_map(|range| {
                let boards = move_piece(self.boards, *range);
                // 打ち歩詰めは除外
                if range.from.z == 1
                    && (range.from.y < 2 || range.from.y > BOARD_SIZE as i32 - 2)
                    && is_checkmate(&boards, self.turn.opponent())
                {
                    return None;
                }
                Some(boards)
            })
            .collect::<Vec<_>>();
        let checkmate_boards = legal_boards
            .par_iter()
            .filter_map(|&boards| {
                if is_checkmate(&boards, self.turn.opponent()) {
                    Some(boards)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // 詰められるときはそれを使う
        if let Some(checkmate_board) = checkmate_boards.first() {
            self.boards = *checkmate_board;
            self.boards_record.push(*checkmate_board);
            println!("turn: {:?}", self.turn);
            return Ok(GameState::Checkmate(self.turn));
        }

        // 王手が解除できない or 自殺手は除外 or 千日手
        let next_boards = legal_boards
            .par_iter()
            .filter(|&boards| {
                if is_checked(&boards[0], self.turn) {
                    return false;
                }
                if self
                    .boards_record
                    .par_iter()
                    .find_first(|&r| r == boards)
                    .is_some()
                {
                    return false;
                }
                true
            })
            .cloned()
            .collect::<Vec<_>>();

        // 打てる手がない場合は詰み
        if next_boards.len() == 0 {
            self.turn = self.turn.opponent();
            return Ok(GameState::Checkmate(self.turn));
        }
        // 打てる手の中から最善を選択
        let best_boards = self.inference.select_best_board(&next_boards, self.turn)?;

        // 盤面の更新
        self.boards = best_boards;
        self.boards_record.push(best_boards);
        self.turn = self.turn.opponent();
        Ok(GameState::Playing)
    }

    pub fn get_legal_moves(&self) -> Result<Vec<(Piece, LegalMove)>, GameState> {
        let move_range = create_move_range(&self.boards, self.turn);
        let moves = move_range
            .iter()
            .filter(|&range| {
                let boards = move_piece(self.boards, *range);
                // 自殺手・打ち歩詰めは除外
                !(range.from.z == 1
                    && (range.from.y < 2 || range.from.y > BOARD_SIZE as i32 - 2)
                    && is_checkmate(&boards, self.turn.opponent()))
                    && !is_checked(&boards[0], self.turn)
            })
            .cloned()
            .collect::<Vec<_>>();
        if moves.len() == 0 {
            return Err(GameState::Checkmate(self.turn.opponent()));
        }
        let moves = moves
            .iter()
            .map(|m| {
                let p =
                    self.boards[m.from.z as usize][m.from.y as usize][m.from.x as usize].unwrap();
                (p, m.clone())
            })
            .collect();
        Ok(moves)
    }

    pub fn play_next(&mut self, movement: &LegalMove) {
        let boards = move_piece(self.boards, *movement);
        self.boards = boards;
        self.boards_record.push(boards);
        self.turn = self.turn.opponent();
    }
}
