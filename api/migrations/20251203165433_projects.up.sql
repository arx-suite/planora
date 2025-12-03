-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS projects (
    project_id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    organization_id uuid REFERENCES organizations (organization_id),
    space_id uuid REFERENCES spaces (space_id),
    name varchar(100) NOT NULL,
    description text,
    created_at timestamptz DEFAULT now(),
    updated_at timestamptz DEFAULT now(),
    UNIQUE (organization_id, space_id, name)
);


/* === indexes === */
CREATE INDEX IF NOT EXISTS idx_organization_projects ON projects (organization_id);

CREATE INDEX IF NOT EXISTS idx_space_projects ON projects (space_id);


/* === functions / triggers === */
SELECT
    attach_archive_trigger ('projects');

CREATE OR REPLACE FUNCTION limit_projects_per_space ()
    RETURNS TRIGGER
    AS $$
DECLARE
    org_plan text;
    projects_count int;
    max_projects_count int;
BEGIN
    SELECT
        plan INTO org_plan
    FROM
        organizations
    WHERE
        organization_id = NEW.organization_id;
    SELECT
        max_projects INTO max_projects_count
    FROM
        plans
    WHERE
        plan_name = org_plan;
    IF max_projects_count IS NULL THEN
        RETURN NEW;
    END IF;
    SELECT
        count(*) INTO projects_count
    FROM
        spaces
    WHERE
        organization_id = NEW.organization_id
        AND space_id = NEW.space_id;
    IF projects_count >= max_projects_count THEN
        RAISE EXCEPTION 'Organization % with plan % has reached the project limit (% projects) for the space %',
            NEW.organization_id, org_plan, max_project_count, NEW.space_id
            USING ERRCODE = 'A0001';
        END IF;
        RETURN NEW;
END;
$$
LANGUAGE plpgsql;


CREATE TRIGGER trg_limit_projects
    BEFORE INSERT ON projects
    FOR EACH ROW
    EXECUTE FUNCTION limit_spaces_per_org ();


/* === policies === */
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

CREATE POLICY org_policy_on_projects ON projects
    USING (organization_id = current_setting('app.organization', TRUE)::uuid)
    WITH CHECK (organization_id = current_setting('app.organization', TRUE)::uuid);
