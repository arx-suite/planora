-- Add up migration script here

/* === types === */
create type resource_type as enum ('org', 'space', 'project');


/* === tables === */
-- resources
create table organizations (
    organization_id uuid primary key default gen_random_uuid(),
    name varchar(255) not null,
    subdomain varchar(255) not null unique,
    plan text not null references plans (plan_name) DEFAULT 'free',
    space_enabled boolean not null default false,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create table organization_features (
    organization_id uuid references organizations (organization_id) on delete cascade,
    feature_name text references features (feature_name) on delete cascade,
    enabled boolean not null,
    updated_at timestamptz not null default now(),
    primary key (organization_id, feature_name)
);

create table resources (
    resource_id uuid primary key default gen_random_uuid(),
    resource_type resource_type not null,
    organization_id uuid not null references organizations (organization_id) on delete cascade,
    parent_resource_id uuid references resources (resource_id) on delete cascade,
    name varchar(255) not null,
    description text,
    created_at timestamptz default now(),
    updated_at timestamptz default now(),

    check (
        (resource_type = 'org' and parent_resource_id is null)
        or (resource_type != 'org' and parent_resource_id is not null)
    ),
    unique (organization_id, resource_type, parent_resource_id, name)
);


/* === indexes === */
-- resources
create index if idx_resources_org on resources (organization_id);
create index if idx_resources_parent on resources (parent_resource_id) where parent_resource_id is not null;
create index if idx_resources_type on resources (resource_type);
