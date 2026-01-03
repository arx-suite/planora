-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS plans (
    plan_name text PRIMARY KEY,
    plan_level smallint UNIQUE,
    description text NOT NULL
);

CREATE TABLE IF NOT EXISTS plan_limits (
    plan_name text NOT NULL REFERENCES plans (plan_name) ON DELETE CASCADE,
    key text NOT NULL,
    value integer,
    description text,
    PRIMARY KEY (plan_name, key)
);

CREATE TABLE IF NOT EXISTS features (
    feature_name text PRIMARY KEY,
    description text NOT NULL,
    min_plan_level smallint NOT NULL,
    default_enabled boolean NOT NULL DEFAULT FALSE,
    created_at timestamptz DEFAULT now()
);


/* === initial values === */
INSERT INTO plans (plan_name, plan_level, description)
VALUES
    ('free', 1, 'Free plan designed for individuals and small teams with basic usage limits'),
    ('pro', 4, 'Pro plan for growing teams that need higher limits and advanced collaboration'),
    ('enterprise', 7, 'Enterprise plan with maximum limits, advanced controls, and priority support');

INSERT INTO plan_limits (plan_name, key, value, description)
VALUES
    ('free', 'max_spaces', 1, 'Maximum number of spaces that can be created within the organization'),
    ('free', 'max_projects', 5, 'Maximum number of projects allowed per space'),
    ('free', 'max_members', 10, 'Maximum number of members allowed in the organization'),
    ('pro', 'max_spaces', 3, 'Maximum number of spaces that can be created within the organization'),
    ('pro', 'max_projects', 10, 'Maximum number of projects allowed per space'),
    ('pro', 'max_members', 25, 'Maximum number of members allowed in the organization'),
    ('enterprise', 'max_spaces', 10, 'Maximum number of spaces that can be created within the organization'),
    ('enterprise', 'max_projects', 25, 'Maximum number of projects allowed per space'),
    ('enterprise', 'max_members', 100, 'Maximum number of members allowed in the organization');

INSERT INTO features (feature_name, description, min_plan_level)
VALUES
    ('spaces', 'Enable spaces inside an organization', (
            SELECT
                plan_level
            FROM
                plans
            WHERE
                plan_name = 'free')),
    ('audit_logs',
        'Audit log access',
        (
            SELECT
                plan_level
            FROM
                plans
            WHERE
                plan_name = 'pro')),
    ('custom_roles',
        'Custom RBAC roles',
        (
            SELECT
                plan_level
            FROM
                plans
            WHERE
                plan_name = 'enterprise')),
    ('sso',
        'SSO login',
        (
            SELECT
                plan_level
            FROM
                plans
            WHERE
                plan_name = 'enterprise'));
