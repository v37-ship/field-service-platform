# Architecture decision: independent Rust/PostgreSQL core

**Status:** Accepted

## Decision

Build the core SaaS product independently with Rust and PostgreSQL. Do not fork ERPNext or Frappe HR and do not make them runtime dependencies of the core platform.

Use ERPNext/Frappe HR and other providers through optional integration adapters. Add native accounting and payroll capabilities incrementally after validating the field-service product.

## Context

The product must support a lightweight, reliable mobile experience with offline work, GPS, camera access, signatures, photos, and background synchronization. The owner also wants a commercial SaaS with control over the technology, database, deployment, branding, and product roadmap.

ERPNext/Frappe HR provide mature accounting and HR functionality, but adopting them as the foundation would introduce a Python/Frappe-centered architecture, upstream upgrade coupling, a different operational model, and additional license/compliance considerations. It would also make the preferred Rust/PostgreSQL architecture a secondary layer.

## Consequences

### Benefits

- Full control over the SaaS product and data model.
- Rust/PostgreSQL consistency across the core backend.
- Mobile and field-service workflows can be designed around offline operation.
- ERPNext/Frappe HR remain useful integrations for existing customers.
- Accounting and payroll scope can grow based on real customer requirements.
- Map and payment providers can be replaced without changing core domain modules.

### Costs

- We must build and maintain core HR, accounting, and payroll capabilities over time.
- Financial and payroll correctness requires specialist review and extensive testing.
- Integrations require mapping, retries, reconciliation, and support.
- The initial product must intentionally limit scope to one market and one payroll jurisdiction.

## Boundaries

The core product owns:

- Tenants, users, roles, customers, sites, orders, jobs, assignments, checklists, travel, evidence, signatures, declarations, costs, incentives, complaints, warranty, attendance events, and audit history.

The core product initially exports or synchronizes:

- Approved invoices/payment records, expenses, attendance, deductions, salary inputs, reimbursements, and incentives.

External providers initially own:

- Card processing and settlement, global map data/routing, email/SMS/push delivery, and jurisdiction-specific payroll filing unless implemented later as native modules.

## Licensing and commercial posture

Use permissively licensed dependencies where practical and maintain a third-party license inventory. Do not copy ERPNext or Frappe source code. Integrations should use documented APIs and customer-authorized credentials. Obtain legal review before distributing software or offering regulated payroll/payment functionality.
