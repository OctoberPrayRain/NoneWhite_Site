# Role C Database/Contracts/Docs/QA Log

## Handoff - Role C

Sub-lane: C1 Structure/Schema, C3 Env/Docs/README, C4 Integration QA
Task IDs: C-01, C-02, C-03, C-04
Changed Files: _Fill after each contract or QA handoff_
Contracts Created: `server/migrations/20260605000000_create_users.sql`
Contracts Changed: _None unless explicitly recorded_
Verification: _Record schema inspection, env consistency checks, and project verification output_
Manual/API/UI QA: _Record curl/browser checks and actual output_
Known Limits: Avatar storage strategy is not confirmed; do not implement avatar upload yet.
Conflict Notes: README status must not be marked complete without implementation and verification evidence.
Next Role Needed: Role A can consume the users migration and env contract for backend user/auth implementation.

## QA Evidence - Role C

Task IDs: _Fill per QA run_
Scenario: _Describe scenario_
Surface: _API, DB, docs, or UI_
Command Or Manual Steps: _Exact command or steps_
Expected: _Expected pass/fail condition_
Actual: _Observed output_
Result: _Pass/Fail_
Artifacts: _Files, screenshots, logs, or command output_
Follow-up Needed: _Remaining action_

## Handoff - Role C - 2026-06-06

Sub-lane: C2 API Contract/Field Map, C3 Env/Docs/README, C4 Integration QA
Task IDs: C-04, INT-01, INT-02, INT-03
Changed Files:
- `agent/COLLABORATION_PLAN.md`: added `18.1 JOURNALIST 日志写入规则`, documenting append-only log behavior and sensitive-data exclusions.
- `README.md`: documented backend Phase 2 auth/user endpoint implementation status, current backend source layout, SQL migration command, API curl examples, and DB联调 limitation.
Contracts Created: none.
Contracts Changed:
- JOURNALIST logs are now explicitly append-only: only append new handoff/QA blocks at file end; do not delete, reorder, or rewrite older log content.
Verification:
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 8 tests.
- `docker compose config`: failed because Docker CLI has no `compose` subcommand in this environment.
- `docker-compose config`: failed because `docker-compose` is not installed.
- `psql --version`: failed because `psql` is not installed.
- `docker --version && docker ps`: Docker CLI exists, but daemon socket is unavailable.
Manual/API/UI QA:
- Backend launched with `cargo run --manifest-path server/Cargo.toml`.
- `GET /api/test`: HTTP 200 with `{"code":0,"data":{"service":"NoneWhite_Site Rust API","status":"ok"},"message":"Backend test API is running"}`.
- `GET /api/users/me` without token: HTTP 401 with `code=40102`.
- `POST /api/auth/login` invalid email: HTTP 400 with `code=40002`.
- `POST /api/auth/register` invalid email: HTTP 400 with `code=40002`.
Known Limits:
- README checkboxes for register/login, auth middleware, profile APIs, and password API remain unchecked because DB happy paths could not be run in this environment.
- Avatar upload remains blocked by storage strategy.
Conflict Notes: README status intentionally says “后端已实现 / 待数据库联调” to avoid marking unverified DB integration complete.
Next Role Needed: In an environment with PostgreSQL, run migration, register a user, login, call `/api/users/me`, update username, change password, and then update README checkboxes if all pass.

## QA Evidence - Role C - 2026-06-06

Task IDs: C-04, INT-03
Scenario: Non-DB backend API regression after Phase 2 auth/user implementation.
Surface: Rust API.
Command Or Manual Steps: `cargo run --manifest-path server/Cargo.toml`, then curl `/api/test`, `/api/users/me` without token, invalid-email login, and invalid-email register.
Expected: Existing `/api/test` still returns 200/code 0; auth failures return contract HTTP statuses and error codes.
Actual: `/api/test` returned 200/code 0; missing token returned 401/code 40102; invalid email for login/register returned 400/code 40002.
Result: Pass for non-DB regression and validation paths.
Artifacts: Command output captured in current development session; no secrets or JWT values recorded.
Follow-up Needed: Run DB-backed happy paths once Docker/PostgreSQL/psql are available.

## QA Evidence - Role C - 2026-06-06 - Final Project Lint

