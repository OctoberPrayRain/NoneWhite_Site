# Quality Guidelines

> Code quality standards for frontend development.

---

## Overview

The frontend quality gate is currently build- and route-surface based. There is no frontend unit test framework configured. Root `npm run lint` runs Rust checks and `npm --prefix client run build`; `client/package.json` also provides `build` and `preview` scripts.

For frontend changes, verify both the production build and the affected route surface. Build success alone is not enough for route/page work.

---

## Forbidden Patterns

- Do not hardcode backend origins such as `http://localhost:3000`; use relative `/api/...` paths and Vite proxy behavior.
- Do not add frontend dependencies without a separate reason, package update, and build verification.
- Do not add Pinia, TypeScript, test frameworks, Tailwind, or schema libraries as an incidental change.
- Do not implement avatar upload until storage strategy, URL form, size limit, and file type whitelist are confirmed.
- Do not request favorites data in Phase 2; the profile favorites tab is placeholder-only.
- Do not mark DB-backed auth/profile integration complete without PostgreSQL evidence.

---

## Required Patterns

- Preserve `/` and `/test-api` when editing router, header, or API helpers.
- Use `requestJson()` for new auth/user-style API endpoint wrappers; `client/src/api/test.js` is the existing Phase 1 smoke-test exception.
- Preserve backend `message` values for user-facing errors.
- Use route metadata (`label`, `guestOnly`, `requiresAuth`) to drive header visibility.
- Keep auth token storage key as `nonewhite_auth_token` unless a contract update changes it.
- Record manual route/browser limitations in Role B handoff logs when full browser automation is unavailable.

---

## Testing Requirements

Minimum checks for frontend docs or code changes:

```bash
npm --prefix client run build
npm run lint
```

For route/page changes, also run the built app through Vite preview and check these routes return HTTP 200:

```txt
/
/test-api
/login
/register
/profile
```

If backend-dependent form flows are changed, run with a live backend and PostgreSQL when available. If the environment cannot provide PostgreSQL, document that limitation and do not claim DB-backed happy paths are complete.

---

## Code Review Checklist

- Are new API calls wrapped in `client/src/api/` modules?
- Are request/response fields aligned with `agent/COLLABORATION_PLAN.md`?
- Do login/register/profile pages keep loading, success, and error states visible?
- Does logout clear token and current user state?
- Are avatar upload and favorites still placeholders unless their backend contracts exist?
- Do `npm run lint` and route preview checks pass?
