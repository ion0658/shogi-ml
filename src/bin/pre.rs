use anyhow::Result;
use shogi_alg::{
    board::{create_initial_board, get_num_array, BOARD_SIZE},
    piece::PieceType,
};
use std::env;
use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let generation = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or_default()
    } else {
        0
    };
    let file_name = format!("model/gen_{}", generation);

    // // グラフにモデルをインポート
    let mut graph = Graph::new();
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, file_name)?;
    let session = &bundle.session;
    let signature = bundle.meta_graph_def().get_signature("serving_default")?;
    // グラフ内の入力と出力のノードを特定
    let input_info = signature.get_input("board_in_input")?;
    let output_info = signature.get_output("winner_out")?;

    let input_node = graph.operation_by_name_required(&input_info.name().name)?;
    let output_node = graph.operation_by_name_required(&output_info.name().name)?;

    let board = create_initial_board();
    let boards = [get_num_array(&board), get_num_array(&board)];

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
    let chunks = output_tensor
        .chunks(2)
        .map(|chunk| [chunk[0], chunk[1]])
        .collect::<Vec<[f32; 2]>>();
    chunks.iter().for_each(|chunk| {
        println!("{:?}", chunk);
    });
    Ok(())
}
