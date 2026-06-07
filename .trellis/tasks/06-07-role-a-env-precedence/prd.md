# Role A Env Precedence

Status: in progress marker before implementation.

## Goal

Complete the executable unfinished Role A item A-01.2 by making backend environment loading match the collaboration contract: `server/.env` must take precedence over the root `.env`, while root `.env` can still provide values that are absent from `server/.env`.

## Scope

- Add test coverage for backend env-file precedence before changing production code.
- Apply the smallest backend change needed for the expected precedence.
- Verify the backend tests and the existing `/api/test` regression surface.
- Record Role A handoff evidence after verification.

## Non-Goals

- Do not implement avatar upload; storage strategy is still blocked.
- Do not change frontend files.
- Do not mark DB happy path as complete unless a real PostgreSQL-backed flow is executed and passes.
- Do not alter API contracts beyond env-loading behavior.

## Scenario Contract

1. Happy path: when both root `.env` and `server/.env` define the same key, the backend uses the `server/.env` value.
2. Edge path: when `server/.env` omits a key present in root `.env`, the backend still reads the root `.env` value.
3. Regression path: after the env-loading change, `GET /api/test` still returns HTTP 200 with `code=0`.

## Evidence Before Work

- `git fetch origin` completed.
- `git rev-list --left-right --count HEAD...origin/development` returned `0 0`.
- `cargo test --manifest-path server/Cargo.toml` passed 8 tests before implementation.
- Direct inspection found current `server/src/main.rs` loads root `../.env` before current `server/.env`, which conflicts with A-01.2 because dotenv-style loading does not override existing values by default.
