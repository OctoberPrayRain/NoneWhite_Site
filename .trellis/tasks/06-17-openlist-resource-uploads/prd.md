# OpenList Direct Resource Uploads

## Goal

Resource file uploads must go directly to OpenList instead of being persisted under the local `UPLOAD_DIR/resources` path. The existing controlled download model remains: stored download targets are internal markers and public users receive protected download API URLs.

## Decisions

- Apply OpenList-direct upload only to downloadable resource files.
- Keep images, covers, screenshots, and avatars on the current local/public upload path.
- Configure OpenList upload through env vars:
  - `OPENLIST_BASE_URL`
  - `OPENLIST_TOKEN`
  - `OPENLIST_RESOURCE_UPLOAD_DIR` in `openlist:/GoogleDrive/...` marker format.
- Reuse `OPENLIST_TOKEN` for upload auth.
- Upload with a temporary request-time buffer/file if needed, then do not keep persistent local resource copies.
- Fail closed if OpenList config or upload fails; do not fall back to local resource storage.
- Keep existing `/uploads/resources/...` markers downloadable through the protected API for already-created links.
- Generate unique timestamp-suffixed stored filenames in the OpenList destination folder.
- Return/store the final `openlist:/...` marker in the existing `resource_url` response field.
- Return the original filename in the existing `file_name` response field.
- Hide internal `openlist:/...` markers in frontend UI; show neutral uploaded-resource status and original filename where available.
- Keep manual download-link fields available.
- Keep current resource size and file validation behavior.
- Require the configured OpenList destination folder to already exist.
- Do not add orphan cleanup in this task.

## OpenList API Contract

- Upload endpoint: `PUT {OPENLIST_BASE_URL}/api/fs/put`.
- Headers:
  - `Authorization: <OPENLIST_TOKEN>` with raw token, no `Bearer` prefix.
  - `File-Path: <URL-encoded full remote path including filename>`.
  - `Content-Type: application/octet-stream`.
  - `Content-Length: <file byte length>`.
  - `Overwrite: false` to prevent accidental replacement.
- Body: raw file bytes.
- Success response: JSON with `code: 200`.
- Error responses may still use HTTP 200 with non-200 JSON `code`; backend must inspect response body.

## Acceptance Criteria

- `POST /api/uploads/resources` authenticates as today, validates size/non-empty as today, uploads resource bytes to OpenList, and returns `{ resourceUrl, fileName, fileSize }`.
- New resource uploads do not write persistent files under `UPLOAD_DIR/resources`.
- The returned `resourceUrl` is an `openlist:/GoogleDrive/...` marker under `OPENLIST_RESOURCE_UPLOAD_DIR`.
- Missing `OPENLIST_BASE_URL`, `OPENLIST_TOKEN`, or `OPENLIST_RESOURCE_UPLOAD_DIR` causes resource upload to fail with a generic upload failure, while other app features can still run.
- Existing local resource markers still resolve through protected download endpoints.
- Frontend admin and submission resource flows hide raw local/OpenList markers while preserving them in payloads.
- Docs and env examples describe the new OpenList upload settings.
