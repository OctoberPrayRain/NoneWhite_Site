# Complete documented unfinished project work

## Scope

Implement only README-documented unfinished work that can be completed in this environment without external infrastructure:

- Resolve the Phase 2/Phase 5 upload-storage decision in docs based on existing local upload implementation.
- Add Phase 5 frontend surfaces for existing backend APIs: admin game CRUD with image upload, game-scoped download-link management, game-scoped comment management, and public download-link display.
- Add minimal Phase 6 keyword search by extending the existing game list/search flow with `keyword` using existing `search_text`/LIKE semantics, plus a frontend search page.
- Improve responsive behavior for existing and newly added pages.
- Add Docker/Nginx local deployment-prep artifacts.

## Out Of Scope

- Do not reimplement existing Phase 5 backend admin/upload/download APIs.
- Do not mark Phase 3/4 browser integration evidence or Phase 5 live PostgreSQL curl evidence complete unless this session obtains real evidence.
- Do not mark online deployment complete without real external infrastructure.
- Do not add Tailwind, Pinia, TypeScript, or a frontend test framework.

## Scenario Contract

1. Happy path: an admin can manage games, upload images, manage download links/comments, and public users can view download links and search games. Passes when frontend build succeeds and real-surface checks pass where available.
2. Edge path: logged-out/non-admin users cannot access admin pages; empty search/download/comment states render without crashes. Passes when UI/API states are observable and backend permission contracts remain unchanged.
3. Regression path: `/`, `/test-api`, `/games`, `/games/:id`, `/profile`, comments, likes, favorites, and existing game filters keep working. Passes when build/lint and route/API checks complete.

## Verification

- `cargo fmt --manifest-path server/Cargo.toml --check`
- `cargo check --manifest-path server/Cargo.toml`
- `cargo test --manifest-path server/Cargo.toml`
- `npm --prefix client run build`
- `npm run lint`
- Browser/curl/Docker checks when required tools and services are available.
