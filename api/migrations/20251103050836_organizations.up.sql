-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS organizations (
    organization_id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    owner_id uuid NOT NULL REFERENCES users (user_id),
    name varchar(100) NOT NULL,
    subdomain varchar(100) NOT NULL UNIQUE,
    plan text NOT NULL REFERENCES plans (plan_name),
    space_enabled boolean NOT NULL DEFAULT false,
    created_at timestamptz DEFAULT now(),
    updated_at timestamptz DEFAULT now()
);

CREATE TABLE IF NOT EXISTS organization_features (
    organization_id uuid REFERENCES organizations (organization_id) ON DELETE CASCADE,
    feature_name text REFERENCES features (feature_name) ON DELETE CASCADE,
    enabled boolean NOT NULL,
    updated_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (organization_id, feature_name)
);


/* === indexes === */
CREATE INDEX IF NOT EXISTS idx_organization_owner ON organizations (owner_id);


/* === functions / triggers === */
SELECT
    attach_archive_trigger ('organizations');

CREATE OR REPLACE FUNCTION limit_organizations_per_user ()
    RETURNS TRIGGER
    AS $$
DECLARE
    org_count int;
BEGIN
    SELECT
        COUNT(*) INTO org_count
    FROM
        organizations
    WHERE
        owner_id = NEW.owner_id;
    IF org_count >= 3 THEN
        RAISE EXCEPTION 'Organization creation per user has reached the limit'
            USING ERRCODE = 'A0001';
        END IF;
        RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER trg_limit_organizations
    BEFORE INSERT ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION limit_organizations_per_user ();
