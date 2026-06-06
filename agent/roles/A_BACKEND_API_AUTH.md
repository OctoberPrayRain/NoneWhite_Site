# Role A Backend/API/Auth Implementation Guide

Role A owns Rust backend implementation for Phase 2 user-system work. This guide splits the backend role into small implementation points so multiple backend people or agents can work without stepping on each other.

## Required Reading

Read these before editing backend code:

1. `README.md`
2. `agent/AGENT_RULES.md`
3. `agent/COLLABORATION_PLAN.md`
4. `agent/roles/README.md`
5. Current backend files under `server/src/`

Role A must not implement API behavior that is missing from the Phase 2 contracts in `agent/COLLABORATION_PLAN.md`.

## Ownership And Shared Files

Role A owns backend code and backend tests:

```txt
server/Cargo.toml
server/Cargo.lock
server/src/**
server/.env.example
server/tests/
```

High-conflict files require coordination with Role C before editing:

```txt
server/src/routes/mod.rs
server/src/response.rs
server/src/error.rs
server/src/config.rs
server/src/db.rs
server/Cargo.toml
server/Cargo.lock
server/.env.example
```

Do not edit frontend files to hide backend contract drift. If the contract is wrong, ask Role C to update the contract first.

## Inputs Required From Role C

Backend implementation may begin only after the required Role C decisions exist:

| Required Input | Provider | Blocks |
|---|---|---|
| C-01 migration and contract file structure | Role C | A-01, A-03 |
| C-02 users table schema or migration | Role C | A-03, A-04, A-05, A-06, A-07 |
| C-03 env variables and precedence | Role C | A-01, A-05 |
| API field and error contract | Role C | A-02, A-04, A-05, A-06, A-07 |

If these are unclear, write tests or scaffolding only where it does not guess the contract.

## Intra-Role Split Lanes

| Lane | Owner Scope | Related Tasks | Handoff Target |
|---|---|---|---|
| A1 Config/State/DB | env loading, config structs, DB pool, app state | A-01 | A3, A4, A5 |
| A2 Response/Error | `ApiResponse`, `AppError`, error-to-envelope conversion, fallback | A-02 | A4, A5 |
| A3 User Data Layer | user model, DTOs, repository functions | A-03 | A4, A5 |
| A4 Auth Flow | register/login tests, handlers, service logic | A-04, A-05 | A5, B2, C4 |
| A5 Authenticated User Flow | auth middleware, `/me`, profile update, password update | A-06, A-07 | B2, B5, C4 |

Each lane owner must list shared files touched and verification run in the handoff.

## Task Map

| Task | Purpose | Depends On | Primary Lane |
|---|---|---|---|
| A-01 | Establish backend app config, env precedence, DB pool, and state | C-03 | A1 |
| A-02 | Establish centralized error and response framework | Contract error table | A2 |
| A-03 | Create User model, DTOs, and repository | C-02, field mapping | A3 |
| A-04 | Implement register API with tests | A-01, A-02, A-03 | A4 |
| A-05 | Implement login API and JWT with tests | A-04, C-03 | A4 |
| A-06 | Implement auth middleware and current user API | A-05 | A5 |
| A-07 | Implement profile update and password change | A-06 | A5 |

## Atomic Implementation Points

### A-01 Config/State/DB

| Point | Owner Output | Verification |
|---|---|---|
| A-01.1 | Define `ServerConfig`, `DatabaseConfig`, `AuthConfig`, `AppConfig` | `cargo check --manifest-path server/Cargo.toml` |
| A-01.2 | Centralize env loading and document `server/.env` precedence | Start backend with explicit `HOST`/`PORT`; `GET /api/test` still works |
| A-01.3 | Add DB pool only after DB dependency is chosen | `cargo check`; no per-request pool creation |
| A-01.4 | Inject app state into router only when handlers need it | Existing `/api/test` route remains available |

### A-02 Response/Error

| Point | Owner Output | Verification |
|---|---|---|
| A-02.1 | Keep `ApiResponse<T>` envelope stable | Existing `/api/test` returns `{ code, data, message }` |
| A-02.2 | Add `AppError` in `server/src/error.rs` | `cargo check` |
| A-02.3 | Convert errors to `(StatusCode, Json<ApiResponse<Value>>)` | Curl error scenario returns contract code/message |
| A-02.4 | Update fallback to contract code `40400` when this task is active | Curl unknown route returns HTTP 404 and `code=40400` |

