# Directory Structure

> How frontend code is organized in this project.

---

## Overview

The frontend is a Vue 3 + Vite single-page app under `client/`. Current code is plain JavaScript, Vue Router, global CSS, and small API/store modules. Keep new frontend code inside `client/src/` unless changing Vite or package configuration.

Do not create framework-level directories before they are needed. The current structure is intentionally small: pages live in `views/`, reusable shell components live in `components/`, HTTP wrappers live in `api/`, shared auth state lives in `stores/`, and global styling lives in `style.css`.

---

## Directory Layout

```txt
client/src/
├── api/                 # Relative /api request helpers and endpoint wrappers
│   ├── auth.js          # register/login/logoutLocal
│   ├── http.js          # API envelope parser and ApiError
│   ├── test.js          # Phase 1 /api/test helper
│   └── users.js         # current-user/profile/password wrappers
├── components/          # Shared layout components
│   ├── AppFooter.vue
│   └── AppHeader.vue
├── router/
│   └── index.js         # Vue Router routes, route meta, route export for header
├── stores/
│   └── auth.js          # Lightweight module-scope auth store using Vue refs
├── views/               # Route-level pages
│   ├── HomeView.vue
│   ├── LoginView.vue
│   ├── ProfileView.vue
│   ├── RegisterView.vue
│   └── TestApiView.vue
├── App.vue              # App shell composition
├── main.js              # Vue app creation and router registration
└── style.css            # Global CSS classes for shell, auth pages, profile pages
```

---

## Module Organization

Route-level features are organized by page, not by nested feature folders. For the current Phase 2 user system, `LoginView.vue`, `RegisterView.vue`, and `ProfileView.vue` contain page-local form state and call shared API/store modules.

API code is grouped by backend surface:

- `client/src/api/http.js` owns common response parsing, auth header creation, and `ApiError`.
- `client/src/api/auth.js` owns `/api/auth/register` and `/api/auth/login`.
- `client/src/api/users.js` owns `/api/users/me` and `/api/users/me/password`.

Router metadata is centralized in `client/src/router/index.js`. Header navigation reads the exported `routes` array from that file instead of duplicating route labels in `AppHeader.vue`.

---

## Naming Conventions

- Vue route pages and layout components use `PascalCase.vue`: `LoginView.vue`, `ProfileView.vue`, `AppHeader.vue`.
- JavaScript modules use lower camel-case or domain names: `auth.js`, `http.js`, `users.js`.
- Routes use short kebab-case paths and lower-case route names: `/test-api`, `test-api`, `/profile`, `profile`.
- Route labels shown in the header are short Chinese strings stored in `route.meta.label`.
- CSS class names use kebab-case global classes such as `auth-page`, `profile-card`, `notice-box`, and `primary-button`.

---

## Examples

- `client/src/router/index.js` is the canonical route registration example. It preserves `/` and `/test-api`, adds Phase 2 auth routes, and uses `meta.guestOnly` / `meta.requiresAuth` for header filtering.
- `client/src/components/AppHeader.vue` is the shared navigation example. It derives visible links from router metadata and handles logout in one place.
- `client/src/api/http.js` is the shared API helper example. New auth/user-style endpoint wrappers should call `requestJson()` instead of repeating envelope parsing; `client/src/api/test.js` remains the Phase 1 smoke-test exception.
- `client/src/views/ProfileView.vue` is the current complex page example: it combines auth store state, route-local tab state, profile update, password change, and placeholder-only avatar/favorites areas.
