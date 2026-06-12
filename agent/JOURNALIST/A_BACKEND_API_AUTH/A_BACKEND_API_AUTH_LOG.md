# Role A Backend/API/Auth Log

## Handoff - Role A

Sub-lane: A1 Config/State/DB, A3 User Data Layer
Task IDs: A-01, A-03
Changed Files: _Fill after each backend handoff_
Contracts Consumed: `agent/COLLABORATION_PLAN.md`, `server/migrations/20260605000000_create_users.sql`
Contracts Changed: _None unless explicitly recorded_
Verification: _Record `cargo fmt`, `cargo check`, `cargo test`, and API regression output_
Manual/API QA: _Record curl commands and observed response bodies_
Known Limits: Register/login/auth middleware are separate follow-up tasks.
Conflict Notes: `server/Cargo.toml`, `server/src/config.rs`, and route files are high-conflict files.
Next Role Needed: Role C should verify contract drift before Phase 2 auth API implementation.

## Handoff - Role A - 2026-06-06

Sub-lane: A2 Response/Error, A4 Auth Flow, A5 Authenticated User Flow
Task IDs: A-02, A-04, A-05, A-06, A-07
Changed Files:
- `server/Cargo.toml`, `server/Cargo.lock`: added `bcrypt` for password hashing and `jsonwebtoken` for JWT issuing/verification.
- `server/src/main.rs`, `server/src/state.rs`, `server/src/db.rs`: added lazy PostgreSQL pool and shared `AppState` injection while preserving `/api/test` startup without immediate DB connection.
- `server/src/error.rs`: centralized contract error responses for auth/user flows.
- `server/src/dto/auth.rs`, `server/src/dto/users.rs`: added `RegisterRequest`, `LoginRequest`, `AuthTokenResponse`, `UpdateUserProfileRequest`, and `ChangePasswordRequest`.
- `server/src/services/auth_service.rs`, `server/src/services/user_service.rs`: implemented validation, bcrypt hashing/verifying via `spawn_blocking`, JWT issue/verify, current-user lookup, profile update, and password change logic.
- `server/src/middleware/auth.rs`: added Bearer token extraction and authenticated user ID resolution.
- `server/src/routes/auth.rs`, `server/src/routes/users.rs`, `server/src/routes/mod.rs`: registered `POST /api/auth/register`, `POST /api/auth/login`, `GET/PATCH /api/users/me`, and `PATCH /api/users/me/password`.
Contracts Consumed: `agent/COLLABORATION_PLAN.md` sections 10.1-10.5, 13.2, 14.1-14.8; `agent/roles/A_BACKEND_API_AUTH.md` A-04 through A-07.
Contracts Changed: none in API shape; implementation follows existing Phase 2 contract. Docs updated separately by Role C for append-only log rule and README status.
Verification:
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 8 tests.
Manual/API QA:
- `GET /api/test`: HTTP 200, `code=0`, existing backend test response preserved.
- `GET /api/users/me` without token: HTTP 401, `code=40102`, `message="Authentication is required"`.
- `POST /api/auth/login` with invalid email: HTTP 400, `code=40002`.
- `POST /api/auth/register` with invalid email: HTTP 400, `code=40002`.
Known Limits:
- Full DB happy path for register/login/current-user/profile/password was not executed because this environment lacks Docker daemon, `docker compose`, `docker-compose`, and `psql`.
- Password validation currently enforces 8+ chars and <=72 bytes for bcrypt safety; username validation enforces 3-32 chars but does not yet restrict character class.
- Avatar upload remains blocked by storage strategy.
Conflict Notes: touched shared files `server/Cargo.toml`, `server/Cargo.lock`, `server/src/routes/mod.rs`, and `server/src/main.rs`.
Next Role Needed: Role C should run PostgreSQL migration and DB-backed curl happy paths in an environment with PostgreSQL, then update README checkboxes if integration passes.

## Handoff - Role A - 2026-06-07

