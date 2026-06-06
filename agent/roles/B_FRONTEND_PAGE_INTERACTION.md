# Role B Frontend/Page/Interaction Implementation Guide

Role B owns Vue frontend implementation for Phase 2 user-system work. This guide splits the frontend role into small implementation points so multiple frontend people or agents can work in parallel without inventing API fields or breaking existing routes.

## Required Reading

Read these before editing frontend code:

1. `README.md`
2. `agent/AGENT_RULES.md`
3. `agent/COLLABORATION_PLAN.md`
4. `agent/roles/README.md`
5. Current frontend files under `client/src/`

Role B must consume the API contract as written. Do not change backend response shapes from frontend code.

## Ownership And Shared Files

Role B owns frontend code and frontend dependencies:

```txt
client/package.json
client/package-lock.json
client/vite.config.js
client/src/api/**
client/src/components/**
client/src/router/**
client/src/stores/**
client/src/views/**
client/src/App.vue
client/src/main.js
client/src/style.css
client/public/**
```

High-conflict files require sequencing:

```txt
client/src/router/index.js
client/src/components/AppHeader.vue
client/src/api/http.js
client/package.json
client/package-lock.json
README.md
```

Do not edit backend code to fit temporary UI assumptions. If an endpoint contract is missing, ask Role C to lock it first.

## Inputs Required From Role C And Role A

| Required Input | Provider | Blocks |
|---|---|---|
| API endpoint paths, request fields, response fields | Role C | B-02, B-03, B-04 |
| Error code and message contract | Role C | B-02, B-03, B-04, B-06, B-07 |
| Token storage strategy | Role C | B-05, B-06, B-07 |
| Register/login/me endpoints available or stubbed by contract | Role A | Real integration for B-03 through B-07 |
| Avatar upload storage strategy | Role C/A | Real upload in B-04/B-07 |

Frontend UI can be built against contract examples, but cannot be marked complete as integrated until the real backend is verified.

## Intra-Role Split Lanes

| Lane | Owner Scope | Related Tasks | Handoff Target |
|---|---|---|---|
| B1 Baseline/HTTP | dependency install/build, `api/http.js`, preserve `/test-api` | B-01, B-02 | B2, B4, B5 |
| B2 API Clients | `api/auth.js`, `api/users.js`, token headers | B-03, B-04 | B3, B4, B5 |
| B3 Auth State | `stores/auth.js`, token persistence, logout | B-05 | B4, B5 |
| B4 Login/Register Views | login/register routes, forms, validation states | B-06 | C4 integration QA |
| B5 Profile Views | profile page, profile/password/avatar/favorites UI | B-07 | C4 integration QA |

Each lane owner must verify `/` and `/test-api` are not broken if they touch router, API, shared styles, or header navigation.

## Task Map

| Task | Purpose | Depends On | Primary Lane |
|---|---|---|---|
| B-01 | Confirm frontend dependencies and build baseline | Current `client/package.json` | B1 |
| B-02 | Create unified HTTP client | API envelope contract | B1 |
| B-03 | Create auth API client | B-02, auth contract | B2 |
| B-04 | Create users API client | B-02, users contract, token header contract | B2 |
| B-05 | Create auth state management | Token storage strategy | B3 |
| B-06 | Create login/register pages | B-03, B-05 | B4 |
| B-07 | Create profile page | B-04, B-05 | B5 |

## Atomic Implementation Points

### B-01 Frontend Baseline

| Point | Owner Output | Verification |
|---|---|---|
| B-01.1 | Install dependencies when missing | `npm --prefix client install` |
| B-01.2 | Confirm production build baseline | `npm --prefix client run build` |
| B-01.3 | Confirm existing routes still load | Manual/browser check `/` and `/test-api` |

### B-02 Unified HTTP Client

| Point | Owner Output | Verification |
|---|---|---|
| B-02.1 | Add `client/src/api/http.js` with `requestJson` | Build passes |
| B-02.2 | Parse `{ code, data, message }` envelope consistently | Unit/manual check handles `body.code !== 0` |
| B-02.3 | Preserve backend `message` for errors | Failed request displays backend message |
| B-02.4 | Preserve `/test-api` behavior | `/test-api` still calls `GET /api/test` |

### B-03 Auth API Client

