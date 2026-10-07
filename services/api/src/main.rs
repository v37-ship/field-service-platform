use std::sync::Arc;

use anyhow::Context;
use tracing::info;

use field_service_api::app::{build_router, AppState};
use field_service_api::config::AppConfig;
use field_service_api::infrastructure::audit_store::PostgresAuditEventStore;
use field_service_api::infrastructure::database::{connect_pool, run_migrations};
use field_service_api::infrastructure::readiness::DatabaseReadiness;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = AppConfig::from_env().context("failed to load configuration")?;
    let pool = connect_pool(&config.database_url)
        .await
        .context("failed to connect to postgres")?;

    run_migrations(&pool)
        .await
        .context("failed to run database migrations")?;

    let state = AppState::new(
        Arc::new(DatabaseReadiness::new(pool.clone())),
        Arc::new(PostgresAuditEventStore::new(pool)),
    );

    let router = build_router(state);
    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .context("failed to bind API listener")?;

    info!(%config.bind_addr, "field-service-api listening");
    axum::serve(listener, router)
        .await
        .context("api server exited unexpectedly")
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "field_service_api=debug,tower_http=info".into()),
        )
        .with_target(false)
        .compact()
        .init();
}
