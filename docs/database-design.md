# Database design

## 1. Core entities

| Entity | Purpose |
|---|---|
| Organization | Tenant, policies, timezone, locale, currency |
| User | Authentication and application identity |
| Employee | Worker/HR profile linked to Frappe HR |
| Customer | Customer account and contact information |
| Site | Service location, access notes, coordinates, timezone |
| Service Type | Service definition, default duration, checklist, warranty policy |
| Order | Commercial/customer request and order lines |
| Job | Executable unit of field work linked to order/site |
| Job Assignment | Worker/team assignment and schedule |
| Checklist Template | Versioned service checklist definition |
| Checklist Run | Job-specific checklist instance and answers |
| Travel Session | Travel start/end, mode, distance, duration |
| Location Event | Timestamped location sample or check-in/out event |
| Media Asset | Photo/document metadata and storage reference |
| Signature | Signer, signature evidence, timestamp, hash, consent |
| Declaration | Generated or submitted completion declaration |
| Job Cost | Labor, travel, material, expense, or miscellaneous cost |
| Incentive Rule | Configurable calculation rule and version |
| Incentive Result | Calculated and approved worker incentive |
| Complaint | Customer/service issue and resolution workflow |
| Warranty Claim | Coverage validation and rework/repair tracking |
| Sync Action | Idempotent mobile action receipt and result |
| Audit Event | Append-only record of sensitive changes |

## 2. Important relationships

```text
Customer 1---N Order 1---N Job
Site 1---N Job
Service Type 1---N Job
Job 1---N Job Assignment
Job 1---1 Checklist Run
Job 1---N Travel Session
Job 1---N Location Event
Job 1---N Media Asset
Job 1---N Job Cost
Job 1---N Complaint
Job 1---N Warranty Claim
Employee 1---N Job Assignment
Employee 1---N Incentive Result
```

## 3. State models

### Job

`draft -> assigned -> accepted -> traveling -> on_site -> paused -> submitted -> approved`

Alternative terminal paths: `rejected`, `cancelled`, `failed`, `needs_rework`.

### Complaint

`new -> triaged -> assigned -> investigating -> awaiting_customer -> resolved -> closed`.

### Warranty claim

`submitted -> eligibility_review -> approved/rejected -> rework_scheduled -> completed -> closed`.

## 4. Data rules

- Every business record includes organization, created_by, modified_by, created_at, and modified_at as appropriate.
- Use stable IDs; do not expose sequential sensitive identifiers where avoidable.
- Version checklist templates. Existing checklist runs retain the version used at job execution.
- Store media checksum, MIME type, size, dimensions, capture time, and job linkage.
- Store signature and declaration hashes so exported documents can be verified.
- Soft-delete or archive operational records according to retention policy; never silently remove audit events.
- Separate sensitive identity-document metadata and permissions from general job media.

## 5. Reporting projections

Begin with database reports and indexed query fields. Add denormalized summaries only after measuring performance. Useful indexes include organization/status, assignment/worker/status, job/scheduled date, complaint/SLA status, and sync action/device/status.
