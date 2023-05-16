mod board;
mod piece;

use board::{create_initial_board, create_move_range};

// ボードの初期配置を生成する関数

fn main() {
    let board = create_initial_board();
    board.iter().for_each(|row| {
        row.iter().for_each(|p| match p {
            Some(piece) => print!("| {} ", piece),
            None => print!("|{: >4}", ""),
        });
        println!("|")
    });
    let _ = create_move_range(&board);
}
