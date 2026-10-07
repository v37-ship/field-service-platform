# field-service-api

Phase 1 backend foundation for the field-service-platform modular monolith.

## Stack

- Rust + Tokio runtime
- Axum HTTP API
- SQLx with PostgreSQL
- SQL migrations in `migrations/`

## Structure

```text
services/api/
├── src/
│   ├── app.rs                     # API router and health/readiness handlers
│   ├── config.rs                  # environment configuration loading/validation
│   ├── domain/
│   │   ├── tenant.rs              # trusted tenant boundary abstraction
│   │   └── audit.rs               # audit-event abstraction
│   ├── infrastructure/
│   │   ├── database.rs            # pool + migrator wiring
│   │   ├── readiness.rs           # db-backed readiness probe
│   │   └── audit_store.rs         # PostgreSQL audit persistence
│   └── main.rs                    # startup, migration, serve
└── migrations/
```

## Environment variables

| Name | Required | Default | Description |
| --- | --- | --- | --- |
| `DATABASE_URL` | yes | none | PostgreSQL connection string for the API and migrations |
| `API_BIND_ADDR` | no | `0.0.0.0:3000` | Socket address for the API listener |
| `APP_ENV` | no | `development` | Environment label for deployment context |
| `RUST_LOG` | no | `field_service_api=debug,tower_http=info` | Structured logging filter |

## Local development

From repository root:

```bash
cp .env.example .env
cargo run -p field-service-api
```

With Docker Compose:

```bash
docker compose up --build
```

Endpoints:

- `GET /health` → liveness (`200`)
- `GET /ready` → readiness with DB probe (`200` or `503`)

## Migration workflow

Migrations are auto-applied during API startup.

Manual workflow:

```bash
# optional: install SQLx CLI locally
cargo install sqlx-cli --no-default-features --features rustls,postgres

# apply migrations to DATABASE_URL
cargo sqlx migrate run
```

(Use `cargo sqlx migrate add <name>` to add new migration files.)

## Commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```
