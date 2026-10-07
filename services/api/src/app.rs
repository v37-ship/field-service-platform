use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use crate::infrastructure::audit_store::AuditEventStore;
use crate::infrastructure::readiness::ReadinessProbe;

#[derive(Clone)]
pub struct AppState {
    readiness: Arc<dyn ReadinessProbe>,
    _audit_store: Arc<dyn AuditEventStore>,
}

impl AppState {
    pub fn new(readiness: Arc<dyn ReadinessProbe>, audit_store: Arc<dyn AuditEventStore>) -> Self {
        Self {
            readiness,
            _audit_store: audit_store,
        }
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ReadinessResponse<'a> {
    status: &'a str,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse { status: "ok" }))
}

async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    match state.readiness.check().await {
        Ok(()) => (StatusCode::OK, Json(ReadinessResponse { status: "ready" })).into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadinessResponse {
                status: "not_ready",
            }),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    use crate::infrastructure::audit_store::InMemoryAuditEventStore;
    use crate::infrastructure::readiness::ReadinessProbe;

    use super::{build_router, AppState};

    struct StaticReadinessProbe {
        ready: bool,
    }

    #[async_trait]
    impl ReadinessProbe for StaticReadinessProbe {
        async fn check(&self) -> Result<(), String> {
            if self.ready {
                Ok(())
            } else {
                Err("db unavailable".to_string())
            }
        }
    }

    fn app(ready: bool) -> axum::Router {
        build_router(AppState::new(
            Arc::new(StaticReadinessProbe { ready }),
            Arc::new(InMemoryAuditEventStore::default()),
        ))
    }

    #[tokio::test]
    async fn health_endpoint_is_ok() {
        let response = app(true)
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn ready_endpoint_returns_service_unavailable_when_probe_fails() {
        let response = app(false)
            .oneshot(
                Request::builder()
                    .uri("/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn ready_endpoint_is_ok_when_probe_passes() {
        let response = app(true)
            .oneshot(
                Request::builder()
                    .uri("/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