Sub-lane: A1 Config/State/DB
Task IDs: A-01.2
Scenario: Backend env precedence now matches the Phase 2 collaboration contract.
Changed Files:
- `server/src/main.rs`: replaced ad hoc dotenv loading with a focused `load_env_files()` helper that loads `server/.env` before root `.env`, using `CARGO_MANIFEST_DIR` so launcher scripts and manifest-based cargo commands resolve the same files.
- `README.md`: updated the backend env precedence section to match the fixed behavior.
- `.trellis/tasks/06-07-role-a-env-precedence/resume.md`: recorded the required pre-implementation resume marker.
Contracts Consumed:
- `agent/COLLABORATION_PLAN.md` environment variable contract: backend `server/.env` should be the primary backend config source, with root `.env` for project defaults.
- `.trellis/tasks/06-07-role-a-env-precedence/prd.md`: A-01.2 scenario contract for server override, root fallback, and `/api/test` regression.
Contracts Changed: none in API shape; backend env loading behavior now aligns with the existing contract.
Verification:
- TDD red phase: `cargo test --manifest-path server/Cargo.toml load_env_files` first failed because `load_env_files_from_paths` did not exist.
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 10 tests.
- `npm run lint`: passed; Rust fmt/check and frontend Vite production build completed.
- `docker compose config`: passed.
- `docker ps`: failed because Docker daemon socket is unavailable at `/var/run/docker.sock`.
- `psql --version`: failed because `psql` is not installed.
Manual/API QA:
- `PORT=3017 cargo run --manifest-path server/Cargo.toml`, then `curl -i http://127.0.0.1:3017/api/test`: HTTP 200 with `code=0` and the existing backend test API response.
Known Limits:
- DB happy path for register/login/current-user/profile/password remains blocked in this environment because Docker daemon and `psql` are unavailable.
- Avatar upload remains blocked by storage strategy.
Next Role Needed: In a PostgreSQL-capable environment, Role C should run the DB-backed happy path and append evidence; no further Role A env precedence work is pending.

## Handoff - Role A - 2026-06-08

Sub-lane: A5 Authenticated User Flow, A1 Config/State/DB usability
Task IDs: A-07 avatar upload completion, local DB setup support
Changed Files:
- `server/Cargo.toml`: enabled Axum multipart and Tokio fs features for avatar upload.
- `server/src/config.rs`: added `UploadConfig` with `UPLOAD_DIR`, `UPLOAD_PUBLIC_BASE_URL`, and `MAX_AVATAR_SIZE_BYTES`.
- `server/src/dto/users.rs`: added `AvatarUploadResponse`.
- `server/src/error.rs`: added avatar upload and uploaded-file error codes.
- `server/src/services/user_service.rs`: added avatar validation and avatar URL update service.
- `server/src/routes/users.rs`: registered `POST /api/users/me/avatar` multipart handler.
- `server/src/routes/uploads.rs`, `server/src/routes/mod.rs`: added `/uploads/avatars/{file_name}` static avatar serving.
- `.env.example`, `server/.env.example`: documented upload env defaults.
- `.gitignore`: ignored `server/uploads/`.
- `setupDatabase.sh`, `setupDatabase.bat`: added cross-platform Docker-based PostgreSQL startup and users migration script.
- `README.md`: documented setup scripts, upload strategy, and avatar curl example.

Contracts Consumed:
- `agent/COLLABORATION_PLAN.md` section 10.6 avatar upload contract: `POST /api/users/me/avatar`, multipart field `avatar`, response `{ "avatarUrl": string }`.
- Phase 2 local upload strategy: `server/uploads/avatars/`, public URL `/uploads/avatars/...`, max 2 MiB, PNG/JPEG/WebP whitelist.

Contracts Changed: Avatar storage strategy is now defined for local development. Phase 5 can later decide whether to reuse or upgrade this file-upload path.

Verification:
- `cargo fmt --manifest-path server/Cargo.toml`: applied formatting.
- `cargo test --manifest-path server/Cargo.toml`: passed, 15 tests.
- `npm run lint`: passed; Rust fmt/check and frontend Vite production build completed.
- `git diff --check`: passed after removing README trailing whitespace; remaining output is Windows LF-to-CRLF warnings only.
- `where docker`: failed because Docker is not installed or not available in PATH on this machine.
- `where psql`: failed because `psql` is not installed or not available in PATH on this machine.
- `sh -n setupDatabase.sh`: not run because `sh` is not available on this Windows environment.

Manual/API QA:
- DB-backed avatar upload curl was not run in this environment because Docker and `psql` are not available.

