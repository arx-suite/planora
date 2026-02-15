-- Add up migration script here

/* === types === */
create type project_status as enum ('planned', 'active', 'on_hold', 'completed', 'archived');
create type project_visibility as enum ('private', 'team', 'public');
create type project_priority as enum ('low', 'medium', 'high', 'critical');


/* === tables === */
create table projects (
    project_id uuid primary key default gen_random_id(),
    organization_id uuid references organizations (organization_id) on delete cascade,

    -- metadata
    project_name varchar(100) not null,
    description text,
    tags varchar(50)[] default '{}',

    -- status
    status project_status default 'planned',
    visibility project_visibility default 'team',
    priority text project_priority default 'low',

    -- timeline
    start_date timestamptz,
    target_date timestamptz,
    actual_end_date timestamptz,

    created_by uuid not null references organization_members (member_id),
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);
