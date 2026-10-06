# Mobile app plan

## 1. Applications

### Worker app

Primary flow:

1. Sign in and select organization if necessary.
2. View assigned jobs and sync state.
3. Open job details, customer/site information, hazards, and instructions.
4. Start travel and optionally record route points according to policy.
5. Check in on site.
6. Complete the checklist, add notes, materials, costs, and photos.
7. Capture customer signature and submit completion.
8. See whether the submission is pending sync, under review, approved, or rejected.

### Manager app

- Dispatch board and job list.
- Worker availability and current status.
- Map view for active jobs and recent location events where enabled.
- Assignment, reassignment, escalation, approval, rejection, and exception handling.
- Complaint and warranty triage.
- Cost and incentive review.

The manager app may initially be a responsive web app to reduce delivery risk.

## 2. Offline-first design

Local data should include only the minimum required for assigned work. Store encrypted local records where the platform supports it.

- Read cache: assigned jobs, customer/site instructions, checklist templates, reference data.
- Write queue: status changes, GPS events, checklist answers, photos, expenses, signatures.
- Queue states: pending, uploading, submitted, accepted, rejected, needs-review.
- User-visible sync indicator and failed-action retry screen.
- Automatic retry with exponential backoff, but no silent loss.
- If a job is changed by a manager while offline, show the conflict and preserve the worker's unsent evidence.

## 3. Mobile security

- Short-lived access tokens with refresh mechanism.
- Device registration and remote session revocation.
- App lock/biometric unlock as an optional local convenience.
- No sensitive documents in logs, analytics, crash reports, or push payloads.
- Camera capture should attach job context and prevent accidental upload to the wrong job.
- Clear location permission explanation and worker consent record.

## 4. UX requirements

- Large controls usable outdoors and with gloves.
- Minimal typing; use selectable answers, voice notes where permitted, and barcode/QR scanning later.
- Show required steps before submission.
- Make evidence state obvious: local, uploading, synchronized, approved.
- Preserve progress after app termination.
- Support multiple languages and configurable labels.

## 5. Suggested mobile implementation

Start with a cross-platform framework such as React Native or Flutter, with a small native layer for background location, secure storage, camera, and notifications. The exact choice should be validated against the team's skills and required device support before implementation.
