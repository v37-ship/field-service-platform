# Contributing

## Development workflow

1. Read `AGENTS.md` and the relevant phase task document.
2. Create a focused branch from `main`.
3. Implement only one phase or sub-phase.
4. Add migrations and tests with the implementation.
5. Run the required checks locally.
6. Open a pull request against `main`; do not merge it automatically.
7. The pull request must be reviewed before merge.

## Phase delivery order

```text
Phase 2A: authentication boundary and tenant authorization
Phase 2B: customer and site CRUD APIs
Phase 2C: correlation IDs, API errors, and hardening
Phase 3: orders, jobs, assignments, and dispatch
Phase 4: offline worker mobile app
Phase 5: travel, checklists, evidence, signatures, and completion reports
Phase 6: costs, complaints, warranty, and incentives
Phase 7: accounting/payroll exports and optional ERP integrations
```

## Review expectations

Security-sensitive changes must include tests for unauthorized access, cross-tenant access, deactivated identities, and invalid client-supplied context. Reviewers should verify that authorization is enforced below the HTTP layer and that migrations preserve organization boundaries.
