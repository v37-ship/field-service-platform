# Phase 2A provider task: authentication and tenant authorization

## Repository

- Repository: `v37-ship/field-service-platform`
- Base branch: `main`
- Current baseline: Phase 1 Rust/PostgreSQL foundation is merged.

## Objective

Implement only the authentication boundary, trusted actor context, role memberships, and tenant-aware authorization foundation. This is a deliberately small follow-up to Phase 1.

Do **not** implement customer/site CRUD, mobile apps, payroll, accounting, jobs, travel, payments, or external integrations in this task.

## Required implementation

1. Add an authentication boundary suitable for future OIDC/passwordless/provider integration.
2. Add a development/test authentication mechanism using explicit test fixtures or headers only where safe.
3. Ensure development authentication cannot run when `APP_ENV=production`.
4. Add middleware or extractors that create trusted actor identity and organization context from server-controlled authentication data.
5. Ignore or reject client-supplied organization IDs, roles, and actor IDs for authorization purposes.
6. Add database migration(s) for organization membership and role assignment data as needed.
7. Enforce that memberships and role assignments cannot cross organization boundaries.
8. Add authorization helpers for authenticated users and required roles.
9. Ensure deactivated users and inactive memberships cannot authorize requests.
10. Add audit events for authentication and authorization decisions where appropriate, without logging credentials or tokens.
11. Add unit/integration tests for the security boundaries.
12. Update backend documentation with the auth model and development-auth limitations.

## Security acceptance criteria

- Protected routes reject unauthenticated requests.
- A request cannot select another organization by sending a different `organization_id`.
- A request cannot elevate privileges by sending a different role list.
- Cross-tenant membership/role access is denied.
- Deactivated users cannot authenticate or authorize protected operations.
- Development/test authentication fails closed in production configuration.
- Error responses do not reveal whether a resource exists in another tenant.
- Tokens, credentials, and sensitive authentication headers are not logged.

## Required checks

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
docker compose config
```

If PostgreSQL-backed integration tests are added, document the exact Docker command required to run them and include a deterministic setup/teardown strategy.

## Pull request requirements

- Open one focused PR against `main`.
- Do not merge it.
- Include a concise implementation summary.
- List migrations and security-sensitive decisions.
- List all checks actually run and their results.
- Include limitations and the recommended follow-up for Phase 2B.
