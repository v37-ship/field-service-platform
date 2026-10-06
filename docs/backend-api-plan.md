# Backend and API plan

## 1. Custom backend modules

- **Organization and configuration:** service types, regions, teams, policies, checklists, working calendars.
- **Job management:** jobs, assignments, status transitions, priorities, dependencies, exceptions.
- **Travel:** travel sessions, GPS events, distance/duration summaries, travel expenses.
- **Execution evidence:** checklist runs, answers, media, notes, signatures, declarations.
- **Costs and incentives:** job costs, approvals, incentive rules, calculated inputs.
- **Complaints and warranty:** cases, SLAs, warranty eligibility, rework jobs, resolutions.
- **Synchronization:** device sessions, action receipts, upload sessions, conflict records.
- **Reporting:** operational, workforce, cost, quality, and payroll export reports.

## 2. API conventions

- Version APIs, for example `/api/method/field_service_platform.v1...`.
- Use authenticated requests and server-side authorization.
- Use idempotency keys for all mobile writes.
- Return stable error codes, human-readable messages, and remediation hints.
- Use pagination and filtering for lists.
- Use UTC timestamps on the wire and preserve the relevant business timezone on the organization/site.
- Never trust client-supplied role, price, approval, payroll, or completion status.

## 3. Representative endpoints

```text
POST /auth/session
GET  /jobs?assigned_to=me&status=active
GET  /jobs/{job_id}
POST /jobs/{job_id}/actions
POST /jobs/{job_id}/travel-sessions
POST /jobs/{job_id}/checklist-runs
POST /jobs/{job_id}/media/upload-session
POST /jobs/{job_id}/signature
POST /jobs/{job_id}/submit
POST /jobs/{job_id}/approve
POST /complaints
POST /warranty-claims
GET  /sync/pull?cursor=...
POST /sync/actions
```

These are logical contracts; actual Frappe whitelisted methods and DocType REST endpoints should be selected during implementation.

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
- `payroll_input.approved`

Consumers may send notifications, generate PDFs, calculate summaries, and update reporting projections. Event handlers must be retry-safe.

## 5. Validation rules

- A job cannot be approved without required checklist completion or documented exception.
- A customer signature is required where the service policy demands it.
- Costs above a configured threshold require supervisor approval.
- Payroll and incentive inputs cannot be edited after payroll lock without an adjustment workflow.
- Warranty claims must validate the service/product and coverage period.
- Identity documents require access controls and expiry warnings.
