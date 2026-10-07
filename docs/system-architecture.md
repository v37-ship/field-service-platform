# System architecture

## 1. Logical architecture

```text
+-------------------------+       HTTPS/API       +--------------------------+
| Worker mobile app       | <-------------------> | Rust API                 |
| Android Kotlin/Compose  |                       | Axum + Tokio             |
| iOS Swift/SwiftUI       |                       | Domain modules            |
+-------------------------+                       +------------+-------------+
                                                               |
                           +-----------------------------------+-----------------------------------+
                           |                                   |                                   |
                    PostgreSQL                         Background workers                 Object storage
              operational and ledger data             sync, PDFs, notifications          photos/documents
                           |                                   |                                   |
                           +-----------------------+-----------+-------------------+---------------+
                                                   |                               |
                                           Provider adapters                 Admin web application
                                    ERPNext/HR, maps, payments,          operations, finance, HR
                                    accounting, payroll, messaging
```

## 2. Platform strategy

The core is a modular Rust application backed by PostgreSQL. Start as a modular monolith with clear domain boundaries. Split services only when scale, security, or operational isolation justifies it.

Recommended components:

- `services/api`: authentication, HTTP API, domain commands, queries, authorization.
- `services/workers`: synchronization, document generation, notifications, exports, reconciliation.
- `services/integrations`: provider interfaces and adapters.
- `apps/mobile-worker`: offline-first native worker app.
- `apps/admin-web`: dispatcher, supervisor, finance, HR, and configuration screens.

## 3. Core domain ownership

The platform owns tenants, users, customers, sites, orders, jobs, assignments, schedules, travel events, checklists, media metadata, signatures, declarations, job costs, incentives, complaints, warranty claims, attendance events, and audit history.

The platform initially creates exportable or synchronizable accounting/payroll inputs rather than attempting to implement every jurisdiction's accounting and payroll rules in the MVP.

## 4. Integration strategy

All external capabilities must use interfaces and adapters:

```text
MapProvider       -> routing, geocoding, distance, map tiles
PaymentProvider   -> payment intent, refund, webhook, payout reference
AccountingAdapter -> invoices, expenses, payments, journal/export records
PayrollAdapter    -> attendance, deductions, reimbursements, incentives, salary inputs
NotificationProvider -> email, SMS, push
```

Initial adapters may include ERPNext/Frappe HR, CSV/SFTP export, and one selected map/payment provider. Provider credentials are tenant-scoped and never embedded in domain logic.

## 5. Deployment environments

- **Local development:** Docker Compose, seeded PostgreSQL, local object storage, fake providers.
- **Test/staging:** isolated database and storage, synthetic data, sandbox provider accounts.
- **Production:** private network, managed PostgreSQL backups, TLS, monitoring, restricted administration, separate storage and credentials.

## 6. Synchronization model

- Each offline action has a client-generated UUID, device ID, user ID, local timestamp, entity ID, and operation type.
- Server endpoints are idempotent using the action UUID.
- Server returns accepted, rejected, or needs-review results for every queued action.
- Entity versions or server timestamps detect conflicting edits.
- Evidence uploads use resumable or retryable uploads where possible.

## 7. Key architectural decisions

- Rust/PostgreSQL is the core platform; ERPNext/Frappe HR are optional integrations.
- Native mobile apps are preferred for reliable camera, location, background work, storage, and battery behavior.
- Financial records use append-only posting and reversal/adjustment workflows.
- Payroll starts with one jurisdiction and explicit approval/lock periods.
- Generate completion PDFs asynchronously and store their content hash.
- Do not expose unrestricted object-storage URLs for identity documents.
- Keep location tracking configurable per organization and job policy.
