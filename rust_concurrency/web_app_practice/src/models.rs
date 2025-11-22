use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateVote {
    pub team_name: String,
}

#[derive(Serialize, FromRow)]
pub struct VoteRecord {
    pub team_name: String,
    pub count: i32,
}