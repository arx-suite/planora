-- Add up migration script here

/* === types === */
create type resource_unit as enum (
    'bytes',
    'mb',
    'gb',
    'requests',
    'minutes'
);


/* === tables === */
-- plans and resource limits
create table plans (
    plan_name text primary key,
    plan_level smallint unique not null,
    description text not null,
    is_metered boolean not null default false,
    created_at timestamptz default now()
);

create table plan_quotas (
    plan_name text references plans (plan_name) on delete cascade,
    quota_key text not null,
    quota_value integer not null,
    description text,
    primary key (plan_name, quota_key)
);

create table plan_resources (
    plan_name text references plans (plan_name) on delete cascade,
    description text,

    unit resource_unit not null,
    resource_key text not null,
    soft_limit bigint,
    hard_limit bigint,

    primary key (plan_name, resource_key)
);

-- features
create table features (
    feature_name text primary key,
    description text not null,
    min_plan_level smallint not null,
    default_enabled boolean not null default false,
    created_at timestamptz default now()
);



/* === initial values === */
-- plans
insert into plans (plan_name, plan_level, description, is_metered)
values
    ('free', 1, 'Basic usage for individuals', false),
    ('pro', 4, 'Advanced collaboration', false),
    ('enterprise', 7, 'Unlimited with metered usage', true);

insert into plan_quotas (plan_name, quota_key, quota_value, description)
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

insert into plan_resources (plan_id, resource_key, soft_limit, hard_limit, unit)
select plan_id, 'db_storage', 0.1*1024, 0.2*1024, 'mb'
from plans where plan_name = 'free';

insert into plan_resources (plan_id, resource_key, soft_limit, hard_limit, unit)
select plan_id, 'db_storage', 0.8*1024, 1*1024, 'mb'
from plans where plan_name = 'pro';

insert into plan_resources (plan_id, resource_key, soft_limit, hard_limit, unit)
select plan_id, 'db_storage', 2.5*1024, 3*1024, 'mb'
from plans where plan_name = 'enterprise';

-- features
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
