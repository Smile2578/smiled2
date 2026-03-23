-- Document storage table
CREATE TABLE IF NOT EXISTS document (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id  UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
    cabinet_id  UUID NOT NULL REFERENCES cabinet(id) ON DELETE CASCADE,
    type        TEXT NOT NULL,
    url_storage TEXT NOT NULL,
    filename    TEXT,
    mime_type   TEXT,
    uploaded_by UUID NOT NULL REFERENCES utilisateur(id),
    uploaded_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_document_patient ON document(patient_id);
CREATE INDEX IF NOT EXISTS idx_document_cabinet ON document(cabinet_id);

-- Document–tooth link table
CREATE TABLE IF NOT EXISTS document_dent (
    document_id UUID     NOT NULL REFERENCES document(id) ON DELETE CASCADE,
    dent_fdi    SMALLINT NOT NULL,
    PRIMARY KEY (document_id, dent_fdi)
);

-- RLS for document
ALTER TABLE document ENABLE ROW LEVEL SECURITY;

CREATE POLICY document_cabinet_isolation ON document
    USING (cabinet_id = (current_setting('app.current_tenant', true))::uuid);

-- RLS for document_dent (via document FK)
ALTER TABLE document_dent ENABLE ROW LEVEL SECURITY;

CREATE POLICY document_dent_cabinet_isolation ON document_dent
    USING (
        document_id IN (
            SELECT id FROM document
            WHERE cabinet_id = (current_setting('app.current_tenant', true))::uuid
        )
    );
