-- Add up migration script here

/* === types === */
create type user_status as enum (
    'pending',
    'active',
    'suspended',
    'deactivated',
    'banned'
);


/* === tables === */
create table users (
    user_id uuid primary key default gen_random_uuid(),

	-- user status
    status user_status not null default 'pending',
    deactivated_at timestamptz,
	locked_until timestamptz,
    email_verified_at timestamptz,

    -- core identity
    usertag text not null unique,
    username varchar(255) not null,
    email varchar(255) unique,
	avatar_url text,

    -- security
    password_hash text,
    password_changed_at timestamptz,
    password_reset_required boolean default false,

    -- preferences
	preferences jsonb default '{
		"locale": "en-US",
		"timezone": "UTC",
		"theme": "auto"
	}'::jsonb,

	notifications_settings jsonb default '{
		"email": true,
		"push": true,
		"inapp": true,
		"digest": "daily"
	}'::jsonb,

    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create table user_identities (
    user_id uuid not null references users(user_id) on delete cascade,
    provider text not null,
    provider_email text,
    data jsonb default '{}',
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    primary key (user_id, provider)
);


/* === indexes === */
create index if idx_users_usertag on users(usertag);
create index if idx_users_email on users(email) where email is not null;
create index if idx_user_identities_provider on user_identities(provider);
create index if idx_user_identities_provider_email on user_identities(provider_email) where provider_email is not null;


/* === functions / triggers === */
select attach_archive_trigger('users');
select attach_archive_trigger('user_identities');
