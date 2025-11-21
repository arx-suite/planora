-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS spaces (
    space_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(organization_id),
    space_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    UNIQUE (organization_id, space_name)
);


/* === indexes === */
CREATE INDEX IF NOT EXISTS idx_organization_spaces ON spaces(organization_id);


/* === functions / triggers === */
CREATE OR REPLACE FUNCTION limit_spaces_per_org()
RETURNS TRIGGER AS $$
DECLARE
    org_plan TEXT;
    space_count INT;
    max_spaces INT;
BEGIN
    SELECT plan INTO org_plan
    FROM organizations
    WHERE organization_id = NEW.organization_id;

    SELECT max_spaces INTO max_spaces
    FROM plans
    WHERE plan_name = org_plan;

    IF max_spaces IS NULL THEN
        RETURN NEW;
    END IF;

    SELECT COUNT(*) INTO space_count
    FROM spaces
    WHERE organization_id = NEW.organization_id;

    IF space_count >= max_spaces THEN
        RAISE EXCEPTION 'Organization % with plan % has reached the space limit (% spaces)',
            NEW.organization_id, org_plan, max_spaces
            USING ERRCODE = 'A0001';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_limit_spaces
BEFORE INSERT ON spaces
FOR EACH ROW EXECUTE FUNCTION limit_spaces_per_org();


/* === policies === */
ALTER TABLE spaces ENABLE ROW LEVEL SECURITY;

CREATE POLICY space_organization_policy ON spaces
USING (organization_id = current_setting('app.organization', true)::UUID)
WITH CHECK (organization_id = current_setting('app.organization', true)::UUID);
