# Implementation roadmap

## Phase 0 — Product validation and architecture (1–2 weeks)

- Confirm service type, target country, payroll scope, worker policies, and signature requirements.
- Approve the independent Rust/PostgreSQL architecture decision.
- Select Android/iOS support policy and Kotlin Multiplatform boundaries.
- Select hosting, object storage, map provider, payment provider, and notification provider for the pilot.
- Define integration contracts for ERPNext/Frappe HR and CSV/API exports.
- Produce a clickable worker-flow prototype and pilot test plan.
- Decide location tracking modes and retention.

**Exit criteria:** approved MVP scope, architecture decision record, prioritized backlog, provider decisions, and test organization.

## Phase 1 — Core platform foundation (3–5 weeks)

- Create Rust workspace with API, domain modules, workers, integrations, and shared libraries.
- Provision development and staging environments with PostgreSQL, object storage, queues, logging, and backups.
- Implement organizations, users, roles, customers, employees, sites, service types, and permissions.
- Establish database migrations, CI, linting, formatting, API tests, and license inventory.
- Implement audit events and tenant isolation before feature expansion.

**Exit criteria:** a developer can run the stack and create core records with role restrictions and automated tests.

## Phase 2 — Core field operations (4–6 weeks)

- Implement orders, jobs, assignments, schedules, and status transitions.
- Build dispatcher/admin web screens.
- Build native worker login, assigned jobs, job detail, and online execution.
- Implement checklist templates and versioned checklist runs.
- Add customer/site instructions and job dependencies.

**Exit criteria:** an online order-to-job flow works end to end without ERPNext/Frappe runtime dependencies.

## Phase 3 — Evidence and offline mobile (5–8 weeks)

- Add Kotlin Multiplatform shared sync/data logic.
- Add Android and iOS local storage and action queues.
- Add travel, GPS check-in/out, photos, expenses, signatures, and declarations.
- Implement idempotent synchronization and conflict UX.
- Add PDF generation, secure media access, and content hashes.

**Exit criteria:** a worker can complete a job offline and a supervisor can approve reliable evidence.

## Phase 4 — Integrations and operational finance (3–5 weeks)

- Implement map/routing, notification, and payment provider adapters.
- Implement accounting and payroll export batches with retries and reconciliation.
- Build the ERPNext/Frappe HR connector as an optional integration, not a core dependency.
- Export attendance, deductions, reimbursements, salary inputs, costs, and incentives.
- Implement complaint and warranty workflows.
- Add employee transfers and approval history.

**Exit criteria:** approved job data reaches at least one accounting/payroll target through an auditable export or connector.

## Phase 5 — Pilot and commercial hardening (4–8 weeks)

- Pilot with a small worker group and one service type in one jurisdiction.
- Measure sync failures, completion time, rejection rate, location battery impact, provider errors, and support cases.
- Perform security review, backup restore test, permissions review, and data-retention review.
- Prepare tenant provisioning, billing boundaries, support runbook, terms, privacy documentation, and SLA targets.
- Improve accessibility, localization, and operational reporting.

**Exit criteria:** pilot acceptance, production runbook, support process, commercial readiness decision, and go-live approval.

## Phase 6 — Native accounting and payroll expansion (post-MVP)

- Build a double-entry accounting ledger, chart of accounts, invoices, expenses, payments, taxes, reconciliation, period closing, and financial reports.
- Start native payroll with one country, one currency, one tax system, and one employment model.
- Add salary components, overtime, deductions, benefits, incentives, reimbursements, salary statements, approval, locking, and adjustment records.
- Add further jurisdictions as versioned payroll rule packages.
- Keep provider adapters available for customers who require existing ERP/payroll systems.

**Exit criteria:** specialist-reviewed accounting and payroll modules pass reconciliation, audit, and jurisdiction-specific acceptance tests.

## Suggested delivery team

- Product owner/domain expert.
- Rust/PostgreSQL backend developer.
- Android/Kotlin developer.
- iOS/Swift developer or Kotlin Multiplatform engineer.
- Full-stack/admin web developer.
- QA/automation engineer.
- UX designer shared during discovery and pilot.
- Accounting/payroll specialist before native financial modules.
- Security/legal/privacy reviewers as needed.

## Main risks and mitigations

| Risk | Mitigation |
|---|---|
| Rebuilding too much ERP too early | Keep MVP focused on field service; use exports and adapters |
| Integration inconsistency | Idempotent APIs, immutable export batches, reconciliation, retry queues |
| Offline conflicts or lost evidence | Idempotent action queue, local persistence, explicit sync states |
| Excessive battery/data use from GPS | Configurable tracking modes, sampling policy, pilot measurement |
| Sensitive documents exposed | Separate permissions, private storage, short-lived links, audit access |
| Payroll disputes | Approval, lock periods, adjustment records, immutable audit trail |
| Accounting errors | Append-only ledger design, reversals, reconciliation, specialist review |
| Scope expands beyond MVP | One service type, one region, explicit post-MVP backlog |
| Poor adoption by workers | Field testing, large controls, minimal typing, offline UX |
| Mobile platform divergence | Share domain/sync logic selectively; keep hardware integrations native |

## Definition of done for each feature

- Backend validation and permissions implemented.
- Mobile/web happy path and failure path tested.
- Audit event defined for sensitive changes.
- Offline behavior documented where applicable.
- Automated tests cover core rules.
- User-facing error and recovery path exists.
- Integration retry/reconciliation behavior defined where applicable.
- Documentation and migration/rollback notes updated.
