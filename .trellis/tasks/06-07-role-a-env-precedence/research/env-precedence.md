# Env Precedence Research

## Source Findings

- `agent/roles/A_BACKEND_API_AUTH.md` A-01.2 requires centralized env loading and documents `server/.env` precedence.
- `agent/COLLABORATION_PLAN.md` A-01 says backend env precedence must be explicit: `server/.env` before root `.env`.
- Context7 docs for `dotenvy` state override mode makes file variables take precedence over existing environment variables. Default loading preserves existing environment values.

## Implementation Implication

To allow root `.env` fallback while making `server/.env` win, load root values first without override, then load server values with override behavior.
