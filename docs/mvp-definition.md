# MVP definition

## MVP objective

Prove the complete operational loop for one service type and one organization:

```text
Order -> assignment -> travel -> checklist -> evidence -> signature -> approval -> completion report
```

## Included in MVP

- Organization, users, roles, customers, sites, workers, and service types.
- Customer order creation by admin web and API.
- Job creation and assignment by dispatcher.
- Worker login and assigned-job list.
- Job status transitions: assigned, accepted, traveling, on-site, paused, submitted, approved, rejected, cancelled.
- GPS check-in/out and travel start/stop.
- Offline checklist completion with required fields.
- Before/after photos and notes.
- Customer signature and worker confirmation.
- Generated completion PDF.
- Job costs and expense receipt upload.
- Supervisor approval or rejection with reason.
- Basic complaint creation linked to an order/job.
- Basic employee attendance and payroll input export to Frappe HR.
- Audit log and operational dashboard.

## Deferred after MVP

- Continuous background location tracking.
- Customer self-service portal and online payments.
- Advanced route optimization.
- Complex commission plans and automated payroll calculation.
- Multi-country tax/payroll rules.
- Biometric identity verification.
- Automated OCR for identity documents.
- Full warranty contract engine.
- Marketplace or multi-vendor functionality.
- Native app-store release automation.

## MVP acceptance scenarios

1. Dispatcher creates an order and assigns it to a worker.
2. Worker opens the job while online, then completes the job with the device offline.
3. Worker records travel, check-in, checklist answers, photos, expenses, and customer signature.
4. Synchronization resumes and the server accepts the queued actions exactly once.
5. Supervisor reviews evidence and approves completion.
6. System generates a completion report and exposes approved cost/payroll inputs.
7. Customer or manager creates a complaint linked to the completed job.
8. Every sensitive action is visible in the audit history.
