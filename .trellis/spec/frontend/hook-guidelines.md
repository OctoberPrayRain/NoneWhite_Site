# Hook Guidelines

> How reusable stateful logic is used in this Vue project.

---

## Overview

This is a Vue project, not a React project, so do not introduce React hooks. The current reusable stateful pattern is a lightweight Vue composable-style function: `useAuthStore()` in `client/src/stores/auth.js` returns module-scope refs and actions.

Within components, use Vue primitives directly:

- `ref()` for form fields, status strings, and local UI state.
- `computed()` for derived values such as `isAuthenticated`, `navRoutes`, and `displayInitial`.
- `onMounted()` for initial page data loading.
- `watch()` for syncing local form state from shared state.

---

## Custom Hook Patterns

The only current custom composable-like API is `useAuthStore()`:

```js
const { authToken, currentUser, loadCurrentUser, logout } = useAuthStore()
```

Observed in:

- `client/src/components/AppHeader.vue` for auth-aware navigation and logout.
- `client/src/views/LoginView.vue` for login flow.
- `client/src/views/RegisterView.vue` for registration flow.
- `client/src/views/ProfileView.vue` for current-user loading and logout.

If another shared stateful module is introduced, follow the same pattern: keep module-scope refs private to one `stores/*.js` file and export an explicit `useXStore()` function that returns refs and actions.

---

## Data Fetching

Data fetching is handled by API modules, not directly scattered across templates:

- `client/src/api/http.js` performs `fetch()`, checks that a response envelope has `code === 0`, returns `data`, and throws `ApiError` otherwise.
- `client/src/api/auth.js` wraps register/login endpoints.
- `client/src/api/users.js` wraps current-user, profile update, and password change endpoints.
- Page handlers call store actions or API wrappers and then set visible loading/error/success state.

There is no React Query, SWR, Apollo, or client-side cache library. Do not add one without a separate decision and dependency update.

---

## Naming Conventions

- Shared store accessors use `useXStore()`, currently `useAuthStore()`.
- Async actions use verb phrases: `loginWithCredentials`, `registerWithCredentials`, `loadCurrentUser`, `handleUpdateProfile`, `handleChangePassword`.
- Local validation helpers are plain functions such as `validateForm()`.
- API wrappers use endpoint verbs: `fetchCurrentUser`, `updateCurrentUser`, `changePassword`.

---

## Common Mistakes

- Do not call backend endpoints directly from templates.
- Do not create a new composable for single-page-only state; keep that state local with `ref()`.
- Do not introduce Pinia for Phase 2 auth unless the dependency, app registration, and README/docs are updated as a separate task.
- Do not hide backend `message` values; preserve them through `ApiError` and page error states.
