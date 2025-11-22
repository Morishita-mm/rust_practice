use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgPoolOptions};
use tokio::signal;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool,
}

// ユーザーから送信されてくるJSON：{"team_name": "A"}
#[derive(Deserialize)]
struct CreateVote {
    team_name: String,
}

#[derive(FromRow, Debug, Serialize)]
struct TeamVote {
    team_name: String,
    count: i32,
}

// 投票用ハンドラ(POST /vote)
async fn cast_vote(
    // 共有状態を注入（Springの@Autowiredに近い感覚）
    State(state): State<AppState>,
    // JSONボディを受け取る（Springの@RequestBody）
    Json(payload): Json<CreateVote>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query!(
        "INSERT INTO votes (team_name, count) VALUES ($1, 1)
        ON CONFLICT (team_name)
        DO UPDATE SET count = votes.count + 1",
        payload.team_name
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok((StatusCode::OK, format!("Voted for {}", payload.team_name))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

// 集計確認用ハンドラ（GET /votes）
async fn get_votes(
    State(state): State<AppState>,
) -> Result<Json<Vec<TeamVote>>, (StatusCode, String)> {
    // DBから全てのデータを取得
    let votes = sqlx::query_as!(TeamVote, "SELECT team_name, count FROM votes")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(votes))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数からDBのURLを取得（compose.yamlで設定したもの）
    let database_url = std::env::var("DATABASE_URL").expect("DATABAES_URL must be set");

    // DB接続プールの作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to Database!");

    let shared_state = AppState { pool };

    let app = Router::new()
        .route("/vote", post(cast_vote)) // POST /vote -> cast_vote関数へマッピング
        .route("/votes", get(get_votes)) // GET /votes -> get_votes関数へマッピング
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on 0.0.0.0:8080");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    println!("👋 Server stopped gracefully.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("🔴 Shutdown signal received. Starting graceful shutdown...");
}