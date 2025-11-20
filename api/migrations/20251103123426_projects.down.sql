-- Add down migration script here

/* === indexes === */
DROP INDEX IF EXISTS idx_organization_projects;
DROP INDEX IF EXISTS idx_user_projects;

/* === policies === */
DROP POLICY IF EXISTS projects_policy ON projects;

/* === tables === */
DROP TABLE IF EXISTS projects;
