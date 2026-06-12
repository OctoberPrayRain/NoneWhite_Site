-- Migration: create Phase 3 game browsing tables.
-- Contract source: README.md Phase 3 and agent/COLLABORATION_PLAN.md section 11.
-- Rollback: drop dependent tables first, then lookup/content tables:
--   DROP TABLE IF EXISTS screenshots;
--   DROP TABLE IF EXISTS game_tags;
--   DROP TABLE IF EXISTS games;
--   DROP TABLE IF EXISTS tags;
--   DROP TABLE IF EXISTS categories;

CREATE TABLE IF NOT EXISTS categories (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  slug VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS tags (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  slug VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS games (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(128) NOT NULL,
  developer VARCHAR(128) NOT NULL,
  publisher VARCHAR(128) NOT NULL,
  release_date DATE,
  description TEXT NOT NULL,
  cover_url TEXT,
  category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
  search_text TEXT NOT NULL DEFAULT '',
  likes_count INTEGER NOT NULL DEFAULT 0 CHECK (likes_count >= 0),
  favorites_count INTEGER NOT NULL DEFAULT 0 CHECK (favorites_count >= 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS game_tags (
  game_id BIGINT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE RESTRICT,
  PRIMARY KEY (game_id, tag_id)
);

CREATE TABLE IF NOT EXISTS screenshots (
  id BIGSERIAL PRIMARY KEY,
  game_id BIGINT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  url TEXT NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0 CHECK (sort_order >= 0)
);

CREATE INDEX IF NOT EXISTS idx_games_category_id ON games (category_id);
CREATE INDEX IF NOT EXISTS idx_games_release_date ON games (release_date DESC);
CREATE INDEX IF NOT EXISTS idx_game_tags_tag_id ON game_tags (tag_id);
CREATE INDEX IF NOT EXISTS idx_screenshots_game_order ON screenshots (game_id, sort_order, id);
