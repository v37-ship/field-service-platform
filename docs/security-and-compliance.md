# Security and compliance

This document is an engineering baseline, not legal advice. The organization must confirm local employment, privacy, identity-document, electronic-signature, tax, and data-retention obligations before production use.

## 1. Access control

- Role-based permissions for worker, dispatcher, supervisor, HR, finance, customer, and administrator.
- Restrict records by organization, region, team, assignment, and case ownership where required.
- Separate permissions for viewing identity documents, location history, payroll, and financial data.
- Enforce authorization server-side for every read and write.
- Review administrator access regularly and record privileged actions.

## 2. Identity and documents

- Collect only required identification photos/documents.
- Record document type, issuing country, expiry, verification status, and access policy.
- Encrypt storage and use short-lived authorized download links.
- Avoid embedding identity documents in general notifications or logs.
- Define retention and deletion rules before collecting production data.

## 3. Location privacy

- Document the business purpose and legal basis for location collection.
- Make tracking modes explicit: check-in/out only, travel session, or continuous tracking.
- Request platform permissions with clear user-facing explanations.
- Provide visibility into tracking status and organizational policy.
- Limit retention and access to route history.

## 4. Signatures and proof

- Capture signer name, role/relationship, consent text, timestamp, job ID, device context where appropriate, and evidence hash.
- Preserve the submitted version of the checklist and declaration.
- Do not treat an image of a signature alone as the entire audit record.
- Make signature policy configurable and have it reviewed for the target jurisdiction.

## 5. Application security

- TLS for all network traffic.
- Secure secrets management; never commit credentials.
- Password/session policy delegated to the platform and strengthened with MFA where possible.
- Rate-limit authentication and sensitive endpoints.
- Validate file type, size, content, and malware scanning policy for uploads.
- Redact personal data from logs and telemetry.
- Back up databases and object storage; test restoration.
- Maintain dependency updates and a vulnerability response process.

## 6. Audit and financial controls

Audit at minimum: login/security events, permission changes, job status changes, evidence submissions, signatures, approvals, payroll input changes, incentive changes, complaints, warranty decisions, exports, and document access.

Financial or payroll-impacting changes require reason, actor, timestamp, before/after values, and approval where applicable.

## 7. Privacy operations

Plan for data subject requests, retention expiry, correction workflows, access reports, and incident response. Regional legal review is required before launch.
