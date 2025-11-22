use crate::models::VoteRecord;
use axum::async_trait;
use sqlx::PgPool;

#[cfg_attr(test, mockall::automock)]    // テストの際に自動的にモックを生成してくれる
#[async_trait]
pub trait VoteRepository: Send + Sync + 'static {
    async fn create(&self, team_name: String) -> anyhow::Result<i32>;
    async fn find_all(&self) -> anyhow::Result<Vec<VoteRecord>>;
}

#[derive(Clone)]
pub struct VoteRepositoryForDb {
    pool: PgPool,
}

impl VoteRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VoteRepository for VoteRepositoryForDb {
    async fn create(&self, team_name: String) -> anyhow::Result<i32> {
        let new_count = sqlx::query_scalar!(
            "INSERT INTO votes (team_name, count) VALUES ($1, 1)
            ON CONFLICT (team_name)
            DO UPDATE SET count = votes.count + 1
            RETURNING count",
            team_name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(new_count)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<VoteRecord>> {
        let votes = sqlx::query_as!(VoteRecord, "SELECT team_name, count FROM votes")
            .fetch_all(&self.pool)
            .await?;
        Ok(votes)
    }
}