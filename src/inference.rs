use crate::{
    board::{Boards, BOARD_SIZE, PAGE_SIZE},
    piece::{Color, PieceType},
};
use anyhow::Result;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tensorflow::{
    Graph, Operation, SavedModelBundle, Session, SessionOptions, SessionRunArgs, Tensor,
};

pub struct Inference {
    session: Option<Session>,
    input_node: Option<Operation>,
    output_node: Option<Operation>,
}

impl Inference {
    pub fn init() -> Result<Self> {
        let model_path = "model/model";
        let path = std::path::Path::new(model_path);
        let i = if path.exists() {
            println!("load model");
            let (session, input_node, output_node) = Self::init_session(model_path)?;
            Self {
                session: Some(session),
                input_node: Some(input_node),
                output_node: Some(output_node),
            }
        } else {
            println!("load model failed");
            Self {
                session: None,
                input_node: None,
                output_node: None,
            }
        };
        Ok(i)
    }

    pub fn init_session(file_name: &str) -> Result<(Session, Operation, Operation)> {
        let mut graph = Graph::new();
        let bundle =
            SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, file_name)?;
        let signature = bundle.meta_graph_def().get_signature("serving_default")?;
        // グラフ内の入力と出力のノードを特定
        let input_info = signature.get_input("board_in_input")?;
        let output_info = signature.get_output("winner_out")?;

        let input_node = graph.operation_by_name_required(&input_info.name().name)?;
        let output_node = graph.operation_by_name_required(&output_info.name().name)?;

        let session = bundle.session;
        Ok((session, input_node, output_node))
    }

    pub fn select_best_board(&self, boards: &[Boards], turn: Color) -> Result<Boards> {
        if let (Some(session), Some(input_node), Some(output_node)) =
            (&self.session, &self.input_node, &self.output_node)
        {
            let board_list = Self::inference(boards, session, input_node, output_node)?;

            let (index, _) = board_list
                .par_iter()
                .max_by(|(_, a_t), (_, b_t)| {
                    if turn == Color::Black {
                        a_t[0].partial_cmp(&b_t[0]).unwrap()
                    } else {
                        a_t[1].partial_cmp(&b_t[1]).unwrap()
                    }
                })
                .cloned()
                .unwrap();
            Ok(boards[index].clone())
        } else {
            Ok(boards[0].clone())
        }
    }

    // fn select_board_for_play(&self, boards: &[Boards], turn: Color) -> Result<Boards> {
    //     if let (Some(session), Some(input_node), Some(output_node)) =
    //         (&self.session, &self.input_node, &self.output_node)
    //     {
    //         let board_list = Self::inference(boards, session, input_node, output_node)?;

    //         let (index, _) = board_list
    //             .par_iter()
    //             .max_by(|(_, a_t), (_, b_t)| {
    //                 if turn == Color::Black {
    //                     a_t[0].partial_cmp(&b_t[0]).unwrap()
    //                 } else {
    //                     a_t[1].partial_cmp(&b_t[1]).unwrap()
    //                 }
    //             })
    //             .cloned()
    //             .unwrap();
    //         Ok(boards[index].clone())
    //     } else {
    //         Ok(boards[0].clone())
    //     }
    // }

    // fn select_board_for_train(&self, boards: &[Boards], turn: Color) -> Result<Boards> {
    //     if let (Some(session), Some(input_node), Some(output_node)) =
    //         (&self.session, &self.input_node, &self.output_node)
    //     {
    //         let mut board_list = Self::inference(boards, session, input_node, output_node)?;
    //         board_list.sort_by(|(_, a_t), (_, b_t)| {
    //             if turn == Color::Black {
    //                 a_t[0].partial_cmp(&b_t[0]).unwrap()
    //             } else {
    //                 a_t[1].partial_cmp(&b_t[1]).unwrap()
    //             }
    //         });
    //         let len = if board_list.len() > 10 {
    //             10
    //         } else {
    //             board_list.len()
    //         };
    //         let board_list = board_list[0..len].to_vec();
    //         let len = board_list.len();
    //         let index = rng.gen_range(0..len);
    //         Ok(boards[index].clone())
    //     } else {
    //         let len = boards.len();
    //         let index = rng.gen_range(0..len);
    //         Ok(boards[index].clone())
    //     }
    // }

    fn inference(
        boards: &[Boards],
        session: &Session,
        input_node: &Operation,
        output_node: &Operation,
    ) -> Result<Vec<(usize, [f32; 2])>> {
        let boards = boards
            .iter()
            .map(|board| get_num_array(board))
            .collect::<Vec<_>>();
        let data = boards.concat().concat().concat();
        // 入力Tensorの作成
        let input_tensor: tensorflow::Tensor<f32> = Tensor::new(&[
            boards.len() as u64,
            BOARD_SIZE as u64,
            BOARD_SIZE as u64,
            (PieceType::get_max() as usize * PAGE_SIZE * 2) as u64,
        ])
        .with_values(&data)?;

        // 推論の実行
        let mut args = SessionRunArgs::new();
        args.add_feed(&input_node, 0, &input_tensor);
        let output_token = args.request_fetch(&output_node, 0);
        session.run(&mut args)?;

        // 出力Tensorの取得
        let output_tensor = args.fetch::<f32>(output_token)?;
        let result = output_tensor
            .chunks(2)
            .map(|chunk| [chunk[0], chunk[1]])
            .enumerate()
            .collect();
        Ok(result)
    }
}

type BoardAsNum = [[[f32; PieceType::get_max() as usize * PAGE_SIZE * 2]; BOARD_SIZE]; BOARD_SIZE];
fn get_num_array(boards: &Boards) -> BoardAsNum {
    let mut b: BoardAsNum =
        [[[0.0; PieceType::get_max() as usize * PAGE_SIZE * 2]; BOARD_SIZE]; BOARD_SIZE];
    boards.iter().enumerate().for_each(|(z, board)| {
        board.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, p)| {
                if let Some(piece) = p {
                    let z_index = (piece.get_u8() as usize
                        + match piece.color {
                            Color::Black => 0,
                            Color::White => PieceType::get_max() as usize,
                        })
                        * match z {
                            0 => 1,
                            _ => 2,
                        }
                        - 1;
                    //println!("index({:?}):{}", piece, z_index);
                    b[x][y][z_index] = 1.0;
                }
                // if let Some(piece) = p {
                //     match (piece.color, z) {
                //         (Color::Black, 0) => {
                //             b[x][y][0] = piece.get_u8() as f32 / PieceType::get_max() as f32
                //         }
                //         (Color::White, 0) => {
                //             b[x][y][1] = piece.get_u8() as f32 / PieceType::get_max() as f32
                //         }
                //         (Color::Black, _) => {
                //             b[x][y][2] = piece.get_u8() as f32 / PieceType::get_max() as f32
                //         }
                //         (Color::White, _) => {
                //             b[x][y][3] = piece.get_u8() as f32 / PieceType::get_max() as f32
                //         }
                //     }
                // }
            });
        });
    });
    b
}
