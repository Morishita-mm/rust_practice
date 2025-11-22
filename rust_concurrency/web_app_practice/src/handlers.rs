use std::convert::Infallible;

use crate::models::{CreateVote, VoteRecord, AppState};
use axum::{Json, extract::State, http::StatusCode, response::{Sse, sse::Event}};
use futures::Stream;
use tokio::sync::broadcast;

pub async fn cast_vote(
    State(state): State<AppState>,
    Json(payload): Json<CreateVote>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let new_count = state.repo.create(payload.team_name.clone())
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    state.observer.notify_new_vote(payload.team_name.clone(), new_count).await;
    Ok((StatusCode::OK, format!("Voted for {}", payload.team_name)))
}

pub async fn get_votes(
    State(state): State<AppState>,
) -> Result<Json<Vec<VoteRecord>>, (StatusCode, String)> {
    let votes = state.repo
    .find_all()
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
Ok(Json(votes))
}

pub async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.tx.subscribe();
    
    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(vote_record) => {
                    let json = serde_json::to_string(&vote_record).unwrap();
                    yield Ok(Event::default().event("update").data(json));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;

    use super::*;
    use crate::{actors::VoteObserverActor, repositories::MockVoteRepository};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cast_vote_success() {
        let mut mock_repo = MockVoteRepository::new();

        // 期待する振る舞いを定義
        // 「createメソッドが呼ばれたら、引数はなんでもいいからOk(())を返す」
        mock_repo
            .expect_create()
            .times(1) // 1回だけ呼ばれるはず
            .returning(|_| Ok(1)); // Okを返すようにセット

        let (tx, _rx) = broadcast::channel(10);

        // DI
        // 本物のDB接続の代わりに、モックを渡す
        // ダミーのアクターハンドルを作成
        let (_actor, observer_handle) = VoteObserverActor::new(tx.clone());

        let state = AppState {
            repo: Arc::new(mock_repo),
            observer: observer_handle,
            tx,
        };

        let payload = Json(CreateVote {
            team_name: "Rust".to_string(),
        });

        let result = cast_vote(State(state), payload).await;

        assert!(result.is_ok());
        let (status, body) = result.unwrap();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "Voted for Rust");
    }
}
