-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS organizations (
    organization_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(user_id),
    name VARCHAR(100) NOT NULL,
    subdomain VARCHAR(100) NOT NULL UNIQUE,
    plan TEXT NOT NULL DEFAULT 'free',
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS plans (
    plan_name TEXT PRIMARY KEY,
    max_spaces INT,
    max_projects INT,
    max_members INT,
    description TEXT
);


/* === indexes === */
CREATE INDEX IF NOT EXISTS idx_organization_owner ON organizations(owner_id);


/* === functions / triggers === */
CREATE OR REPLACE FUNCTION limit_organizations_per_user()
RETURNS TRIGGER AS $$
DECLARE
    org_count INT;
BEGIN
    SELECT COUNT(*) INTO org_count
    FROM organizations
    WHERE owner_id = NEW.owner_id;

    IF org_count >= 3 THEN
        RAISE EXCEPTION 'Organization creation per user has reached the limit'
            USING ERRCODE = 'A0001';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_limit_organizations
BEFORE INSERT ON organizations
FOR EACH ROW EXECUTE FUNCTION limit_organizations_per_user();


/* === values === */
INSERT INTO
    plans (
        plan_name,
        max_spaces,
        max_projects,
        max_members,
        description
    )
VALUES
    (
        'free',
        3,
        20,
        50,
        'Free plan with limited spaces'
    ),
    ('pro', 10, 50, 100, 'Pro plan with more spaces'),
    (
        'enterprise',
        NULL,
        NULL,
        NULL,
        'Unlimited spaces'
    );
