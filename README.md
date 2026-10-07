# Field Service Platform

A private project plan and implementation repository for an independent, mobile-first workforce and field-service SaaS platform.

## Product goal

Connect customer orders to dispatch, worker travel, job execution, proof of completion, costs, incentives, payroll inputs, complaints, and warranty handling.

## Strategic decision

Build the differentiated product in-house using Rust and PostgreSQL. Do not make ERPNext or Frappe HR the platform foundation.

Use external systems through optional adapters during early releases:

- ERPNext/Frappe HR for customers that already use them.
- Accounting and payroll providers for customers that do not want native modules yet.
- Map, routing, payment, email, SMS, and push providers behind replaceable interfaces.

Native accounting and payroll will be added incrementally after the field-service workflow is validated with customers.

## Technology direction

- **Backend:** Rust, Axum, Tokio, SQLx, PostgreSQL.
- **Mobile:** Kotlin Multiplatform for shared logic; Kotlin/Jetpack Compose for Android; Swift/SwiftUI for iOS.
- **Admin web:** Start lightweight and expand only where needed.

## Repository status

### Implemented (Phase 1 backend foundation)

- Rust workspace at repository root.
- API service in `services/api` with `/health` and `/ready`.
- Environment configuration loading and validation.
- SQLx PostgreSQL pool integration and startup migrations.
- Initial tenant-owned schema migrations for organizations, users, roles, customers, sites, employees, service types, and audit events.
- Tenant-boundary and audit abstractions with tests.
- Local Docker Compose stack with API + PostgreSQL.
- CI workflow for format, clippy, tests, and build checks.

### Planned next

- Domain modules for jobs, travel, checklist execution, evidence, costs, incentives, complaints, warranty, exports, and provider integrations.

## Quick start

```bash
cp .env.example .env
cargo run -p field-service-api
```

or:

```bash
docker compose up --build
```

See [`services/api/README.md`](services/api/README.md) for backend setup, migrations, and commands.
