-- Add up migration script here

/* === types === */
create type project_status as enum ('planned', 'active', 'on_hold', 'completed', 'archived');
create type project_visibility as enum ('private', 'team', 'public');
create type project_priority as enum ('low', 'medium', 'high', 'critical');


/* === tables === */
create table projects (
    project_id uuid primary key default gen_random_uuid(),
    organization_id uuid not null references organizations (organization_id) on delete cascade,

    -- metadata
    project_name varchar(100) not null,
    description text,
    tags text[] default '{}',

    -- status
    status project_status not null default 'planned',
    visibility project_visibility not null default 'team',
    priority project_priority not null default 'low',

    -- timeline
    start_date timestamptz,
    target_date timestamptz,
    actual_end_date timestamptz,

    -- TODO: uncomment after the `organization_members` was created
    -- created_by uuid not null references organization_members (member_id),
    created_at timestamptz default now(),
    updated_at timestamptz default now(),

    unique (organization_id, project_name)
);
