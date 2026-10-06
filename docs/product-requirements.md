# Product requirements

## 1. Product vision

Build a field-service and workforce platform that lets an organization recruit and manage workers, sell or receive service orders, dispatch work, track travel, prove completion, calculate costs and incentives, and manage complaints and warranty obligations.

The platform must be usable by workers with unreliable connectivity and must preserve a trustworthy record of what happened at a job site.

## 2. User roles

- **Worker/technician:** receives jobs, travels, performs checklists, captures evidence, records materials and costs, and obtains signatures.
- **Dispatcher:** creates and assigns jobs, monitors status and travel, and reschedules work.
- **Supervisor:** reviews evidence, approves completion, costs, exceptions, and incentives.
- **HR/payroll administrator:** manages recruitment, employee data, attendance, deductions, salary, incentives, and transfers.
- **Finance administrator:** manages orders, billing, costs, reimbursements, and payments.
- **Customer:** places or confirms orders, receives updates, signs completion, and raises complaints or warranty claims.
- **System administrator:** configures permissions, forms, checklists, locations, policies, and integrations.

## 3. Functional requirements

### Recruitment and workforce

- Candidate records, application stages, interview notes, documents, and hiring decisions.
- Employee profile with identity information, contact details, emergency contact, skills, certifications, documents, and expiry dates.
- Employee onboarding, assignment to teams/regions, transfer history, suspension, and offboarding.

### Working hours and travel

- Shift and work calendar configuration.
- Mobile check-in/check-out with timestamp and location.
- Breaks, overtime, absence, and attendance corrections with approval.
- Job travel start/stop, route points where enabled, distance, duration, transport mode, and travel expenses.
- Explicit consent and policy controls for location tracking.

### Orders and jobs

- Customer order creation through admin web, customer portal, or API.
- Order lines, service location, requested time window, priority, price, tax, and attachments.
- Job generation from an order, assignment to one or more workers, and job dependencies.
- Configurable checklist templates by service type.
- Materials, equipment, notes, hazards, before/after photos, and exceptions.

### Proof of completion

- Required checklist items must be completed or marked as not applicable with a reason.
- Photos must include metadata and be linked to the job step where practical.
- Customer signature, worker signature, signed-at timestamp, and signer identity.
- Generated completion report with order, job, checklist, evidence, costs, signatures, and declaration text.
- Immutable audit history for submitted completion evidence.

### Complaints and warranty

- Complaint intake from customer, worker, manager, or API.
- Categorization, severity, SLA, owner, status, communication history, and resolution.
- Warranty policy, eligibility dates, covered service/product, claim evidence, approval, repair/rework job, and closure.

### Costs, deductions, salary, incentives, and transfers

- Job-level labor, travel, material, subcontractor, and miscellaneous costs.
- Expense receipts and approval workflow.
- Payroll inputs for hours, overtime, approved deductions, advances, reimbursements, and incentives.
- Configurable incentive rules based on completed jobs, quality, speed, revenue, or policy exceptions.
- Employee transfer requests with source, destination, effective date, approvers, and history.

## 4. Non-functional requirements

- Offline-first worker experience with safe retry and conflict handling.
- Role-based access control and tenant/organization isolation.
- Auditability for status changes, signatures, approvals, payroll inputs, and sensitive document access.
- Encryption in transit and at rest where supported by the deployment.
- Scalable media storage; application database should not store large original images directly.
- Localization-ready dates, time zones, currencies, tax rules, and languages.
- Observability: structured logs, metrics, error reporting, job synchronization diagnostics, and audit reports.

## 5. Product principles

1. Evidence should be collected at the point of work, not reconstructed later.
2. Offline actions must be visible to the worker until synchronized.
3. Payroll and financial effects require explicit approval.
4. Sensitive identity and location data should be collected only when necessary.
5. Core business behavior belongs in the backend; mobile clients are untrusted views of server state.
