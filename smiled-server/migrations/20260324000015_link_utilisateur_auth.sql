-- Link our utilisateur table to Better Auth's user table
-- Better Auth uses TEXT IDs, not UUID
ALTER TABLE utilisateur ADD COLUMN IF NOT EXISTS auth_user_id TEXT UNIQUE;
