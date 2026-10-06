# Workflows

## 1. Order to completion

1. Customer order is created and validated.
2. Order is converted to one or more jobs based on service type and site.
3. Dispatcher selects a worker/team and time window.
4. Worker accepts or declines with a reason.
5. Worker starts travel; permitted location events are recorded.
6. Worker checks in at the site.
7. Worker completes required checklist steps and captures evidence.
8. Worker records materials, time, travel, and expenses.
9. Customer reviews the result and signs, or an exception is recorded.
10. Worker submits the job.
11. Supervisor approves or rejects; rejection creates a rework path.
12. System generates the completion declaration/report.
13. Approved costs and incentives become payroll/finance inputs.

## 2. Checklist exception

A required checklist item may be marked `not_applicable`, `blocked`, or `failed` only with a reason. `failed` or `blocked` can automatically require supervisor review or create a complaint/rework task.

## 3. Complaint workflow

- Intake from customer, worker, manager, or API.
- Link to customer, order, job, worker, service type, and evidence.
- Assign severity and SLA.
- Notify owner and customer according to communication policy.
- Investigate and attach findings.
- Choose resolution: explanation, refund/credit, rework, replacement, warranty claim, or rejection.
- Capture approval and close with customer communication.

## 4. Warranty workflow

- Customer submits claim with order/job/product reference and evidence.
- System validates dates, service type, coverage, exclusions, and previous claims.
- Authorized reviewer approves or rejects with reason.
- Approved claim creates a rework/repair job.
- Completion evidence is captured as normal.
- Claim is closed after resolution and any financial adjustment is approved.

## 5. Payroll and incentive workflow

1. Attendance and job time are synchronized to Frappe HR.
2. Job costs and incentive results are calculated.
3. Supervisor approves exceptions and incentive results.
4. Payroll administrator reviews deductions, reimbursements, and salary inputs.
5. Payroll lock prevents ordinary edits.
6. Corrections use an adjustment record linked to the original input.

## 6. Employee transfer workflow

1. Employee or manager submits transfer request.
2. Current and destination managers review.
3. HR validates effective date, role, compensation, region, and access.
4. Approved transfer updates assignments and permissions on the effective date.
5. Historical records retain the prior organization/team/location.

## 7. Stamped declaration

The declaration should be generated from a versioned template containing organization details, job/order identifiers, service result, checklist summary, evidence references, costs where appropriate, signatures, timestamp, and an organization stamp image or cryptographic verification marker. The generated PDF is stored with a content hash and audit event.
