# Field Service Platform

A private project plan for a mobile-first workforce and field-service platform built around ERPNext, Frappe HR, and a custom Frappe application.

## Product goal

Connect customer orders to dispatch, worker travel, job execution, proof of completion, costs, incentives, payroll, complaints, and warranty handling.

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
  -> Job completed
  -> Incentive and payroll inputs calculated
  -> Complaint or warranty workflow if required
```

## Repository plan

- `docs/` — product, architecture, data, workflows, security, and delivery plans.
- `apps/mobile-worker/` — future worker mobile application.
- `apps/mobile-manager/` — future dispatch and supervisor application.
- `apps/admin-web/` — future operations and HR administration interface.
- `services/frappe-custom-app/` — future custom Frappe application extending ERPNext/Frappe HR.
- `services/integrations/` — future integrations such as maps, notifications, storage, and payments.

## Recommended technology direction

- ERPNext for orders, customers, accounting, inventory, issues, and warranty processes.
- Frappe HR for recruitment, employees, attendance, payroll, salary, expenses, and transfers.
- A custom Frappe app for jobs, checklists, travel, photos, signatures, declarations, incentives, and domain-specific workflows.
- A mobile app with offline-first synchronization for workers in the field.
- Private object storage for photos, identity documents, signatures, and generated declarations.

## MVP success criteria

A manager can create or receive an order, assign a worker, and see the job completed with a checklist, timestamped GPS events, photos, costs, and customer signature. The system produces a completion document and makes approved costs and incentives available to payroll.

See the full plan in [`docs/`](docs/).
