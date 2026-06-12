# Resume Marker: Phase 2 README Frontend Status Alignment

## Planned Work

Update `README.md` so the Phase 2 frontend checklist matches verified current Vue/Vite code.

## Scope

- Mark register/login pages complete with frontend-only evidence.
- Mark logout complete with auth-store/header evidence.
- Mark profile page complete only as profile display/edit/password plus avatar placeholder.
- Mark favorites tab complete only as a Phase 2 UI placeholder.

## Out of Scope

- Do not implement or mark complete real avatar upload.
- Do not run or mark complete PostgreSQL-backed DB happy paths in this environment.
- Do not implement favorites data or Phase 4 APIs.
- Do not start Phase 3.

## Verification Plan

- `git diff --check`
- `npm --prefix client run build`
- `npm run lint`
- README/API searches to confirm wording and no new upload/favorites API claims
- Vite preview route checks for `/`, `/test-api`, `/login`, `/register`, `/profile`

This marker is committed and pushed before editing `README.md` to satisfy the pre-implementation marker requirement.

## Superseded Note

Before this task's README edit could be pushed, remote `development` advanced with Phase 2 backend avatar/API/DB evidence and README contract cleanup. The remote README now already distinguishes completed frontend profile surfaces from the remaining unchecked frontend avatar upload interaction, so this README-status task is superseded and should be archived without additional README changes.
