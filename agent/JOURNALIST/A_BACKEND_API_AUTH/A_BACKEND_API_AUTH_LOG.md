# Role A Backend/API/Auth Log

## Handoff - Role A

Sub-lane: A1 Config/State/DB, A3 User Data Layer
Task IDs: A-01, A-03
Changed Files: _Fill after each backend handoff_
Contracts Consumed: `agent/COLLABORATION_PLAN.md`, `server/migrations/20260605000000_create_users.sql`
Contracts Changed: _None unless explicitly recorded_
Verification: _Record `cargo fmt`, `cargo check`, `cargo test`, and API regression output_
Manual/API QA: _Record curl commands and observed response bodies_
Known Limits: Register/login/auth middleware are separate follow-up tasks.
Conflict Notes: `server/Cargo.toml`, `server/src/config.rs`, and route files are high-conflict files.
Next Role Needed: Role C should verify contract drift before Phase 2 auth API implementation.

## Handoff - Role A - 2026-06-06

Sub-lane: A2 Response/Error, A4 Auth Flow, A5 Authenticated User Flow
Task IDs: A-02, A-04, A-05, A-06, A-07
Changed Files:
- `server/Cargo.toml`, `server/Cargo.lock`: added `bcrypt` for password hashing and `jsonwebtoken` for JWT issuing/verification.
- `server/src/main.rs`, `server/src/state.rs`, `server/src/db.rs`: added lazy PostgreSQL pool and shared `AppState` injection while preserving `/api/test` startup without immediate DB connection.
- `server/src/error.rs`: centralized contract error responses for auth/user flows.
- `server/src/dto/auth.rs`, `server/src/dto/users.rs`: added `RegisterRequest`, `LoginRequest`, `AuthTokenResponse`, `UpdateUserProfileRequest`, and `ChangePasswordRequest`.
- `server/src/services/auth_service.rs`, `server/src/services/user_service.rs`: implemented validation, bcrypt hashing/verifying via `spawn_blocking`, JWT issue/verify, current-user lookup, profile update, and password change logic.
- `server/src/middleware/auth.rs`: added Bearer token extraction and authenticated user ID resolution.
- `server/src/routes/auth.rs`, `server/src/routes/users.rs`, `server/src/routes/mod.rs`: registered `POST /api/auth/register`, `POST /api/auth/login`, `GET/PATCH /api/users/me`, and `PATCH /api/users/me/password`.
Contracts Consumed: `agent/COLLABORATION_PLAN.md` sections 10.1-10.5, 13.2, 14.1-14.8; `agent/roles/A_BACKEND_API_AUTH.md` A-04 through A-07.
Contracts Changed: none in API shape; implementation follows existing Phase 2 contract. Docs updated separately by Role C for append-only log rule and README status.
Verification:
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 8 tests.
Manual/API QA:
- `GET /api/test`: HTTP 200, `code=0`, existing backend test response preserved.
- `GET /api/users/me` without token: HTTP 401, `code=40102`, `message="Authentication is required"`.
- `POST /api/auth/login` with invalid email: HTTP 400, `code=40002`.
- `POST /api/auth/register` with invalid email: HTTP 400, `code=40002`.
Known Limits:
- Full DB happy path for register/login/current-user/profile/password was not executed because this environment lacks Docker daemon, `docker compose`, `docker-compose`, and `psql`.
- Password validation currently enforces 8+ chars and <=72 bytes for bcrypt safety; username validation enforces 3-32 chars but does not yet restrict character class.
- Avatar upload remains blocked by storage strategy.
Conflict Notes: touched shared files `server/Cargo.toml`, `server/Cargo.lock`, `server/src/routes/mod.rs`, and `server/src/main.rs`.
Next Role Needed: Role C should run PostgreSQL migration and DB-backed curl happy paths in an environment with PostgreSQL, then update README checkboxes if integration passes.

## Handoff - Role A - 2026-06-07

Sub-lane: A1 Config/State/DB
Task IDs: A-01.2
Scenario: Backend env precedence now matches the Phase 2 collaboration contract.
Changed Files:
- `server/src/main.rs`: replaced ad hoc dotenv loading with a focused `load_env_files()` helper that loads `server/.env` before root `.env`, using `CARGO_MANIFEST_DIR` so launcher scripts and manifest-based cargo commands resolve the same files.
- `README.md`: updated the backend env precedence section to match the fixed behavior.
- `.trellis/tasks/06-07-role-a-env-precedence/resume.md`: recorded the required pre-implementation resume marker.
Contracts Consumed:
- `agent/COLLABORATION_PLAN.md` environment variable contract: backend `server/.env` should be the primary backend config source, with root `.env` for project defaults.
- `.trellis/tasks/06-07-role-a-env-precedence/prd.md`: A-01.2 scenario contract for server override, root fallback, and `/api/test` regression.
Contracts Changed: none in API shape; backend env loading behavior now aligns with the existing contract.
Verification:
- TDD red phase: `cargo test --manifest-path server/Cargo.toml load_env_files` first failed because `load_env_files_from_paths` did not exist.
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 10 tests.
- `npm run lint`: passed; Rust fmt/check and frontend Vite production build completed.
- `docker compose config`: passed.
- `docker ps`: failed because Docker daemon socket is unavailable at `/var/run/docker.sock`.
- `psql --version`: failed because `psql` is not installed.
Manual/API QA:
- `PORT=3017 cargo run --manifest-path server/Cargo.toml`, then `curl -i http://127.0.0.1:3017/api/test`: HTTP 200 with `code=0` and the existing backend test API response.
Known Limits:
- DB happy path for register/login/current-user/profile/password remains blocked in this environment because Docker daemon and `psql` are unavailable.
- Avatar upload remains blocked by storage strategy.
Next Role Needed: In a PostgreSQL-capable environment, Role C should run the DB-backed happy path and append evidence; no further Role A env precedence work is pending.
