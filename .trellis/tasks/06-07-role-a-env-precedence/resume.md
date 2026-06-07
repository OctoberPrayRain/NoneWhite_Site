# Role A Env Precedence Resume Marker

Status: resumed before implementation on 2026-06-07.

## Planned Work

- Complete A-01.2 by making backend env loading prefer `server/.env` over root `.env` for duplicate keys.
- Keep root `.env` as fallback for keys omitted by `server/.env`.
- Add focused Rust regression coverage for server override and root fallback behavior.
- Verify backend tests and the non-DB `/api/test` regression surface.

## Guardrails

- Do not change frontend files.
- Do not implement avatar upload; storage strategy is still blocked.
- Do not mark DB happy path complete without a real PostgreSQL-backed run.
- Do not change API contracts beyond backend env-loading behavior.

## Evidence Before Resume

- `git fetch origin` completed.
- `git rev-list --left-right --count HEAD...origin/development` returned `0 0`.
- Active Trellis task: `.trellis/tasks/06-07-role-a-env-precedence`.
- Current code in `server/src/main.rs` loads root `../.env` before `server/.env` using non-override dotenvy calls.
