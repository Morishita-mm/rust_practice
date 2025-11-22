use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use std::sync::Arc;
use crate::models::{CreateVote, VoteRecord};
use crate::repositories::VoteRepository;

pub async fn cast_vote(
    State(repo): State<Arc<dyn VoteRepository>>,
    Json(payload): Json<CreateVote>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    repo.create(payload.team_name.clone())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, format!("Voted for {}", payload.team_name)))
}

pub async fn get_votes(
    State(repo): State<Arc<dyn VoteRepository>>,
) -> Result<Json<Vec<VoteRecord>>, (StatusCode, String)> {
    
    let votes = repo.find_all()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(votes))
}