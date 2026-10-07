# Backend and API plan

## 1. Rust backend modules

- **Identity and tenancy:** organizations, users, roles, sessions, device registration, authorization.
- **Workforce:** candidates, employees, contracts, attendance events, transfers, documents.
- **Orders and jobs:** customers, sites, orders, jobs, assignments, schedules, status transitions.
- **Travel:** travel sessions, GPS events, distance/duration summaries, travel expenses.
- **Execution evidence:** checklist templates/runs, answers, media, notes, signatures, declarations.
- **Costs and incentives:** job costs, approvals, incentive rules, calculated inputs.
- **Complaints and warranty:** cases, SLAs, warranty eligibility, rework jobs, resolutions.
- **Accounting foundation:** invoices, expenses, payment records, export batches, reconciliation references; native double-entry ledger is a later milestone.
- **Payroll foundation:** attendance summaries, overtime, deductions, reimbursements, incentive inputs, payroll export batches; native payroll rules are initially limited to one jurisdiction.
- **Synchronization:** device sessions, action receipts, upload sessions, conflict records.
- **Reporting:** operational, workforce, cost, quality, finance-export, and payroll-export reports.
- **Integrations:** ERPNext/Frappe HR, accounting/payroll providers, maps, payments, and notifications.

## 2. API conventions

- Version APIs, for example `/api/v1/...`.
- Use authenticated requests and server-side authorization.
- Use idempotency keys for all mobile writes and provider webhook processing.
- Return stable error codes, human-readable messages, and remediation hints.
- Use pagination and filtering for lists.
- Use UTC timestamps on the wire and preserve the relevant business timezone on the organization/site.
- Never trust client-supplied role, price, approval, payroll, or completion status.
- Expose integration health, sync cursors, retry status, and reconciliation results to administrators.

## 3. Representative endpoints

```text
POST /api/v1/auth/session
GET  /api/v1/jobs?assigned_to=me&status=active
GET  /api/v1/jobs/{job_id}
POST /api/v1/jobs/{job_id}/actions
POST /api/v1/jobs/{job_id}/travel-sessions
POST /api/v1/jobs/{job_id}/checklist-runs
POST /api/v1/jobs/{job_id}/media/upload-session
POST /api/v1/jobs/{job_id}/signature
POST /api/v1/jobs/{job_id}/submit
POST /api/v1/jobs/{job_id}/approve
POST /api/v1/complaints
POST /api/v1/warranty-claims
POST /api/v1/integrations/{provider}/webhooks
POST /api/v1/exports/accounting
POST /api/v1/exports/payroll
GET  /api/v1/sync/pull?cursor=...
POST /api/v1/sync/actions
```

## 4. Domain events

Publish internal events for:

- `job.assigned`
- `job.status_changed`
- `travel.started`
- `job.checked_in`
- `evidence.submitted`
- `job.submitted_for_approval`
- `job.approved`
- `job.rejected`
- `complaint.created`
- `warranty.claimed`
- `incentive.calculated`
- `attendance.approved`
- `payroll_input.approved`
- `accounting_export.created`
- `payroll_export.created`
- `integration.sync_failed`

Consumers may send notifications, generate PDFs, calculate summaries, build exports, synchronize providers, and update reporting projections. Event handlers must be retry-safe.

## 5. Validation rules

- A job cannot be approved without required checklist completion or documented exception.
- A customer signature is required where the service policy demands it.
- Costs above a configured threshold require supervisor approval.
- Payroll and incentive inputs cannot be edited after payroll lock without an adjustment workflow.
- Posted financial entries cannot be edited or deleted; corrections create reversals or adjustments.
- Export batches are immutable after submission and support reconciliation status.
- Warranty claims must validate the service/product and coverage period.
- Identity documents require access controls and expiry warnings.
