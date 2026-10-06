# Implementation roadmap

## Phase 0 — Discovery and validation (1–2 weeks)

- Confirm service types, countries, payroll rules, worker policies, and signature requirements.
- Select mobile framework and hosting approach.
- Validate ERPNext/Frappe HR versions and extension points.
- Produce clickable worker-flow prototype.
- Decide location tracking modes and retention.

**Exit criteria:** approved MVP scope, architecture decision record, prioritized backlog, and test organization.

## Phase 1 — Platform foundation (2–4 weeks)

- Provision development and staging environments.
- Install/configure ERPNext and Frappe HR.
- Create custom Frappe app and CI checks.
- Configure organizations, roles, customers, employees, sites, service types, and permissions.
- Establish object storage, backups, logging, and error monitoring.

**Exit criteria:** a developer can run the stack and create the core records with role restrictions.

## Phase 2 — Core operations (3–5 weeks)

- Implement orders, jobs, assignments, schedules, and status transitions.
- Build dispatcher web screens.
- Build worker login, assigned jobs, job detail, and basic online execution.
- Implement checklist templates and versioned checklist runs.

**Exit criteria:** an online order-to-job flow works end to end.

## Phase 3 — Evidence and offline mobile (4–6 weeks)

- Add local database/cache and action queue.
- Add travel, GPS check-in/out, photos, expenses, signatures, and declarations.
- Implement idempotent synchronization and conflict UX.
- Add PDF generation and secure media access.

**Exit criteria:** worker can complete a job offline and supervisor can approve reliable evidence.

## Phase 4 — Finance, HR, complaints, and warranty (3–5 weeks)

- Map job costs to ERPNext accounting.
- Export attendance, deductions, salary inputs, and incentives to Frappe HR.
- Implement complaint and warranty workflows.
- Add employee transfers and approval history.

**Exit criteria:** approved job data reaches finance/payroll workflows without manual re-entry for the MVP scenario.

## Phase 5 — Pilot and hardening (3–6 weeks)

- Pilot with a small worker group and one service type.
- Measure sync failures, completion time, rejection rate, location battery impact, and support cases.
- Perform security review, backup restore test, permissions review, and data-retention review.
- Improve accessibility, localization, and operational reporting.

**Exit criteria:** pilot acceptance, production runbook, support process, and go-live decision.

## Suggested delivery team

- Product owner/domain expert.
- Frappe/ERPNext backend developer.
- Mobile developer.
- Full-stack/admin web developer.
- QA/automation engineer.
- UX designer shared during discovery and pilot.
- Security/legal/privacy reviewers as needed.

## Main risks and mitigations

| Risk | Mitigation |
|---|---|
| Upstream ERP changes break customizations | Separate custom app, pin versions, maintain upgrade tests |
| Offline conflicts or lost evidence | Idempotent action queue, local persistence, explicit sync states |
| Excessive battery/data use from GPS | Configurable tracking modes, sampling policy, pilot measurement |
| Sensitive documents exposed | Separate permissions, private storage, short-lived links, audit access |
| Payroll disputes | Approval, lock periods, adjustment records, immutable audit trail |
| Scope expands beyond MVP | One service type, one region, explicit post-MVP backlog |
| Poor adoption by workers | Field testing, large controls, minimal typing, offline UX |

## Definition of done for each feature

- Backend validation and permissions implemented.
- Mobile/web happy path and failure path tested.
- Audit event defined for sensitive changes.
- Offline behavior documented where applicable.
- Automated tests cover core rules.
- User-facing error and recovery path exists.
- Documentation and migration/rollback notes updated.
