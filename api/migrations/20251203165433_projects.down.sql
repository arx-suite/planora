-- Add down migration script here

/* === indexes === */
DROP INDEX IF EXISTS idx_organization_projects;

DROP INDEX IF EXISTS idx_space_projects;


/* === functions / triggers === */
DROP TRIGGER IF EXISTS trg_limit_projects ON projects;

DROP FUNCTION IF EXISTS limit_projects_per_space;


/* === policies === */
DROP POLICY IF EXISTS org_policy_on_projects ON projects;


/* === tables === */
DROP TABLE IF EXISTS projects;
