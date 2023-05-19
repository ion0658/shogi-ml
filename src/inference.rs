use crate::{
    board::{get_num_array, is_checkmate, Boards, BOARD_SIZE},
    piece::{Color, PieceType},
};
use anyhow::Result;
use rand::prelude::*;
use rayon::prelude::*;
use tensorflow::{
    Graph, Operation, SavedModelBundle, Session, SessionOptions, SessionRunArgs, Tensor,
};

pub struct Inference {
    session: Option<Session>,
    input_node: Option<Operation>,
    output_node: Option<Operation>,
}

impl Inference {
    pub fn init(generation: i32, color: Color) -> Result<Self> {
        let i = if generation > 0 {
            let color_str = match color {
                Color::Black => "b",
                Color::White => "w",
            };
            let file_name = format!("model/gen_{}{}", generation - 1, color_str);
            let (session, input_node, output_node) = Self::init_session(file_name)?;
            Self {
                session: Some(session),
                input_node: Some(input_node),
                output_node: Some(output_node),
            }
        } else {
            Self {
                session: None,
                input_node: None,
                output_node: None,
            }
        };
        Ok(i)
    }

    pub fn init_session(file_name: String) -> Result<(Session, Operation, Operation)> {
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
        let checkmate_boards = boards
            .par_iter()
            .filter(|boards| is_checkmate(boards, turn.opponent()))
            .collect::<Vec<_>>();

        if let Some(&boards) = checkmate_boards.first() {
            return Ok(boards.clone());
        }
        if let (Some(session), Some(input_node), Some(output_node)) =
            (&self.session, &self.input_node, &self.output_node)
        {
            let index = Self::inference(turn, boards, session, input_node, output_node)?;

            Ok(boards[index].clone())
        } else {
            let mut rng = rand::thread_rng();
            let len = boards.len();
            let index = rng.gen_range(0..len);
            Ok(boards[index].clone())
        }
    }

    fn inference(
        turn: Color,
        boards: &[Boards],
        session: &Session,
        input_node: &Operation,
        output_node: &Operation,
    ) -> Result<usize> {
        let boards = boards
            .iter()
            .map(|board| get_num_array(board))
            .collect::<Vec<_>>();
        // 入力Tensorの作成
        let mut input_tensor: tensorflow::Tensor<f32> =
            Tensor::new(&[boards.len() as u64, 4, BOARD_SIZE as u64, BOARD_SIZE as u64]);

        boards.iter().enumerate().for_each(|(i, &hands)| {
            hands.iter().enumerate().for_each(|(z, &board)| {
                board.iter().enumerate().for_each(|(y, &row)| {
                    row.iter().enumerate().for_each(|(x, &piece)| {
                        let index = x
                            + y * BOARD_SIZE
                            + z * BOARD_SIZE * BOARD_SIZE
                            + i * BOARD_SIZE * BOARD_SIZE * 4;
                        input_tensor[index] = piece as f32 / PieceType::get_max() as f32;
                    });
                });
            });
        });

        // 推論の実行
        let mut args = SessionRunArgs::new();
        args.add_feed(&input_node, 0, &input_tensor);
        let output_token = args.request_fetch(&output_node, 0);
        session.run(&mut args)?;

        // 出力Tensorの取得
        let output_tensor = args.fetch::<f32>(output_token)?;
        let mut chunks = output_tensor.chunks(2).collect::<Vec<_>>();
        chunks.sort_by(|a, b| a[turn as usize].partial_cmp(&b[turn as usize]).unwrap());
        //println!("turn: {:?}\n{:?}", turn, chunks);
        let (max_winrate_index, _) = chunks
            .par_iter()
            .map(|chunk| [chunk[0], chunk[1]])
            .enumerate()
            .max_by(|(_, a), (_, b)| a[turn as usize].partial_cmp(&b[turn as usize]).unwrap())
            .unwrap();

        Ok(max_winrate_index)
    }
}
