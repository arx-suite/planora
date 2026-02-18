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

insert into plan_resources (plan_name, resource_key, soft_limit, hard_limit, unit)
select plan_name, 'db_storage', 0.1*1024, 0.2*1024, 'mb'
from plans where plan_name = 'free';

insert into plan_resources (plan_name, resource_key, soft_limit, hard_limit, unit)
select plan_name, 'db_storage', 0.8*1024, 1*1024, 'mb'
from plans where plan_name = 'pro';

insert into plan_resources (plan_name, resource_key, soft_limit, hard_limit, unit)
select plan_name, 'db_storage', 2.5*1024, 3*1024, 'mb'
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

-- TODO: check these
/*
create table resource_usage (
    org_id uuid not null,
    resource_key text not null references resources(resource_key),
    used_amount bigint not null default 0,
    period_start timestamptz not null,
    period_end timestamptz not null,
    updated_at timestamptz default now(),
    primary key (org_id, resource_key, period_start)
);

create table if not exists resource_pricing (
    resource_key text primary key references resources(resource_key),
    price_per_unit numeric(12,4) not null,
    billing_unit bigint not null default 1
);

insert into resource_pricing values
    ('api_requests', 0.002, 1000),
    ('file_storage_mb', 0.10, 1024);

create table if not exists usage_overages (
    id uuid primary key default gen_random_uuid(),
    org_id uuid not null,
    resource_key text not null,

    exceeded_amount bigint not null,
    cost numeric(12,4) not null,

    period_start timestamptz not null,
    period_end timestamptz not null,

    created_at timestamptz default now()
);
*/
