use crate::piece::{Color, Piece, PieceType};

pub const BOARD_SIZE: usize = 9;

// ボード上の座標を表す構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn is_valid(&self) -> bool {
        self.x >= 0 && self.x < BOARD_SIZE as i32 && self.y >= 0 && self.y < BOARD_SIZE as i32
    }
}

// ボード上の駒配置を表す2次元配列
pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

// ボード上での駒の移動範囲を表す2次元配列
pub type MoveRange = Vec<Vec<Option<Vec<Position>>>>; //[[Option<Vec<Position>>; BOARD_SIZE]; BOARD_SIZE];

pub fn create_initial_board() -> Board {
    let board: Board = [[None; BOARD_SIZE]; BOARD_SIZE];
    let board = create_initial_board_black(board);
    let board = create_initial_board_white(board);
    board
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
pub fn create_move_range(board: &Board, turn: Color) -> MoveRange {
    let mut move_range: MoveRange = vec![vec![None; BOARD_SIZE]; BOARD_SIZE];
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if let Some(piece) = board[y][x] {
                if piece.color != turn {
                    continue;
                }
                move_range[y][x] =
                    Some(piece.create_move_range(Position::new(x as i32, y as i32), board));
            }
        }
    }
    move_range
}
