use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{actors::VoteObserverHandle, repositories::VoteRepository};

#[derive(Deserialize)]
pub struct CreateVote {
    pub team_name: String,
}

#[derive(Serialize, FromRow)]
pub struct VoteRecord {
    pub team_name: String,
    pub count: i32,
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn VoteRepository>,
    pub observer: VoteObserverHandle,
}
