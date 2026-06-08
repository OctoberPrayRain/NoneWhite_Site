# Bootstrap Guidelines Resume Marker

Status: resumed before implementation on 2026-06-08.

## Planned Work

- Fill `.trellis/spec/frontend/` with project-specific frontend conventions based on real Vue/Vite code in `client/src/`.
- Include concrete file examples from this repository.
- Update the bootstrap task checklist only after the frontend guidelines and examples are filled.
- Avoid changing application behavior while filling Trellis specs.

## Guardrails

- Document current patterns, not aspirational rewrites.
- Do not introduce new frontend dependencies.
- Do not implement avatar upload; storage strategy remains blocked.
- Do not mark backend DB happy paths complete without PostgreSQL evidence.

## Evidence Before Resume

- `git fetch origin` completed.
- `git rev-list --left-right --count HEAD...origin/development` returned `0 0`.
- Active task: `.trellis/tasks/00-bootstrap-guidelines`.
- Frontend spec files currently contain placeholder text and empty examples.
