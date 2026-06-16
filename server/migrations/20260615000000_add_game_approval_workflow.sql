-- Migration: add moderation workflow fields for user-submitted games.
-- Rollback: drop indexes first, then columns:
--   DROP INDEX IF EXISTS idx_games_submitted_by_user_id;
--   DROP INDEX IF EXISTS idx_games_approval_status;
--   ALTER TABLE games DROP COLUMN IF EXISTS reviewed_at;
--   ALTER TABLE games DROP COLUMN IF EXISTS reviewed_by_user_id;
--   ALTER TABLE games DROP COLUMN IF EXISTS submitted_by_user_id;
--   ALTER TABLE games DROP COLUMN IF EXISTS approval_status;

ALTER TABLE games
  ADD COLUMN IF NOT EXISTS approval_status VARCHAR(32) NOT NULL DEFAULT 'approved',
  ADD COLUMN IF NOT EXISTS submitted_by_user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS reviewed_by_user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS reviewed_at TIMESTAMPTZ;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'games_approval_status_check'
  ) THEN
    ALTER TABLE games
      ADD CONSTRAINT games_approval_status_check
      CHECK (approval_status IN ('pending', 'approved'));
  END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_games_approval_status ON games (approval_status, id);
CREATE INDEX IF NOT EXISTS idx_games_submitted_by_user_id ON games (submitted_by_user_id, id);
