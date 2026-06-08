# Phase 2 README Frontend Status Alignment

## Goal

Align the `README.md` Phase 2 frontend checklist with the frontend work that already exists and has been verified.

This is a documentation/status task only. It must not implement new frontend behavior, backend behavior, database work, avatar upload, favorites data, or Phase 3 features.

## Background Evidence

- `README.md` still shows Phase 2 frontend items unchecked.
- `client/src/router/index.js` already registers `/login`, `/register`, and `/profile`.
- `client/src/views/LoginView.vue` implements the login page and auth-store login flow.
- `client/src/views/RegisterView.vue` implements the register page and keeps `confirmPassword` frontend-only.
- `client/src/components/AppHeader.vue` exposes auth-aware navigation and logout.
- `client/src/stores/auth.js` clears token/current-user state through `logout()` / `clearAuth()`.
- `client/src/views/ProfileView.vue` implements profile display, username update, password change, avatar placeholder, and a Phase 2-only favorites placeholder tab.
- `agent/JOURNALIST/B_FRONTEND_PAGE_INTERACTION/B_FRONTEND_PAGE_INTERACTION_LOG.md` records Vite build success and route preview HTTP 200 checks for `/`, `/test-api`, `/login`, `/register`, and `/profile`.

## Requirements

- Update only the README Phase 2 frontend status block unless verification reveals a directly related wording issue.
- Mark register/login pages complete only because the frontend pages exist and build/route checks have passed.
- Mark logout complete only because `AppHeader.vue` and `stores/auth.js` implement token/current-user clearing.
- Mark the profile page complete only with precise wording that real avatar upload is placeholder-only and remains blocked.
- Mark the favorites tab complete only as a Phase 2 UI placeholder, not as Phase 4 favorites data.
- Keep all DB happy-path items blocked/pending.
- Keep avatar upload API and real avatar upload blocked/pending until storage strategy, upload target, URL form, size limit, and file type whitelist are decided.
- Do not start Phase 3.

## Verification

Run after the README edit:

```bash
git diff --check
npm --prefix client run build
npm run lint
rg -n "注册 / 登录页面|退出登录|个人中心页面|收藏列表选项卡|头像上传 API|数据库 happy path" README.md
rg -n "uploadAvatar|/api/users/me/avatar|FormData|/api/favorites" client/src/api
```

For route-surface confidence, also run Vite preview and confirm HTTP 200 for:

```txt
/
/test-api
/login
/register
/profile
```

## Done Criteria

- README Phase 2 frontend status matches current verified frontend files.
- README does not claim DB-backed happy paths are complete.
- README does not claim real avatar upload or favorites data are implemented.
- Verification commands pass, or any environment limitation is documented.
- Changes are committed and pushed separately from this pre-implementation marker.
