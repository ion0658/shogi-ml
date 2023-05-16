use crate::board::{Board, Position, BOARD_SIZE};
use std::fmt;

#[allow(unused)]
// 駒の種類を表す列挙型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
    Dragon,
    Horse,
    PromotedSilver,
    PromotedKnight,
    PromotedLance,
    PromotedPawn,
}

// プレイヤーを表す列挙型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

// 駒を表す構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.piece_type {
            PieceType::King => write!(f, "玉"),
            PieceType::Rook => write!(f, "飛"),
            PieceType::Bishop => write!(f, "角"),
            PieceType::Gold => write!(f, "金"),
            PieceType::Silver => write!(f, "銀"),
            PieceType::Knight => write!(f, "桂"),
            PieceType::Lance => write!(f, "香"),
            PieceType::Pawn => write!(f, "歩"),
            PieceType::Dragon => write!(f, "龍"),
            PieceType::Horse => write!(f, "馬"),
            PieceType::PromotedSilver => write!(f, "全"),
            PieceType::PromotedKnight => write!(f, "圭"),
            PieceType::PromotedLance => write!(f, "杏"),
            PieceType::PromotedPawn => write!(f, "と"),
        }
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    fn move_vec(&self) -> Vec<(i32, i32)> {
        match (self.piece_type, self.color) {
            (PieceType::King, _) => vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            (PieceType::Rook, _) => vec![
                (-(BOARD_SIZE as i32), 0),
                (0, -(BOARD_SIZE as i32)),
                (0, BOARD_SIZE as i32),
                (BOARD_SIZE as i32, 0),
            ],
            (PieceType::Bishop, _) => vec![
                (-(BOARD_SIZE as i32), -(BOARD_SIZE as i32)),
                (-(BOARD_SIZE as i32), BOARD_SIZE as i32),
                (BOARD_SIZE as i32, -(BOARD_SIZE as i32)),
                (BOARD_SIZE as i32, BOARD_SIZE as i32),
            ],
            (
                PieceType::Gold
                | PieceType::PromotedKnight
                | PieceType::PromotedLance
                | PieceType::PromotedSilver
                | PieceType::PromotedPawn,
                Color::Black,
            ) => vec![(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)],
            (
                PieceType::Gold
                | PieceType::PromotedKnight
                | PieceType::PromotedLance
                | PieceType::PromotedSilver
                | PieceType::PromotedPawn,
                Color::White,
            ) => vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)],
            (PieceType::Silver, Color::Black) => vec![(-1, -1), (-1, 1), (0, 1), (1, -1), (1, 1)],
            (PieceType::Silver, Color::White) => vec![(-1, -1), (-1, 1), (0, -1), (1, -1), (1, 1)],
            (PieceType::Knight, Color::Black) => vec![(-1, 2), (1, 2)],
            (PieceType::Knight, Color::White) => vec![(-1, -2), (1, -2)],
            (PieceType::Lance, Color::Black) => vec![(0, BOARD_SIZE as i32)],
            (PieceType::Lance, Color::White) => vec![(0, -(BOARD_SIZE as i32))],
            (PieceType::Pawn, Color::Black) => vec![(0, 1)],
            (PieceType::Pawn, Color::White) => vec![(0, -1)],
            (PieceType::Dragon, _) => vec![
                (-(BOARD_SIZE as i32), 0),
                (-1, -1),
                (-1, 1),
                (0, -(BOARD_SIZE as i32)),
                (0, BOARD_SIZE as i32),
                (1, -1),
                (1, 1),
                (BOARD_SIZE as i32, 0),
            ],
            (PieceType::Horse, _) => vec![
                (-(BOARD_SIZE as i32), -(BOARD_SIZE as i32)),
                (-(BOARD_SIZE as i32), BOARD_SIZE as i32),
                (-1, 0),
                (0, -1),
                (0, 1),
                (1, 0),
                (BOARD_SIZE as i32, -(BOARD_SIZE as i32)),
                (BOARD_SIZE as i32, BOARD_SIZE as i32),
            ],
        }
    }

    pub fn create_move_range(&self, position: Position, board: &Board) -> Vec<Position> {
        let mut move_range = Vec::new();
        for (vx, vy) in self.move_vec() {
            if self.piece_type == PieceType::Knight {
                let new_position = Position::new(position.x + vx, position.y + vy);
                if new_position.is_valid() {
                    let piece = board[new_position.y as usize][new_position.x as usize];
                    // 空きマスなら動ける
                    if piece.is_none() || piece.unwrap().color != self.color {
                        move_range.push(new_position);
                    }
                }
                continue;
            }
            let dx = calc_delta(vx);
            let dy = calc_delta(vy);
            //println!("d: [{}, {}], v: [{}, {}]", dx, dy, vx, vy);
            let mut x = 0;
            let mut y = 0;
            loop {
                x += dx;
                y += dy;
                //println!("p: [{},{}], d: [{}, {}], v: [{}, {}]", x, y, dx, dy, vx, vy);
                let new_position = Position::new(position.x + x, position.y + y);
                if new_position.is_valid() {
                    let piece = board[new_position.y as usize][new_position.x as usize];
                    // 空きマスなら動ける
                    if piece.is_none() {
                        move_range.push(new_position);
                    } else if piece.unwrap().color != self.color {
                        move_range.push(new_position);
                        break;
                    } else {
                        break;
                    }
                }
                if (dx != 0 && x == vx) || (dy != 0 && y == vy) {
                    break;
                }
            }
        }
        println!(
            "piece: {}, position: [{}, {}], move_range: {:?}",
            self, position.x, position.y, move_range
        );
        move_range
    }
}

fn calc_delta(v: i32) -> i32 {
    if v > 0 {
        1
    } else if v < 0 {
        -1
    } else {
        0
    }
}
