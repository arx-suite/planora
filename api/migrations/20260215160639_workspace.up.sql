-- Add up migration script here

/* === tables === */
create table organizations (
    organization_id uuid primary key default gen_random_uuid(),

    -- move this field to organization_members schema
    created_by uuid not null references users (user_id) on delete cascade,

    name varchar(255) not null,
    subdomain varchar(255) not null unique,

    plan text not null references plans (plan_name) default 'free',
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


/* === indexes === */
create index idx_organizations_name on organizations (name);
create index idx_organizations_subdomain on organizations (subdomain);
create index idx_organizations_plan on organizations (plan);
