# Role C API QA Docs

Status: in progress marker before implementation.

## Goal

Complete the non-blocked Role C documentation and QA work for Phase 2 backend auth/user contracts: document backend env precedence clearly, maintain curl examples with expected status/code outcomes, and record current verification evidence without overstating DB-backed completion.

## Scope

- C-03.2: clarify `server/.env` vs root `.env` precedence for backend startup.
- C-04.1: maintain register/login/current-user curl examples with expected HTTP status and `code`.
- C-04.2: maintain failure examples for invalid email, wrong password, and missing token.
- INT-03: run and record available regression checks.

## Non-Goals

- Do not implement application code.
- Do not modify frontend files or create Phase 2 frontend pages.
- Do not mark DB happy path complete unless a real PostgreSQL-backed flow runs and passes.
- Do not implement avatar upload; storage strategy is still unresolved.

## Scenario Contract

1. Happy path docs: README contains register/login/current-user examples with binary expected HTTP status and `code` outcomes.
2. Edge docs: README contains failure examples for invalid email, wrong password, and missing token with expected status/code/message.
3. Regression evidence: Role C log records current available checks: `docker compose config`, `cargo test`, `npm run lint`, and any Docker daemon/psql blockers observed.

## Evidence Before Work

- `git fetch origin` completed.
- `git rev-list --left-right --count HEAD...origin/development` returned `0 0`.
- `docker compose config` passed in this environment.
- `docker ps` failed because Docker daemon socket is unavailable.
- Existing Role C logs say DB happy paths remain pending.
