use crate::piece::{concat_vec, Color, Piece, PieceType};
use rand::prelude::*;
use rayon::prelude::*;
use std::fmt;

pub const BOARD_SIZE: usize = 9;
pub const PAGE_SIZE: usize = 2;

// ボード上の座標を表す構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    pub fn is_valid(&self) -> bool {
        self.x >= 0
            && self.x < BOARD_SIZE as i32
            && self.y >= 0
            && self.y < BOARD_SIZE as i32
            && self.z >= 0
            && self.z < PAGE_SIZE as i32
    }
}

// ボード上の駒配置を表す2次元配列
pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];
// 持ち駒を含むボード全体を表す3次元配列
pub type Boards = [Board; PAGE_SIZE];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegalMove {
    pub from: Position,
    pub to: Position,
}

pub fn create_initial_board() -> Boards {
    let mut boards: Boards = [[[None; BOARD_SIZE]; BOARD_SIZE]; PAGE_SIZE];
    boards[0] = create_initial_board_black(boards[0]);
    boards[0] = create_initial_board_white(boards[0]);
    boards
}

// 先手番の駒の初期配置を生成する関数
fn create_initial_board_black(mut board: Board) -> Board {
    // 先手の駒の配置
    // 玉将
    board[0][4] = Some(Piece::new(PieceType::King, Color::Black));
    // 金将
    board[0][3] = Some(Piece::new(PieceType::Gold, Color::Black));
    board[0][5] = Some(Piece::new(PieceType::Gold, Color::Black));
    // 銀将
    board[0][2] = Some(Piece::new(PieceType::Silver, Color::Black));
    board[0][6] = Some(Piece::new(PieceType::Silver, Color::Black));
    // 桂馬
    board[0][1] = Some(Piece::new(PieceType::Knight, Color::Black));
    board[0][7] = Some(Piece::new(PieceType::Knight, Color::Black));
    // 香車
    board[0][0] = Some(Piece::new(PieceType::Lance, Color::Black));
    board[0][8] = Some(Piece::new(PieceType::Lance, Color::Black));
    // 飛車
    board[1][1] = Some(Piece::new(PieceType::Rook, Color::Black));
    // 角行
    board[1][7] = Some(Piece::new(PieceType::Bishop, Color::Black));
    // 歩兵
    for i in 0..BOARD_SIZE {
        board[2][i] = Some(Piece::new(PieceType::Pawn, Color::Black));
    }
    board
}

// 後手番の駒の初期配置を生成する関数
fn create_initial_board_white(mut board: Board) -> Board {
    // 後手の駒の配置
    // 玉将
    board[8][4] = Some(Piece::new(PieceType::King, Color::White));
    // 金将
    board[8][3] = Some(Piece::new(PieceType::Gold, Color::White));
    board[8][5] = Some(Piece::new(PieceType::Gold, Color::White));
    // 銀将
    board[8][2] = Some(Piece::new(PieceType::Silver, Color::White));
    board[8][6] = Some(Piece::new(PieceType::Silver, Color::White));
    // 桂馬
    board[8][1] = Some(Piece::new(PieceType::Knight, Color::White));
    board[8][7] = Some(Piece::new(PieceType::Knight, Color::White));
    // 香車
    board[8][0] = Some(Piece::new(PieceType::Lance, Color::White));
    board[8][8] = Some(Piece::new(PieceType::Lance, Color::White));
    // 飛車
    board[7][7] = Some(Piece::new(PieceType::Rook, Color::White));
    // 角行
    board[7][1] = Some(Piece::new(PieceType::Bishop, Color::White));
    // 歩兵
    for i in 0..BOARD_SIZE {
        board[6][i] = Some(Piece::new(PieceType::Pawn, Color::White));
    }
    board
}

// ボード上の駒の移動範囲を生成する関数
pub fn create_move_range(boards: &Boards, turn: Color) -> Vec<LegalMove> {
    boards
        .par_iter()
        .enumerate()
        .flat_map(|(z, move_range_z)| {
            let ranges = vec![];
            let new_ranges = move_range_z
                .par_iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    let ranges = vec![];
                    let new_ranges = row
                        .par_iter()
                        .enumerate()
                        .filter(|(_, piece)| piece.is_some() && piece.unwrap().color == turn)
                        .flat_map(|(x, piece)| {
                            let range = if z == 0 {
                                piece.unwrap().create_move_range(
                                    Position::new(x as i32, y as i32, z as i32),
                                    &boards[0],
                                )
                            } else {
                                piece.unwrap().create_put_range(
                                    Position::new(x as i32, y as i32, z as i32),
                                    &boards[0],
                                )
                            };
                            range
                        })
                        .collect();
                    concat_vec(ranges, new_ranges)
                })
                .collect();
            concat_vec(ranges, new_ranges)
        })
        .collect()
}

#[allow(unused)]
pub fn get_piece_count(boards: &Boards) -> usize {
    boards
        .par_iter()
        .flat_map(|board| {
            board
                .par_iter()
                .flat_map(|row| row.par_iter().filter(|piece| piece.is_some()))
        })
        .count()
}

// 駒を動かしその結果を返す関数
pub fn move_piece(mut boards: Boards, from: Position, to: Position) -> Boards {
    let current_piece = boards[0][to.y as usize][to.x as usize];
    boards[0][to.y as usize][to.x as usize] =
        boards[from.z as usize][from.y as usize][from.x as usize];
    boards[from.z as usize][from.y as usize][from.x as usize] = None;
    if let Some(mut piece) = current_piece {
        piece.color = piece.color.opponent();
        match piece.color {
            Color::Black => {
                'outer: for y in 0..BOARD_SIZE {
                    for x in 0..BOARD_SIZE {
                        if boards[1][y][x].is_none() {
                            boards[1][y][x] = Some(piece);
                            break 'outer;
                        }
                    }
                }
            }
            Color::White => {
                'outer: for y in (0..BOARD_SIZE).rev() {
                    for x in (0..BOARD_SIZE).rev() {
                        if boards[1][y][x].is_none() {
                            boards[1][y][x] = Some(piece);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    boards
}

pub fn print_boards(boards: &Boards) {
    let mut board_data = format!("{:->46}\n", "");

    boards.iter().for_each(|board| {
        board.iter().for_each(|row| {
            row.iter().for_each(|p| match p {
                None => board_data.push_str(&format!("|{: >4}", "")),
                Some(piece) => {
                    if piece.color == Color::Black {
                        board_data.push_str(&format!("| \x1b[31m{}\x1b[m ", piece))
                    } else {
                        board_data.push_str(&format!("| {} ", piece))
                    }
                }
            });
            board_data.push_str("|\n");
        });
        board_data.push_str(&format!("{:->46}\n", ""));
    });
    println!("{}", board_data);
}

pub fn select_best_board(boards: &[Boards]) -> Boards {
    let mut rng = rand::thread_rng();
    let len = boards.len();
    let index = rng.gen_range(0..len);
    boards[index].clone()
}

// 二歩判定
pub fn is_nifu(board: &Board, m: LegalMove, color: Color) -> bool {
    for y in 0..BOARD_SIZE {
        if let Some(piece) = board[y][m.to.x as usize] {
            if piece.piece_type == PieceType::Pawn && piece.color == color {
                return true;
            }
        }
    }
    false
}
