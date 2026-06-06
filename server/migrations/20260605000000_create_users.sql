-- Migration: create users table for Phase 2 user system.
-- Contract source: agent/COLLABORATION_PLAN.md sections 9.1 and 13.2.
-- Rollback: drop indexes first, then drop the users table:
--   DROP INDEX IF EXISTS idx_users_username;
--   DROP INDEX IF EXISTS idx_users_email;
--   DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR(32) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  avatar_url TEXT,
  role VARCHAR(32) NOT NULL DEFAULT 'user',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);