Task IDs: C-04, INT-03
Scenario: Full project verification after backend auth/user implementation and documentation updates.
Surface: Rust backend and Vue frontend build.
Command Or Manual Steps:
- First run: `npm run lint`.
- Dependency recovery: `npm --prefix client install`.
- Final run: `npm run lint`.
Expected: Root lint script completes Rust format/check and frontend production build.
Actual:
- First `npm run lint` failed at `vite: command not found` because `client/node_modules` was missing.
- `npm --prefix client install` added 39 packages, audited 40 packages, and found 0 vulnerabilities.
- Final `npm run lint` passed: Rust `cargo fmt --check` and `cargo check` passed; `npm --prefix client run build` completed with Vite output under `client/dist`.
Result: Pass after installing frontend dependencies.
Artifacts: Command output captured in current development session; no secrets or tokens recorded.
Follow-up Needed: DB-backed happy paths still require a running PostgreSQL environment.

## Documentation Correction - Role C - 2026-06-06

Task IDs: C-03, C-04, INT-03
Scenario: README Phase 2 backend checklist needed clearer completed markers for code-finished backend items.
Changed Files:
- `README.md`: marked implemented backend auth/user API items as `[x]` while keeping explicit DB happy path caveats; added a dedicated “Phase 2 后端待补 / 待联调” list for PostgreSQL migration, DB-backed happy path checks, and avatar upload strategy/API work.
Reason:
- Register/login, auth middleware, current-user/profile, and password-change code paths are implemented and covered by Rust compile/unit/non-DB regression checks.
- DB-backed happy paths are still not verified because this environment lacks usable Docker Compose/PostgreSQL/psql.
Append-only Note: This entry corrects the previous README-status wording by appending a new log block; older log entries were not rewritten.
Follow-up Needed: Once PostgreSQL is available, append DB happy path QA evidence and update any remaining README caveats if the full flow passes.

## Handoff - Role C - 2026-06-07

Sub-lane: C3 Env/Docs/README, C4 Integration QA
Task IDs: C-03.2, C-04.1, C-04.2, INT-03
Scenario: Role C non-blocked API QA documentation closeout after backend auth/user implementation.
Changed Files:
- `README.md`: documented current backend env loading precedence; added explicit expected HTTP status and `body.code` outcomes for register, login, current-user, invalid-email register, wrong-password login, and missing-token current-user curl examples; clarified current Docker/psql environment limitation.
- `agent/JOURNALIST/C_DATABASE_CONTRACTS_DOCS_QA/C_DATABASE_CONTRACTS_DOCS_QA_LOG.md`: appended this handoff/QA evidence block.
Contracts Created: none.
Contracts Changed:
- README now records current observed backend env precedence as shell environment > root `.env` > `server/.env` > code defaults, because `server/src/main.rs` loads `../.env` before `server/.env` and `dotenvy` does not overwrite existing variables by default.
- Collaboration preference still recommends `server/.env` as the primary backend config source; this remains a documented implementation-contract mismatch for a future backend code task, not a Role C docs-only code change.
Verification:
- `docker compose config`: passed and rendered the PostgreSQL service configuration.
- `docker ps`: failed because Docker daemon socket is unavailable at `/var/run/docker.sock`.
- `psql --version`: failed because `psql` is not installed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 8 tests.
- `npm run lint`: passed; Rust fmt/check and frontend Vite production build completed.
Manual/API/UI QA:
- No PostgreSQL-backed curl happy path was run in this environment because Docker daemon and `psql` are unavailable.
- README curl examples document expected contract outcomes only; no real JWT, password, secret, database dump, or `.env` value was recorded.
Known Limits:
- DB happy path remains pending: run migration, register, login, `GET /api/users/me`, update username, and change password in a working PostgreSQL environment.
- Avatar upload remains blocked by unresolved storage strategy, upload target, URL form, size limit, and file type whitelist.
Result: Pass for C-03.2/C-04.1/C-04.2 documentation closeout and INT-03 available regression checks; DB-backed integration remains blocked by environment.
Follow-up Needed: Backend owner should decide whether to change env loading so `server/.env` overrides root `.env`; Role C should append DB happy path evidence once PostgreSQL and `psql` are available.
