-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS projects (
    project_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(organization_id),
    name VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    UNIQUE (organization_id, name)
);


/* === indexes === */
CREATE INDEX IF NOT EXISTS idx_organization_projects ON projects(organization_id);


/* === policies === */
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

CREATE POLICY projects_policy ON projects
USING (organization_id = current_setting('app.organization', true)::UUID)
WITH CHECK (organization_id = current_setting('app.organization', true)::UUID);
