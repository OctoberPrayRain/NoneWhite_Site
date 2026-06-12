# Frontend Development Guidelines

> Project-specific conventions for the NoneWhite Vue/Vite frontend.

---

## Overview

The frontend is a Vue 3 + Vite single-page app in `client/`. It uses plain JavaScript, Vue Router, global CSS in `client/src/style.css`, relative `/api` calls, and a lightweight auth store built with Vue refs. The current frontend does not use TypeScript, Pinia, Tailwind, React hooks, or a frontend unit-test framework.

These guidelines document the codebase as it exists now so Trellis implement/check agents preserve project conventions instead of inventing generic frontend structure.

---

## Pre-Development Checklist

Before changing frontend code or docs:

1. Read the relevant route/component/API/store files under `client/src/`.
2. Preserve existing routes `/`, `/test-api`, `/login`, `/register`, and `/profile`.
3. Use relative `/api/...` requests through `client/src/api/` wrappers.
4. Keep avatar upload and favorites as placeholders unless their backend contracts exist.
5. Do not introduce dependencies or tooling unless the task explicitly asks for that change.

---

## Guidelines Index

| Guide | Description | Status |
|-------|-------------|--------|
| [Directory Structure](./directory-structure.md) | Vue/Vite file layout and ownership rules | Project-specific |
| [Component Guidelines](./component-guidelines.md) | Vue SFC patterns, styling, accessibility, and forbidden component behavior | Project-specific |
| [Hook Guidelines](./hook-guidelines.md) | Vue composable/store patterns and data-fetching boundaries | Project-specific |
| [State Management](./state-management.md) | Local route state, shared auth refs, router metadata, and server-state limits | Project-specific |
| [Quality Guidelines](./quality-guidelines.md) | Build, preview-route checks, forbidden dependencies, and review checklist | Project-specific |
| [Type Safety](./type-safety.md) | Plain-JavaScript contract discipline and API envelope validation | Project-specific |

---

## Quality Check

Minimum verification for frontend changes:

```bash
npm --prefix client run build
npm run lint
```

For route or shell changes, also run the built app through Vite preview and confirm these routes return HTTP 200:

```txt
/
/test-api
/login
/register
/profile
```

If a flow depends on PostgreSQL-backed backend behavior, verify with a live backend and database before claiming the happy path is complete. If the environment lacks PostgreSQL or Docker, record the blocker instead of overstating coverage.

---

## Current Frontend Source References

- `client/src/router/index.js` defines route paths and metadata consumed by the header.
- `client/src/components/AppHeader.vue` implements auth-aware navigation and logout.
- `client/src/api/http.js` owns request/envelope parsing and auth headers.
- `client/src/stores/auth.js` owns token persistence and current-user state.
- `client/src/views/LoginView.vue`, `RegisterView.vue`, and `ProfileView.vue` are the current Phase 2 user-system page examples.
