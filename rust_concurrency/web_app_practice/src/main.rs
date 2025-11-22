mod handlers;
mod models;
mod repositories;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{postgres::PgPoolOptions};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;

use crate::repositories::{VoteRepository, VoteRepositoryForDb};

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

    let repo = VoteRepositoryForDb::new(pool);
    let app_state: Arc<dyn VoteRepository> = Arc::new(repo);

    let app = Router::new()
        .route("/vote", post(handlers::cast_vote)) // POST /vote -> cast_vote関数へマッピング
        .route("/votes", get(handlers::get_votes)) // GET /votes -> get_votes関数へマッピング
        .with_state(app_state);

    let _addr = SocketAddr::from(([0, 0, 0, 0], 8080));
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
