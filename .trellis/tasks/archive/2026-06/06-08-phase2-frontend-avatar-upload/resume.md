# Resume Marker: Phase 2 Frontend Avatar Upload Interaction

## Planned Work

Complete the remaining Phase 2 frontend avatar upload interaction by wiring the profile page to the existing backend `POST /api/users/me/avatar` contract.

## Expected Code Targets

- `client/src/api/users.js`
- `client/src/views/ProfileView.vue`
- `client/src/style.css`
- `README.md`
- `agent/JOURNALIST/B_FRONTEND_PAGE_INTERACTION/B_FRONTEND_PAGE_INTERACTION_LOG.md`

## Guardrails

- Use multipart field `avatar` and `Authorization: Bearer <token>`.
- Do not change backend upload behavior or storage strategy.
- Do not implement favorites data or Phase 3 work.
- Do not add dependencies.
- Do not claim live upload happy path unless it is actually run in this session.

## Verification Plan

- `git diff --check`
- `npm --prefix client run build`
- `npm run lint`
- Vite preview route checks for `/`, `/test-api`, `/login`, `/register`, `/profile`
- LSP diagnostics on changed frontend files

This marker is committed and pushed before implementation begins.
