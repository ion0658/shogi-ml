use anyhow::Result;
use shogi_alg::db::get_connection;

#[tokio::main]
async fn main() -> Result<()> {
    load_game_data(0).await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct GameData {
    winner: i8,
    records: Vec<u8>,
}

async fn load_game_data(generation: i64) -> Result<Vec<GameData>> {
    let pool = get_connection().await?;
    let query = sqlx::query_as::<_, GameData>(
        "SELECT WINNER as winner, RECORDS as records FROM KIFU WHERE GENERATION=?",
    )
    .bind(generation);
    let games = query.fetch_all(&pool).await?;
    for game in games.iter() {
        println!("winner: {}", game.winner);
        println!("records: {:?}", game.records.len());
    }
    Ok(games)
}
