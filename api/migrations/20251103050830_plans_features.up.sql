-- Add up migration script here

/* === tables === */
create table if not exists plans (
    plan_name text primary key,
    plan_level smallint unique,
    description text not null
);

create table if not exists plan_limits (
    plan_name text not null references plans(plan_name) on delete cascade,
    key text not null,
    value integer,
    description text,
    primary key (plan_name, key)
);

create table if not exists features (
    feature_name text primary key,
    description text not null,
    min_plan_level smallint not null,
    default_enabled boolean not null default false,
    created_at timestamptz default now()
);


/* === initial values === */
insert into plans (plan_name, plan_level, description)
values
    ('free', 1, 'Free plan designed for individuals and small teams with basic usage limits'),
    ('pro', 4, 'Pro plan for growing teams that need higher limits and advanced collaboration'),
    ('enterprise', 7, 'Enterprise plan with maximum limits, advanced controls, and priority support');

insert into plan_limits (plan_name, key, value, description)
values
    ('free', 'max_spaces', 1, 'Maximum number of spaces that can be created within the organization'),
    ('free', 'max_projects', 5, 'Maximum number of projects allowed per space'),
    ('free', 'max_members', 10, 'Maximum number of members allowed in the organization'),
    ('pro', 'max_spaces', 3, 'Maximum number of spaces that can be created within the organization'),
    ('pro', 'max_projects', 10, 'Maximum number of projects allowed per space'),
    ('pro', 'max_members', 25, 'Maximum number of members allowed in the organization'),
    ('enterprise', 'max_spaces', 10, 'Maximum number of spaces that can be created within the organization'),
    ('enterprise', 'max_projects', 25, 'Maximum number of projects allowed per space'),
    ('enterprise', 'max_members', 100, 'Maximum number of members allowed in the organization');

insert into features (feature_name, description, min_plan_level)
values
    ('space', 'Enable spaces inside an organization', (
        select plan_level from plans where plan_name = 'free'
    )),
    ('audit_log', 'Audit log access', (
        select plan_level from plans where plan_name = 'pro'
    )),
    ('custom_acl', 'Custom RBAC roles', (
        select plan_level from plans where plan_name = 'enterprise'
    )),
    ('sso', 'SSO login', (
        select plan_level from plans where plan_name = 'enterprise'
    ));
