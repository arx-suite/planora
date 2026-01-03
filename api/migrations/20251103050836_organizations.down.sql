-- Add down migration script here

/* === functions / triggers === */
DROP TRIGGER IF EXISTS trg_limit_organizations ON organizations;

DROP FUNCTION IF EXISTS limit_organizations_per_user;


/* === indexes === */
DROP INDEX IF EXISTS idx_organization_owner;


/* === tables === */
DROP TABLE IF EXISTS organization_features;

DROP TABLE IF EXISTS organizations;
