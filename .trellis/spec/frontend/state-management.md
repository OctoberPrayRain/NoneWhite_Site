# State Management

> How state is managed in this project.

---

## Overview

Phase 2 uses a lightweight auth store implemented with Vue `ref()` values in `client/src/stores/auth.js`. Pinia is not installed and is not registered in `client/src/main.js`.

The store is intentionally narrow: it owns auth token persistence, current user, auth status, and auth error message. Page-specific form fields, tabs, loading messages, and validation errors remain local to the route component.

---

## State Categories

Local component state:

- Login form fields and status in `LoginView.vue`.
- Register form fields, `confirmPassword`, and status in `RegisterView.vue`.
- Profile tab, username form, password form, and message refs in `ProfileView.vue`.

Shared auth state:

- `authToken`, `tokenType`, `expiresIn`, `currentUser`, `authStatus`, and `authErrorMessage` in `client/src/stores/auth.js`.
- Token persistence uses `localStorage` key `nonewhite_auth_token`.

Router state:

- Route visibility is derived from `route.meta.guestOnly` and `route.meta.requiresAuth` in `AppHeader.vue`.
- Current routes are centralized in `client/src/router/index.js`.

Server state:

- Current user data is loaded on demand with `loadCurrentUser()` and stored in `currentUser`.
- There is no general server-state cache.

---

## When to Use Global State

Use the auth store only for state that must be shared across the header and multiple pages:

- The saved auth token.
- The current user displayed in the header/profile.
- Logout and auth clearing behavior.

Keep state local when it is used by one route or one form. Examples: `activeTab`, `profileUsername`, `currentPassword`, `confirmNewPassword`, form status strings, and success/error messages.

---

## Server State

Server calls should flow through API modules and store/page actions:

```js
currentUser.value = await fetchCurrentUser(authToken.value)
```

The current project does not cache lists, paginate data, or synchronize background server state. Favorites are a Phase 4 placeholder in `ProfileView.vue` and must not request a favorites API during Phase 2.

When a 401 occurs while loading the current user, `loadCurrentUser()` clears auth state through `clearAuth()`. Preserve that behavior for token invalidation paths.

---

## Common Mistakes

- Do not add Pinia without adding the dependency, registering it in `main.js`, and updating project docs.
- Do not store password fields, confirm-password fields, or transient form errors globally.
- Do not create global state for avatar upload while the upload contract is blocked.
- Do not keep stale `currentUser` after a 401; clear auth state.
