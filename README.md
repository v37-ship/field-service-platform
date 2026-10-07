# Field Service Platform

A private project plan for an independent, mobile-first workforce and field-service SaaS platform.

## Product goal

Connect customer orders to dispatch, worker travel, job execution, proof of completion, costs, incentives, payroll inputs, complaints, and warranty handling.

## Strategic decision

Build the differentiated product in-house using Rust and PostgreSQL. Do not make ERPNext or Frappe HR the platform foundation.

Use external systems through optional adapters during early releases:

- ERPNext/Frappe HR for customers that already use them.
- Accounting and payroll providers for customers that do not want native modules yet.
- Map, routing, payment, email, SMS, and push providers behind replaceable interfaces.

Native accounting and payroll will be added incrementally after the field-service workflow is validated with customers.

## Core workflow

```text
Customer order
  -> Job created
  -> Worker assigned
  -> Worker travels to site
  -> GPS check-in
  -> Checklist and work execution
  -> Photos, materials, and costs recorded
  -> Customer signature and stamped declaration
  -> Job completed and approved
  -> Incentive/payroll inputs calculated
  -> Accounting/payroll export or integration
  -> Complaint or warranty workflow if required
```

## Technology direction

- **Backend:** Rust, Axum, Tokio, SQLx, PostgreSQL.
- **Mobile:** Kotlin Multiplatform for shared logic; Kotlin/Jetpack Compose for Android; Swift/SwiftUI for iOS.
- **Admin web:** Start with a lightweight web application; use a full SPA only where it adds clear value.
- **Jobs and workers:** Native Rust/PostgreSQL product modules.
- **Files:** S3-compatible private object storage.
- **Queues/cache:** Redis or Valkey where needed.
- **External capabilities:** provider interfaces for maps, payments, notifications, and accounting/payroll integrations.

## Repository plan

- `docs/` — product, architecture, data, workflows, security, and delivery plans.
- `apps/mobile-worker/` — future native worker mobile application.
- `apps/mobile-manager/` — future manager mobile application if responsive web is insufficient.
- `apps/admin-web/` — operations, finance, HR, and administration interface.
- `services/api/` — future Rust API and application modules.
- `services/workers/` — future Rust background jobs and synchronization workers.
- `services/integrations/` — provider adapters, including ERPNext/Frappe HR, accounting, payroll, maps, payments, and notifications.
- `infrastructure/` — Docker, deployment, observability, and environment configuration.

## MVP success criteria

A manager can create or receive an order, assign a worker, and see the job completed with a checklist, timestamped GPS events, photos, costs, and customer signature. The system produces a completion document and makes approved attendance, cost, incentive, and payroll inputs available through export or an integration.

See the full plan in [`docs/`](docs/).
