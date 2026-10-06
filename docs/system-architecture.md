# System architecture

## 1. Logical architecture

```text
+----------------------+       HTTPS/API       +----------------------+
| Worker mobile app    | <-------------------> | Frappe API layer     |
| Manager mobile app   |                       | Custom Frappe app    |
+----------------------+                       +----------+-----------+
                                                          |
                                  +-----------------------+-----------------------+
                                  |                       |                       |
                           ERPNext modules         Frappe HR              Background jobs
                           orders/accounting       recruitment/payroll      notifications/reports
                                  |                       |                       |
                                  +-----------------------+-----------------------+
                                                          |
                           +------------------+-----------+------------------+
                           |                  |                              |
                    Database              Object storage                 Integrations
                 business records       photos/documents/PDFs        maps, email, SMS, push
```

## 2. Platform strategy

Use ERPNext and Frappe HR as upstream applications. Implement domain-specific behavior in a separate custom Frappe app rather than editing upstream source. This preserves upgradeability and allows custom DocTypes, server-side validation, workflows, reports, permissions, and API endpoints.

Recommended custom app name: `field_service_platform`.

## 3. Deployment environments

- **Local development:** Docker or bench, seeded demo data, local object storage.
- **Test/staging:** isolated database, object storage bucket, test notification providers, synthetic data.
- **Production:** private network, managed backups, TLS, monitoring, restricted administrator access, separate storage and credentials.

## 4. Service boundaries

- Frappe/ERPNext remains the system of record for customers, orders, accounting, and standard HR/payroll records.
- Custom app owns field jobs, assignments, checklists, travel events, completion evidence, declarations, incentive calculations, and domain workflows.
- Mobile apps own local cache and an outbound action queue, not authoritative business state.
- Object storage owns large binary files; the database stores metadata, hashes, permissions, and references.

## 5. Synchronization model

- Each offline action has a client-generated UUID, device ID, user ID, local timestamp, entity ID, and operation type.
- Server endpoints are idempotent using the action UUID.
- Server returns accepted, rejected, or needs-review results for every queued action.
- Entity versions or server timestamps detect conflicting edits.
- Evidence uploads use resumable or retryable uploads where possible.

## 6. Notifications

Use server events/background jobs for push notifications, email, and SMS. Notifications must be advisory; the server remains authoritative for job status and approvals.

## 7. Key architectural decisions

- Prefer modular monolith first; split services only when scale or isolation requires it.
- Use server-side permission checks for every API operation.
- Generate completion PDFs asynchronously and store their content hash.
- Do not expose unrestricted object-storage URLs for identity documents.
- Keep location tracking configurable per organization and job policy.
