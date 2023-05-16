use std::fmt;

use crate::board::{Board, Position, BOARD_SIZE};

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

// 駒を表す構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
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
            (PieceType::Rook, _) => {
                let mut move_vec = Vec::new();
                for i in 1..BOARD_SIZE as i32 {
                    move_vec.push((0, i));
                    move_vec.push((0, -i));
                    move_vec.push((i, 0));
                    move_vec.push((-i, 0));
                }
                move_vec
            }
            (PieceType::Bishop, _) => {
                let mut move_vec = Vec::new();
                for i in 1..BOARD_SIZE as i32 {
                    move_vec.push((i, i));
                    move_vec.push((i, -i));
                    move_vec.push((-i, i));
                    move_vec.push((-i, -i));
                }
                move_vec
            }
            (
                PieceType::Gold
                | PieceType::PromotedKnight
                | PieceType::PromotedLance
                | PieceType::PromotedSilver
                | PieceType::PromotedPawn,
                Color::Black,
            ) => {
                vec![(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 1), (1, 0)]
            }
            (
                PieceType::Gold
                | PieceType::PromotedKnight
                | PieceType::PromotedLance
                | PieceType::PromotedSilver
                | PieceType::PromotedPawn,
                Color::White,
            ) => {
                vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)]
            }
            (PieceType::Silver, Color::Black) => {
                vec![(-1, -1), (-1, 1), (0, 1), (1, -1), (1, 1)]
            }
            (PieceType::Silver, Color::White) => {
                vec![(-1, -1), (-1, 1), (0, -1), (1, -1), (1, 1)]
            }
            (PieceType::Knight, Color::Black) => vec![(-1, 2), (1, 2)],
            (PieceType::Knight, Color::White) => vec![(-1, -2), (1, -2)],
            (PieceType::Lance, Color::Black) => {
                let mut move_vec = Vec::new();
                for i in 1..BOARD_SIZE as i32 {
                    move_vec.push((0, i));
                }
                move_vec
            }
            (PieceType::Lance, Color::White) => {
                let mut move_vec = Vec::new();
                for i in 1..BOARD_SIZE as i32 {
                    move_vec.push((0, -i));
                }
                move_vec
            }
            (PieceType::Pawn, Color::Black) => vec![(0, 1)],
            (PieceType::Pawn, Color::White) => vec![(0, -1)],
            (PieceType::Dragon, _) => {
                let mut move_vec = Vec::new();
                for i in 1..BOARD_SIZE as i32 {
                    move_vec.push((0, i));
                    move_vec.push((0, -i));
                    move_vec.push((i, 0));
                    move_vec.push((-i, 0));
                }
                move_vec.push((-1, -1));
                move_vec.push((-1, 1));
                move_vec.push((1, -1));
                move_vec.push((1, 1));
                move_vec
            }
            (PieceType::Horse, _) => {
                let mut move_vec = Vec::new();
                for i in 1..BOARD_SIZE as i32 {
                    move_vec.push((i, i));
                    move_vec.push((i, -i));
                    move_vec.push((-i, i));
                    move_vec.push((-i, -i));
                }
                move_vec.push((-1, 0));
                move_vec.push((0, -1));
                move_vec.push((0, 1));
                move_vec.push((1, 0));
                move_vec
            }
        }
    }

    pub fn create_move_range(&self, position: Position, board: &Board) -> Vec<Position> {
        let mut move_range = Vec::new();
        for (x, y) in self.move_vec() {
            let new_position = Position::new(position.x + x, position.y + y);
            if new_position.is_valid() {
                let piece = board[new_position.y as usize][new_position.x as usize];
                if piece.is_none() || piece.unwrap().color != self.color {
                    move_range.push(new_position);
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