Known Limits:
- Register/login/current-user/profile/password/avatar DB happy path still requires a PostgreSQL-capable machine to run `setupDatabase` and curl through the real backend.
- Local uploaded avatar files are intentionally untracked under `server/uploads/`.

Conflict Notes:
- Touched shared backend files `server/Cargo.toml`, `server/src/routes/mod.rs`, `server/src/config.rs`, `.env.example`, `server/.env.example`, and `README.md`.

Next Role Needed:
- Role C should run the DB-backed happy path using `setupDatabase.sh` or `setupDatabase.bat`, append evidence, and decide whether README DB pending checkboxes can be marked complete.

## Handoff - Role A - 2026-06-08 DB Integration Evidence

Sub-lane: A4 Auth Flow, A5 Authenticated User Flow
Task IDs: A-04, A-05, A-06, A-07 DB happy path
Environment:
- Docker Desktop installation could not be completed in this non-admin Windows shell because `winget` source access failed and Docker/WSL feature changes require elevation.
- WSL distro `Ubuntu-Work` already had PostgreSQL 16.14 online on port 5432.
- Created/updated role `nonewhite_user` with password `nonewhite_password` and database `nonewhite_site` owned by that role.
- Applied `server/migrations/20260605000000_create_users.sql` through WSL `psql`.

Verification:
- Confirmed `users` table columns: `id`, `username`, `email`, `password_hash`, `avatar_url`, `role`, `created_at`, `updated_at`.
- Started Windows backend with `DATABASE_URL=postgres://nonewhite_user:nonewhite_password@localhost:5432/nonewhite_site`, `JWT_SECRET=local-integration-secret`, and `PORT=3021`.
- `GET /api/test`: HTTP 200.
- Register happy path: `POST /api/auth/register` returned `code=0`.
- Login happy path: `POST /api/auth/login` returned `code=0`, `tokenType=Bearer`.
- Current user happy path: `GET /api/users/me` with Bearer token returned `code=0`.
- Profile update happy path: `PATCH /api/users/me` returned `code=0` and updated username.
- Password change happy path: `PATCH /api/users/me/password` returned `code=0`.
- Avatar upload happy path: `curl.exe -F avatar=@...png;type=image/png POST /api/users/me/avatar` returned `code=0`, `data.avatarUrl=/uploads/avatars/user-7-1780887306586.png`.
- Static avatar fetch: `GET /uploads/avatars/user-7-1780887306586.png` returned HTTP 200 and 67 bytes.

Known Limits:
- Docker Desktop remains unavailable in this shell; `setupDatabase.bat` correctly fails fast with an installation/PATH message until Docker is installed from an elevated/admin context.
- WSL PostgreSQL was used as the local DB runtime for integration evidence.

Next Role Needed:
- Role C can now treat Phase 2 auth/user/avatar DB happy path as verified and continue with contract/docs/QA cleanup or Phase 3 planning.

## Handoff - Role A - 2026-06-12 Phase 3 Game Browsing Backend

Sub-lane: Phase 3 public game browsing API
Task IDs: Phase 3 backend game browsing
Changed Files:
- `server/src/models/game.rs`, `server/src/models/mod.rs`: added game/category/tag/screenshot database row models.
- `server/src/dto/games.rs`, `server/src/dto/mod.rs`: added Phase 3 query and response DTOs with camelCase JSON output.
- `server/src/repositories/game_repository.rs`, `server/src/repositories/mod.rs`: added SQL data access for categories, tags, filtered/paginated games, detail lookup, game tags, and screenshots.
- `server/src/services/game_service.rs`, `server/src/services/mod.rs`: added pagination normalization, list/detail composition, and service-level DTO assembly.
- `server/src/routes/games.rs`, `server/src/routes/mod.rs`: registered public `GET /api/games`, `GET /api/games/{id}`, `GET /api/categories`, and `GET /api/tags` routes.
- `server/src/error.rs`: added `40403 Game not found` for game detail misses.
Contracts Consumed:
- README Phase 3 game browsing requirements.
- `agent/COLLABORATION_PLAN.md` Phase 3 frontend handoff: list shape `{ list, total, page, pageSize }`, params `page` / `pageSize` / `categoryId` / `tagId`, and detail fields `category` / `tags` / `screenshots`.
Contracts Changed:
- Phase 3 backend now confirms screenshots are embedded in `GET /api/games/{id}` rather than exposed as a separate endpoint.
- Image URLs are returned as stored strings; development seed uses empty string placeholders.
Verification:
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 17 tests.
- `npm run lint`: passed; Rust fmt/check and frontend Vite production build completed.
Manual/API QA:
- Live PostgreSQL-backed curl was not run in this Linux environment because `docker` is not installed.
Known Limits:
- Real `/games` browser/API integration still needs a PostgreSQL-capable environment to apply Phase 3 migration + seed and hit the running backend.
- Phase 4 like/favorite/comment behavior and Phase 5 admin/download/file-upload behavior remain out of scope.
Conflict Notes:
- Touched shared route module `server/src/routes/mod.rs` and shared error module `server/src/error.rs` in a small sequenced backend pass.
Next Role Needed:
- Role C should apply migrations/seeds in a PostgreSQL environment, run real `/api/games` and `/games` integration checks, and then decide whether frontend mock fallback can be treated only as a development fallback.

