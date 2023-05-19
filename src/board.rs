use crate::piece::{concat_vec, Color, Piece, PieceType};
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
            && self.z == 0
    }
}

// ボード上の駒配置を表す2次元配列
pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];
// 持ち駒を含むボード全体を表す3次元配列
pub type Boards = [Board; PAGE_SIZE];

pub type BoardAsNum = [[u8; BOARD_SIZE]; BOARD_SIZE];
pub type BoardsAsNum = [BoardAsNum; PAGE_SIZE * 2];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegalMove {
    pub from: Position,
    pub to: Position,
    pub revolute: bool,
}

impl LegalMove {
    // 成れるかどうかを判定する関数
    pub fn can_revolte(&self, turn: Color) -> bool {
        if self.from.z == 1 {
            return false;
        }
        match turn {
            Color::Black => {
                if self.from.y >= 6 || self.to.y >= 6 {
                    return true;
                } else {
                    false
                }
            }
            Color::White => {
                if self.from.y <= 2 || self.to.y <= 2 {
                    return true;
                } else {
                    false
                }
            }
        }
    }
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
pub fn move_piece(mut boards: Boards, legal_move: LegalMove) -> Boards {
    let current_piece = boards[0][legal_move.to.y as usize][legal_move.to.x as usize];
    // 駒を移動する
    // 成る場合は成る

    boards[0][legal_move.to.y as usize][legal_move.to.x as usize] = if legal_move.revolute {
        boards[legal_move.from.z as usize][legal_move.from.y as usize][legal_move.from.x as usize]
            .unwrap()
            .revolute()
            .into()
    } else {
        boards[legal_move.from.z as usize][legal_move.from.y as usize][legal_move.from.x as usize]
    };
    // 移動元を空欄にする
    boards[legal_move.from.z as usize][legal_move.from.y as usize][legal_move.from.x as usize] =
        None;
    // 駒を取った場合の処理
    if let Some(mut piece) = current_piece {
        piece.color = piece.color.opponent();
        match piece.color {
            Color::Black => {
                'outer: for y in 0..BOARD_SIZE {
                    for x in 0..BOARD_SIZE {
                        if boards[1][y][x].is_none() {
                            boards[1][y][x] = Some(piece.revolute_back());
                            break 'outer;
                        }
                    }
                }
            }
            Color::White => {
                'outer: for y in (0..BOARD_SIZE).rev() {
                    for x in (0..BOARD_SIZE).rev() {
                        if boards[1][y][x].is_none() {
                            boards[1][y][x] = Some(piece.revolute_back());
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
    let mut board_data = format!(
        "{:->46}\n|{:>19}\x1b[31m先  手\x1b[m{:>19}|\n{:->46}\n",
        "", "", "", ""
    );

    boards.iter().enumerate().for_each(|(z, board)| {
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
        if z == 0 {
            board_data.push_str(&format!("{:->46}\n|{:>19}後  手{:>19}|\n", "", "", "",));
            board_data.push_str(&format!(
                "{:->46}\n|{:>19}持ち駒{:>19}|\n{:->46}\n",
                "", "", "", ""
            ));
        } else {
            board_data.push_str(&format!("{:->46}\n", "",));
        }
    });
    println!("{}", board_data);
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

pub fn get_num_array(boards: &Boards) -> BoardsAsNum {
    let mut b: BoardsAsNum = [[[0; BOARD_SIZE]; BOARD_SIZE]; PAGE_SIZE * 2];
    boards.iter().enumerate().for_each(|(z, board)| {
        board.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, p)| {
                if let Some(piece) = p {
                    match (piece.color, z) {
                        (Color::Black, 0) => b[0][y][x] = piece.get_u8(),
                        (Color::White, 0) => b[1][y][x] = piece.get_u8(),
                        (Color::Black, _) => b[1][y][x] = piece.get_u8(),
                        (Color::White, _) => b[2][y][x] = piece.get_u8(),
                    }
                }
            });
        });
    });
    b
}

pub fn is_checkmate(boards: &Boards, turn: Color) -> bool {
    create_move_range(boards, turn)
        .par_iter()
        .filter_map(|range| {
            let boards = move_piece(*boards, *range);
            let checked = is_checked(&boards[0], turn);
            if checked {
                None
            } else {
                Some(boards)
            }
        })
        .count()
        == 0
}

pub fn is_checked(board: &Board, color: Color) -> bool {
    // 王の位置を検索
    let king_position = find_king_position(board, color);

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
