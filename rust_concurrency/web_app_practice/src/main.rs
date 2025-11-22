use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    voting_box: Arc<Mutex<HashMap<String, u32>>>,
}

// ユーザーから送信されてくるJSON：{"team_name": "A"}
#[derive(Deserialize)]
struct CreateVote {
    team_name: String,
}

// 投票用ハンドラ(POST /vote)
async fn cast_vote(
    // 共有状態を注入（Springの@Autowiredに近い感覚）
    State(state): State<AppState>,
    // JSONボディを受け取る（Springの@RequestBody）
    Json(payload): Json<CreateVote>,
) -> (StatusCode, String) {
    let mut box_guard = state.voting_box.lock().unwrap();

    // 集計ロジック
    *box_guard.entry(payload.team_name.clone()).or_insert(0) += 1;

    (StatusCode::OK, format!("Voted for {}", payload.team_name))
}

// 集計確認用ハンドラ（GET /votes）
async fn get_votes(
    State(state): State<AppState>
) -> Json<HashMap<String, u32>> {
    let box_guard = state.voting_box.lock().unwrap();

    // データをコピーしてJSONとして返す
    // (*box_guard)で中身を取り出し、clone()で新しいHashMapを作る
    Json(box_guard.clone())
}

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        voting_box: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/vote", post(cast_vote))    // POST /vote -> cast_vote関数へマッピング
        .route("/votes", get(get_votes))    // GET /votes -> get_votes関数へマッピング
        .with_state(shared_state);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on 0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}