## Handoff - Role A - 2026-06-13 Phase 4 Interactions Backend

Sub-lane: Phase 4 comments/likes/favorites backend API
Task IDs: Phase 4 backend interactions
Changed Files:
- `server/src/models/interaction.rs`, `server/src/models/mod.rs`: added comment row/owner/parent models for SQLx mapping.
- `server/src/dto/interactions.rs`, `server/src/dto/mod.rs`: added comment list/create DTOs and like/favorite status response DTOs with camelCase JSON.
- `server/src/repositories/interaction_repository.rs`, `server/src/repositories/mod.rs`: added SQL access for comments, idempotent likes/favorites, count refreshes, and user favorite game listing.
- `server/src/services/interaction_service.rs`, `server/src/services/mod.rs`: added comment validation, pagination normalization, game/user existence checks, parent-comment same-game checks, delete permission checks, and favorite list assembly.
- `server/src/routes/interactions.rs`, `server/src/routes/mod.rs`: registered Phase 4 routes under `/api` and reused `auth::authenticated_user_id` for authenticated writes/listing.
- `server/src/error.rs`: added `40009`, `40010`, `40301`, and `40404` constructors.
Contracts Consumed:
- README Phase 4 backend checklist and user-provided endpoint requirements.
- Existing Axum/sqlx architecture: handlers return `Json<ApiResponse<T>>` with `AppResult`, use `AppState.db_pool`, and authenticate through `middleware::auth::authenticated_user_id(headers, &state.config.auth)`.
- `agent/COLLABORATION_PLAN.md` naming/envelope/error conventions.
Contracts Changed:
- Phase 4 backend now defines concrete API shapes for comment list/create/delete, like/unlike, favorite/unfavorite, and current-user favorites list.
- Comment content limit is enforced at 1000 Unicode scalar characters after trimming.
- Favorite list returns existing `GameListResponse`; list-item screenshots are intentionally empty arrays.
Verification:
- Rust LSP diagnostics could not run because `rust-analyzer` is not installed in this environment.
- Initial `cargo fmt --manifest-path server/Cargo.toml --check` reported formatting diffs; `cargo fmt --manifest-path server/Cargo.toml` was run.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml interaction_service`: passed, 5 interaction-service tests.
Manual/API QA:
- Live PostgreSQL-backed curl QA was not run during this implementation pass because it requires an available PostgreSQL/Docker environment.
Known Limits:
- Full Phase 4 DB happy path and permission-path validation still need a PostgreSQL-capable environment: apply migrations, create/login users, create comments/replies, verify admin/user delete rules, and verify like/favorite count refreshes.
- No frontend code was modified; Phase 4 frontend interaction components remain pending.
- No Phase 5 admin/download/file-upload work was started.
Conflict Notes:
- Touched shared backend route module `server/src/routes/mod.rs` and shared error module `server/src/error.rs` in a narrow backend pass.
Next Role Needed:
- Role C should run full verification commands, append contract/docs QA evidence, and in a PostgreSQL-capable environment run live Phase 4 curl checks.


## Handoff - Role A - 2026-06-12 Phase 5 Admin Resources Backend

Sub-lane: Phase 5 admin middleware, reusable upload, game management, download-link backend API
Task IDs: README Phase 5 backend scope; Role A responsibilities
Changed Files:
- `server/src/middleware/auth.rs`: added `admin_user()` helper that reuses Bearer token verification and `users.role`, preserving existing 401 behavior and returning `40301 Permission denied` for non-admin users.
- `server/src/config.rs`, `.env.example`, `server/.env.example`: added `MAX_IMAGE_SIZE_BYTES` for reusable admin image uploads while keeping `MAX_AVATAR_SIZE_BYTES` for avatars.
- `server/src/services/upload_service.rs`: extracted shared PNG/JPEG/WebP MIME + signature + size validation and timestamped file naming used by avatar and admin image upload.
- `server/src/routes/users.rs`: refactored avatar upload to call shared upload service without changing `POST /api/users/me/avatar` response shape.
- `server/src/routes/admin_uploads.rs`, `server/src/dto/uploads.rs`, `server/src/routes/uploads.rs`: added admin image upload `POST /api/admin/uploads/images` returning `imageUrl` and static `/uploads/images/{file_name}` serving.
- `server/src/dto/games.rs`, `server/src/repositories/game_repository.rs`, `server/src/services/game_service.rs`, `server/src/routes/admin_games.rs`: added admin game create/list/update/delete with transactional `games` + `game_tags` + `screenshots` writes.
- `server/src/models/download_link.rs`, `server/src/dto/download_links.rs`, `server/src/repositories/download_link_repository.rs`, `server/src/services/download_link_service.rs`: added download-link CRUD/read model, DTO, validation, repository, and service layer.
- `server/src/error.rs`, `server/src/routes/mod.rs`, module `mod.rs` files: registered Phase 5 modules and error codes.
Contracts Consumed:
- README Phase 5 backend checklist: admin identity middleware, reusable image upload API, game management CRUD API, download link management API.
- Existing API envelope contract `{ code, data, message }`, `AppError`, `AppState`, auth middleware, layered DTO/model/repository/service/route split, SQLx/PostgreSQL style.
- Existing Phase 3 game schema and DTO field mapping; Phase 5 did not duplicate `games`, `categories`, `tags`, `game_tags`, or `screenshots` schema.
Contracts Changed:
- New admin-only routes under `/api/admin/...`; non-admin access returns HTTP 403 / `code=40301` / `message="Permission denied"`.
- New reusable admin image upload contract: `POST /api/admin/uploads/images`, multipart field `image`, response `data.imageUrl`, static `/uploads/images/{file}`.
- New admin game management contract: `GET/POST /api/admin/games`, `PUT/DELETE /api/admin/games/{gameId}`.
- New download link contract: admin CRUD under `/api/admin/games/{gameId}/download-links`, public read `GET /api/games/{gameId}/download-links`.
Verification:
- Rust LSP diagnostics could not run because `rust-analyzer` is not installed in this environment.
- `cargo fmt --manifest-path server/Cargo.toml --check`: passed.
- `cargo check --manifest-path server/Cargo.toml`: passed.
- `cargo test --manifest-path server/Cargo.toml`: passed, 28 tests.
- `npm run lint`: passed; Rust fmt/check and frontend Vite production build completed.
- `sh -n setupDatabase.sh`: passed.
- `docker compose config`: passed.
- `git diff --check`: passed.
Manual/API QA:
- Live PostgreSQL-backed curl QA was not run because Docker daemon access is denied at `/var/run/docker.sock` and the configured local PostgreSQL endpoint `localhost:5432` returned no response. `psql` exists, but no reachable database was available.
Known Limits:
- A PostgreSQL-capable environment still needs to apply `server/migrations/20260614000000_create_download_links.sql`, create/login an admin user, and run admin upload/game CRUD/download-link curl happy paths plus non-admin `40301` permission checks.
- No frontend source was modified; Phase 5 admin pages and front-end download area remain pending.
- The public download-link endpoint intentionally returns contracted `extractCode` and `password` fields for frontend display; no real links or secrets were logged.
Conflict Notes:
- Touched shared backend route module `server/src/routes/mod.rs`, shared error module `server/src/error.rs`, shared config/env examples, README, and collaboration contract in a single sequenced backend/docs pass.
Next Role Needed:
- Role C should run live DB/API QA in an environment with PostgreSQL/Docker access and append evidence without recording real JWTs, production download URLs, or secrets.
