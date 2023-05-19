use crate::{
    board::{
        create_initial_board, create_move_range, get_num_array, is_checked, move_piece,
        print_boards, select_best_board, Boards,
    },
    piece::Color,
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
    boards_record: Vec<Boards>,
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

    pub async fn save(&self, generation: i32) -> Result<()> {
        let records = self
            .boards_record
            .iter()
            .map(|boards| get_num_array(boards))
            .collect::<Vec<_>>();
        let record = records.as_slice().concat().concat().concat();
        let query = sqlx::query("INSERT INTO KIFU (WINNER, GENERATION, RECORDS) VALUES (?, ?, ?)")
            .bind(self.turn.opponent() as i8)
            .bind(generation)
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
                let checked = is_checked(&boards[0], self.turn);
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
        let best_boards = select_best_board(&next_boards, self.turn);
        self.boards = best_boards;
        self.boards_record.push(best_boards);
        self.turn = self.turn.opponent();
        std::thread::yield_now();
        GameState::Playing
    }
}
