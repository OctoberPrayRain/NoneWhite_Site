# Type Safety

> Type safety patterns in this project.

---

## Overview

The current frontend is plain JavaScript. There is no TypeScript, no generated API client, and no runtime schema library. Type safety is handled through small API wrappers, envelope validation, explicit request payload construction, form validation, and careful field naming against `agent/COLLABORATION_PLAN.md`.

Do not add TypeScript, Zod, Yup, or generated types as part of normal feature work. Those would be separate tooling decisions.

---

## Type Organization

Current request/response shapes are implicit in API wrapper signatures:

```js
register({ username, email, password })
login({ email, password })
updateCurrentUser({ username }, authToken)
changePassword({ currentPassword, newPassword }, authToken)
```

Use backend contract field names exactly as documented:

- Auth token response fields: `token`, `tokenType`, `expiresIn`, `user`.
- User fields: `id`, `username`, `email`, `avatarUrl`, `role`, `createdAt`, `updatedAt`.
- Password change fields: `currentPassword`, `newPassword`.

Do not invent frontend aliases for contracted API fields.

---

## Validation

Runtime validation currently happens in two places:

- `requestJson()` in `client/src/api/http.js` checks that the response has an API envelope and that `body.code === 0`.
- Route forms validate required fields and simple constraints before calling the store/API, such as username length, password length, and matching confirmation passwords.

Examples:

- `RegisterView.vue` validates `confirmPassword` but sends only `username`, `email`, and `password`.
- `ProfileView.vue` validates new password length and prevents sending the same current/new password pair.

---

## Common Patterns

- Use optional chaining and fallback display text for partially loaded data: `currentUser?.email || '邮箱暂未加载'`.
- Use `error instanceof Error ? error.message : ...` when displaying caught errors.
- Use the shared `ApiError` class to preserve backend `status`, `code`, and `data` for callers.
- Use `createAuthHeaders(authToken)` for Bearer token headers instead of repeating header construction.

---

## Forbidden Patterns

- Do not add fields outside the API contract just to make a page easier to render.
- Do not send `confirmPassword`, `avatarUrl`, or favorites data to Phase 2 backend endpoints.
- Do not swallow backend error messages and replace them with generic text when `ApiError.message` is available.
- Do not hardcode sample users or tokens into production frontend code.
