-- Migration: create Phase 4 interaction tables for comments, likes, and favorites.
-- Contract source: README.md Phase 4 and agent/COLLABORATION_PLAN.md Phase 4 interaction contract.
-- Rollback: drop dependent interaction tables first:
--   DROP TABLE IF EXISTS favorites;
--   DROP TABLE IF EXISTS likes;
--   DROP TABLE IF EXISTS comments;

CREATE TABLE IF NOT EXISTS comments (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  game_id BIGINT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  content TEXT NOT NULL,
  parent_id BIGINT REFERENCES comments(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_comments_game_created ON comments (game_id, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments (parent_id);

CREATE TABLE IF NOT EXISTS likes (
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  game_id BIGINT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, game_id)
);

CREATE INDEX IF NOT EXISTS idx_likes_game_created ON likes (game_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_likes_user_created ON likes (user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS favorites (
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  game_id BIGINT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, game_id)
);

CREATE INDEX IF NOT EXISTS idx_favorites_game_created ON favorites (game_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_favorites_user_created ON favorites (user_id, created_at DESC);