### A-03 User Data Layer

| Point | Owner Output | Verification |
|---|---|---|
| A-03.1 | Add `UserRow` using DB field names | Struct fields match C-02 schema |
| A-03.2 | Add `UserResponse` using camelCase JSON output | JSON contains `avatarUrl`, `createdAt`, `updatedAt` |
| A-03.3 | Add repository functions for create/find/update | Repository tests or service tests cover each function used |
| A-03.4 | Ensure `password_hash` never enters response DTO | Test or assertion checks response body excludes `password_hash` |

### A-04 Register API

Write tests first:

```txt
register_success_returns_user
register_rejects_invalid_email
register_rejects_short_password
register_rejects_duplicate_email
```

| Point | Owner Output | Verification |
|---|---|---|
| A-04.1 | Register request validation | Invalid email and short password tests fail before implementation, pass after |
| A-04.2 | Password hashing and user insert | Success test creates user without returning `password_hash` |
| A-04.3 | Duplicate username/email mapping | 409 tests return `40901` or `40902` |
| A-04.4 | Route registration under `/api/auth/register` | Curl success returns HTTP 201 and `code=0` |

### A-05 Login API And JWT

Write tests first:

```txt
login_success_returns_token_and_user
login_rejects_invalid_email
login_rejects_wrong_password
```

| Point | Owner Output | Verification |
|---|---|---|
| A-05.1 | Login request validation | Invalid email/password tests pass |
| A-05.2 | Password verification | Wrong password returns HTTP 401 and `40101` |
| A-05.3 | JWT issue using configured secret and expiry | Success response has `token`, `tokenType=Bearer`, `expiresIn` seconds |
| A-05.4 | Route registration under `/api/auth/login` | Curl login success returns `code=0` |

### A-06 Auth Middleware And Current User

Write tests first:

```txt
me_requires_token
me_returns_current_user
```

| Point | Owner Output | Verification |
|---|---|---|
| A-06.1 | Bearer token extraction | Missing token returns HTTP 401 and `40102` |
| A-06.2 | Token verification and user id extraction | Invalid token returns HTTP 401 and `40103` |
| A-06.3 | Current user handler | Valid token returns `UserResponse` |
| A-06.4 | Route registration under `/api/users/me` | Existing `/api/test` remains available |

### A-07 Profile Update And Password Change

| Point | Owner Output | Verification |
|---|---|---|
| A-07.1 | `PATCH /api/users/me` username update only | Direct `avatarUrl` body returns HTTP 400 and `40005` |
| A-07.2 | Username conflict mapping | Conflict returns HTTP 409 and `40901` |
| A-07.3 | `PATCH /api/users/me/password` | Old password fails after change; new password succeeds |
| A-07.4 | Current password validation | Wrong current password returns HTTP 401 and `40104` |

## API Contract Rules

- Every endpoint path starts with `/api`.
- Every body uses `{ code, data, message }`.
- Success business code is `0`.
- Rust structs may use `snake_case`, but JSON response fields must use contract `camelCase`.
- `password`, `password_hash`, JWT secret, and raw database errors never appear in response bodies.
- Register success returns HTTP 201. Other Phase 2 success endpoints return HTTP 200 unless the contract changes.
- `PATCH /api/users/me` cannot directly update `avatarUrl`.

## Backend Verification

Run the narrowest relevant test first, then widen:

```bash
cargo fmt --manifest-path server/Cargo.toml --check
cargo check --manifest-path server/Cargo.toml
cargo test --manifest-path server/Cargo.toml
```

For API changes, also run curl against the real backend. At minimum confirm:

```bash
curl http://127.0.0.1:3000/api/test
```

For changed auth endpoints, include one happy path, one failure case, and the existing `/api/test` regression.

## Handoff Checklist

```md
## Handoff - Role A

Sub-lane:
Task IDs:
Changed Files:
Contracts Consumed:
Contracts Changed:
Verification:
Manual/API QA:
Known Limits:
Conflict Notes:
Next Role Needed:
```

## Stop Conditions

Stop before implementation if:

- C-02 users schema is missing but the task needs DB fields.
- C-03 env contract is missing but the task needs DB/JWT config.
- Token payload or expiry is undefined.
- Avatar storage strategy is undefined but upload implementation is requested.
- A shared file such as `routes/mod.rs`, `response.rs`, `config.rs`, or `Cargo.toml` would be edited by multiple people without an order.
