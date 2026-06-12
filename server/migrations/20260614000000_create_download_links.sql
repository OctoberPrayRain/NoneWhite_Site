-- Migration: create Phase 5 download link table for admin-managed game resources.
-- Contract source: README.md Phase 5 and agent/COLLABORATION_PLAN.md Phase 5 backend contract.
-- Rollback: drop the dependent table first:
--   DROP TABLE IF EXISTS download_links;

CREATE TABLE IF NOT EXISTS download_links (
  id BIGSERIAL PRIMARY KEY,
  game_id BIGINT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  platform VARCHAR(64) NOT NULL,
  url TEXT NOT NULL,
  extract_code TEXT,
  password TEXT,
  file_size VARCHAR(64),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_download_links_game_id ON download_links (game_id, id);
