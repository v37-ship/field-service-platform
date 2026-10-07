# Agent instructions

## Product

This repository contains an independent, mobile-first field-service SaaS platform. The core product is implemented in Rust and PostgreSQL. ERPNext and Frappe HR are optional integrations only; they are not runtime dependencies of the core.

## Technology

- Backend: Rust, Axum, Tokio, SQLx, PostgreSQL.
- Mobile: Android Kotlin/Jetpack Compose and iOS Swift/SwiftUI. Use Kotlin Multiplatform only for shared logic where it reduces duplication.
- Storage: PostgreSQL for structured data and private S3-compatible object storage for media/documents.
- Architecture: modular monolith first; split services only when scale or isolation requires it.

## Development rules

1. Make small, reviewable pull requests. Implement only the requested phase/task.
2. Never commit secrets, real credentials, production URLs, or customer data.
3. Never copy ERPNext/Frappe source code or branding.
4. Every schema change requires a forward SQL migration.
5. Keep tenant isolation in service/repository queries, not only in HTTP handlers.
6. Never trust client-supplied organization IDs, role lists, actor IDs, approval flags, or payroll values.
7. Use stable machine-readable API error codes and safe public messages.
8. Add tests for authorization, tenant isolation, validation, and security-sensitive behavior.
9. Preserve backward compatibility unless the task explicitly requires a breaking change.
10. Update relevant documentation and the PR description.

## Required checks

Run before opening a pull request:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
docker compose config
```

If a check requires Docker or external services, document the exact command and result in the PR. Do not claim a check passed if it was not run.

## Pull request rules

- Target `main`.
- Do not merge the pull request.
- Include a summary, files changed, tests run, security considerations, limitations, and follow-up work.
- Keep unrelated formatting or refactoring out of the PR.
