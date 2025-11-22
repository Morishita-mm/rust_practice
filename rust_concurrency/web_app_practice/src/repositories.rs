use crate::models::VoteRecord;
use axum::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait VoteRepository: Send + Sync + 'static {
    async fn create(&self, team_name: String) -> anyhow::Result<()>;
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
    async fn create(&self, team_name: String) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO votes (team_name, count) VALUES ($1, 1)
            ON CONFLICT (team_name)
            DO UPDATE SET count = votes.count + 1",
            team_name
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_all(&self) -> anyhow::Result<Vec<VoteRecord>> {
        let votes = sqlx::query_as!(VoteRecord, "SELECT team_name, count FROM votes")
            .fetch_all(&self.pool)
            .await?;
        Ok(votes)
    }
}
