-- Add extra columns to audit_log for middleware-captured request metadata.
-- entity_type and entity_id are made nullable since the middleware logs at the
-- HTTP layer and doesn't always know which entity was touched.
ALTER TABLE audit_log ALTER COLUMN entity_type DROP NOT NULL;
ALTER TABLE audit_log ALTER COLUMN entity_id DROP NOT NULL;

ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS method      TEXT;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS path        TEXT;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS status_code INTEGER;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS request_id  TEXT;
