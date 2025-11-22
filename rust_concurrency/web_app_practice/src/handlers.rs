use crate::models::{CreateVote, VoteRecord, AppState};
use axum::{Json, extract::State, http::StatusCode};

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

#[cfg(test)]
mod tests {
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

        // DI
        // 本物のDB接続の代わりに、モックを渡す
        // ダミーのアクターハンドルを作成
        let (_actor, observer_handle) = VoteObserverActor::new();

        let state = AppState {
            repo: Arc::new(mock_repo),
            observer: observer_handle,
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
