# Phase 2 Frontend Avatar Upload Interaction

## Goal

Implement the remaining Phase 2 frontend avatar upload interaction on the profile page, using the backend contract that now exists.

## Scope

- Add a frontend API wrapper for `POST /api/users/me/avatar` in `client/src/api/users.js`.
- Replace the disabled avatar placeholder in `client/src/views/ProfileView.vue` with a real file input and upload action.
- Use `multipart/form-data` with field name `avatar` and `Authorization: Bearer <token>`.
- On upload success, update `currentUser.avatarUrl` from the API response and keep the existing avatar preview behavior.
- Show user-visible loading, success, validation, and error states consistent with existing profile/password forms.
- Update `README.md` only after implementation verification, keeping backend/Phase 5 wording precise.
- Append Role B handoff evidence to `agent/JOURNALIST/B_FRONTEND_PAGE_INTERACTION/B_FRONTEND_PAGE_INTERACTION_LOG.md`.

## Out of Scope

- Do not change backend upload behavior.
- Do not change storage strategy or Phase 5 file-upload decisions.
- Do not request favorites APIs or implement Phase 4 favorites data.
- Do not start Phase 3.
- Do not add frontend dependencies or a test framework.

## Contract

See `research/avatar-upload-contract.md` for the endpoint, field name, accepted types, size limit, response shape, and verification constraints.

## Verification

Minimum checks:

```bash
git diff --check
npm --prefix client run build
npm run lint
```

Route-surface check:

```txt
/
/test-api
/login
/register
/profile
```

If no live backend/PostgreSQL environment is available in this session, do not claim a live frontend upload happy path. Record the limitation in the Role B log and README wording if needed.

## Done Criteria

- Profile page has a real avatar upload form/action instead of a disabled placeholder.
- Upload uses the exact backend contract and auth token.
- Successful response updates the visible avatar URL state.
- README Phase 2 frontend checklist accurately reflects the verified frontend state without overclaiming live DB/browser coverage.
- Role B log records changed files, verification commands, manual limitations, and remaining non-goals.
- Changes are committed, pushed, and the Trellis task is archived.
