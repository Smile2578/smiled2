-- Better Auth will create its own tables (user, session, account, verification)
-- via its PostgreSQL adapter on first Nuxt startup.
-- We only create the schema here so it exists before Better Auth runs.
CREATE SCHEMA IF NOT EXISTS auth;
