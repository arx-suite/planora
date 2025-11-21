-- Add down migration script here

/* === policy === */
DROP POLICY IF EXISTS space_organization_policy ON projects;

/* === functions / triggers === */
DROP TRIGGER IF EXISTS trg_limit_spaces ON spaces;
DROP FUNCTION IF EXISTS limit_spaces_per_org;

/* === indexes === */
DROP INDEX IF EXISTS idx_organization_spaces;

/* === tables === */
DROP TABLE IF EXISTS spaces;