| Point | Owner Output | Verification |
|---|---|---|
| B-03.1 | Add `register({ username, email, password })` | Request excludes `confirmPassword` |
| B-03.2 | Add `login({ email, password })` | Returns contract `body.data` only |
| B-03.3 | Add `logoutLocal()` if needed by auth store | Does not call fake backend logout endpoint |
| B-03.4 | Keep all paths relative under `/api` | No hardcoded backend origin |

### B-04 Users API Client

| Point | Owner Output | Verification |
|---|---|---|
| B-04.1 | Add `fetchCurrentUser(authToken)` | Sends `Authorization: Bearer <token>` |
| B-04.2 | Add `updateCurrentUser(payload, authToken)` | Sends only allowed profile fields |
| B-04.3 | Add `changePassword(payload, authToken)` | Uses `currentPassword` and `newPassword` fields |
| B-04.4 | Add `uploadAvatar(file, authToken)` only after upload contract is confirmed | If not confirmed, leave documented TODO and no fake call |

### B-05 Auth State

| Point | Owner Output | Verification |
|---|---|---|
| B-05.1 | Add `client/src/stores/auth.js` | Build passes |
| B-05.2 | Load token from `localStorage` key `nonewhite_auth_token` | Refresh preserves token state |
| B-05.3 | Provide `saveToken`, `clearAuth`, `logout` | Logout clears token and `currentUser` |
| B-05.4 | Decide Pinia vs lightweight store according to plan | If no Pinia, file comment states Phase 2 uses lightweight store |

### B-06 Login/Register Views

| Point | Owner Output | Verification |
|---|---|---|
| B-06.1 | Add `/login` route and `LoginView.vue` | Route is accessible |
| B-06.2 | Add `/register` route and `RegisterView.vue` | Route is accessible |
| B-06.3 | Implement form fields from contract | `confirmPassword` is frontend-only and not submitted |
| B-06.4 | Implement loading/error/success states | Empty/malformed inputs produce visible state |
| B-06.5 | Update header navigation only if needed | `/` and `/test-api` remain reachable |

### B-07 Profile View

| Point | Owner Output | Verification |
|---|---|---|
| B-07.1 | Add `/profile` route and `ProfileView.vue` | Route is accessible |
| B-07.2 | Show unauthenticated state clearly | Missing token does not crash page |
| B-07.3 | Show `currentUser.username`, `email`, `avatarUrl`, `role` | Field names match API contract |
| B-07.4 | Add profile edit and password UI | Errors display backend `message` |
| B-07.5 | Add avatar area only as confirmed upload or “待接入” placeholder | No fake upload success |
| B-07.6 | Add favorites tab as Phase 2 UI placeholder only | Does not request favorites API or mark Phase 4 complete |

## API Client Rules

- Use `client/src/api/` wrappers for API calls.
- Use relative paths such as `/api/auth/login`.
- Do not hardcode `http://localhost:3000` in app code.
- Do not invent response fields. Use `avatarUrl`, `createdAt`, `updatedAt`, `tokenType`, and `expiresIn` exactly as contracted.
- Prefer one shared envelope parser over repeated `fetch` error logic.
- Do not swallow backend `message`.

## Page Rules

- Every new page must be registered in `client/src/router/index.js`.
- `meta.label` must remain a short Chinese navigation label if the route appears in the header.
- If a route should not be visible in the header, adjust header filtering once in `AppHeader.vue` instead of hacking individual pages.
- `ProfileView.vue` must not request favorites API in Phase 2.
- Avatar upload must remain placeholder-only until storage strategy is confirmed.

## Frontend Verification

Run:

```bash
npm --prefix client install
npm --prefix client run build
```

For route or page changes, verify the real UI surface:

```txt
/
/test-api
/login
/register
/profile
```

If browser automation is unavailable, record that limitation and provide manual steps. Build alone is not enough for page behavior, but it is still required.

## Handoff Checklist

```md
## Handoff - Role B

Sub-lane:
Task IDs:
Changed Files:
Contracts Consumed:
Contracts Changed:
Verification:
Manual/UI QA:
Known Limits:
Conflict Notes:
Next Role Needed:
```

## Stop Conditions

Stop before implementation if:

- API request/response fields are unclear.
- Token storage strategy is not agreed.
- Backend endpoint does not exist and no contract example exists.
- Avatar storage strategy is undefined but real upload is requested.
- Header/router changes would conflict with another frontend owner.
- A page would need Phase 3+ data to look complete.
