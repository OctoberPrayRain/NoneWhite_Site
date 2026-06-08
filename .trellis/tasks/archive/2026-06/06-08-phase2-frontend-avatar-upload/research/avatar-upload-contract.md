# Avatar Upload Contract Research

## Source Evidence

- `README.md` Phase 2 now leaves only `前端头像上传交互` unchecked in the frontend section.
- `agent/COLLABORATION_PLAN.md` section 10.6 defines the backend avatar upload contract.
- `agent/COLLABORATION_PLAN.md` section 15.4 says frontend avatar upload must use multipart field `avatar`, include `Authorization: Bearer <token>`, and refresh `currentUser.avatarUrl` on success.
- `server/src/routes/users.rs` registers `POST /api/users/me/avatar` and reads multipart field `avatar`.
- `server/src/routes/uploads.rs` serves uploaded avatars from `/uploads/avatars/{file_name}`.
- `agent/JOURNALIST/C_DATABASE_CONTRACTS_DOCS_QA/C_DATABASE_CONTRACTS_DOCS_QA_LOG.md` records that `ProfileView.vue` still has a disabled placeholder and `client/src/api/users.js` has no `uploadAvatar` function.

## Frontend Contract

- Method: `POST`
- Path: `/api/users/me/avatar`
- Auth: required, `Authorization: Bearer <token>`
- Request body: `multipart/form-data`
- File field name: `avatar`
- Response data: `{ "avatarUrl": string }`
- Success envelope message: `Avatar uploaded successfully`
- Static avatar URL format: `/uploads/avatars/user-{id}-{timestamp}.{ext}`

## File Rules

- Backend limit: 2 MiB by default (`MAX_AVATAR_SIZE_BYTES=2097152`).
- Accepted MIME/signature types: `image/png`, `image/jpeg`, `image/webp`.
- Frontend may validate file type and size for user experience, but backend remains final authority.

## Error Constraints

- Missing token: `40102`, `Authentication is required`.
- Invalid/expired token: `40103`, `Token is invalid or expired`.
- Missing `avatar` field: `40006`, `Avatar file is required`.
- Type/signature mismatch: `40007`, `Avatar file type is not allowed`.
- File too large: `40008`, `Avatar file is too large`.

## Non-Goals

- Do not implement Phase 5 generalized file upload.
- Do not change backend storage strategy.
- Do not add favorites API calls.
- Do not commit files under `server/uploads/`.
