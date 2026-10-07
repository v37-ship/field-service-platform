use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::domain::audit::{AuditError, AuditEvent};

#[async_trait]
pub trait AuditEventStore: Send + Sync {
    async fn persist(&self, event: &AuditEvent) -> Result<(), AuditError>;
}

#[derive(Clone)]
pub struct PostgresAuditEventStore {
    pool: PgPool,
}

impl PostgresAuditEventStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditEventStore for PostgresAuditEventStore {
    async fn persist(&self, event: &AuditEvent) -> Result<(), AuditError> {
        sqlx::query(
            r#"
            INSERT INTO audit_events (
                id,
                organization_id,
                actor_user_id,
                action,
                target_type,
                target_id,
                metadata,
                occurred_at,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            "#,
        )
        .bind(event.id)
        .bind(event.organization_id)
        .bind(event.actor_user_id)
        .bind(&event.action)
        .bind(&event.target_type)
        .bind(event.target_id)
        .bind(&event.metadata)
        .bind(event.occurred_at)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| AuditError::Persist(err.to_string()))
    }
}

#[derive(Default, Clone)]
pub struct InMemoryAuditEventStore {
    events: Arc<Mutex<Vec<AuditEvent>>>,
}

impl InMemoryAuditEventStore {
    pub async fn events(&self) -> Vec<AuditEvent> {
        self.events.lock().await.clone()
    }
}

#[async_trait]
impl AuditEventStore for InMemoryAuditEventStore {
    async fn persist(&self, event: &AuditEvent) -> Result<(), AuditError> {
        self.events.lock().await.push(event.clone());
        Ok(())
    }
}
