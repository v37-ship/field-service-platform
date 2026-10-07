use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ReadinessProbe: Send + Sync {
    async fn check(&self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct DatabaseReadiness {
    pool: PgPool,
}

impl DatabaseReadiness {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadinessProbe for DatabaseReadiness {
    async fn check(&self) -> Result<(), String> {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}
