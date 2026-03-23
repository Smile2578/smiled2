-- Migration 012: Password reset tokens table
-- Stores one active reset token per user (upsert pattern)

CREATE TABLE password_reset_token (
  user_id    UUID        PRIMARY KEY REFERENCES utilisateur(id) ON DELETE CASCADE,
  token_hash TEXT        NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  used       BOOLEAN     NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_prt_expires ON password_reset_token(expires_at);
