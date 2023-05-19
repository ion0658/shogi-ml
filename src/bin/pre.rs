use anyhow::Result;
use shogi_alg::board::{create_initial_board, get_num_array};
use std::{env, io::Read};
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let generation = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or_default()
    } else {
        0
    };
    let file_name = format!("model/gen_{}/saved_model.pb", generation);
    // 学習済みのH5ファイルを読み込む
    let mut proto = Vec::new();
    std::fs::File::open(file_name)?.read_to_end(&mut proto)?;

    // TensorFlowのセッションとグラフを作成
    let mut graph = Graph::new();

    // グラフにモデルをインポート
    graph.import_graph_def(&proto, &ImportGraphDefOptions::new())?;

    // グラフ内の入力と出力のノードを特定
    let input_node = graph.operation_by_name_required("board_in")?;
    let output_node = graph.operation_by_name_required("winner_out")?;

    let board = create_initial_board();
    let board = get_num_array(&board);
    let board = board.concat().concat();
    let input_data = board.iter().map(|x| *x as u64).collect::<Vec<_>>();

    // 入力Tensorの作成
    let input_tensor: tensorflow::Tensor<f32> = Tensor::new(&input_data);

    // 推論の実行
    let mut args = SessionRunArgs::new();
    args.add_feed(&input_node, 0, &input_tensor);
    let output_token = args.request_fetch(&output_node, 0);

    let session = create_session(&graph)?;
    session.run(&mut args).expect("Failed to run session");

    // 出力Tensorの取得
    let output_tensor = args
        .fetch::<f32>(output_token)
        .expect("Failed to fetch output tensor");

    // 出力データの利用
    let output_data = output_tensor[0];
    println!("Output: {}", output_data);
    Ok(())
}

fn create_session(graph: &Graph) -> Result<Session> {
    let options = SessionOptions::new();
    Ok(Session::new(&options, &graph)?)
}
