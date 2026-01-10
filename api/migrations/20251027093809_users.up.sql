-- Add up migration script here

/* === tables === */
create table if not exists users (
    user_id uuid primary key default gen_random_uuid(),
    user_tag text not null unique,
    username varchar(255) not null unique,
    email varchar(255) not null unique,
    password text,
    timezone text,
    avatar_url text,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create table if not exists user_identities (
    identity_id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(user_id) on delete cascade,
    provider text not null,
    provider_user_id text not null,
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    unique (provider, provider_user_id)
);


/* === indexes === */
create index if not exists idx_users_email on users(email);
create index if not exists idx_users_user_tag on users(user_tag);
create index if not exists idx_user_identities_provider on user_identities(provider, provider_user_id);


/* === functions / triggers === */
select attach_archive_trigger('users');
select attach_archive_trigger('user_identities');
