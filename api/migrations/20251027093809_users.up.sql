-- Add up migration script here

/* === types === */
create type user_status as enum (
    'pending',
    'active',
    'suspended',
    'deactivated',
    'banned'
);

create type session_status as enum (
    'active',
    'revoked',
    'expired',
    'suspicious'
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

create table user_sessions (
    session_id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users (user_id) on delete cascade,

    -- device and network
    user_agent text not null,
    ip_address inet,
    ip_country char(2),

    device_type text,
    device_name text,
    os_name text,

    -- session status
    status session_status not null default 'active',
    revoked_at timestamptz,
    revoked_reason varchar(100),

    -- token metadata
    access_expires_at timestamptz not null,
    refresh_expires_at timestamptz not null,

    -- activity
    last_activity_at timestamptz,
    last_ip inet,

    failed_attempts int not null default 0,

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
create index idx_users_usertag on users(usertag);
create index idx_users_email on users(email) where email is not null;
create index idx_sessions_user on user_sessions(user_id);
create index idx_sessions_status on user_sessions(status);
create index idx_user_identities_provider on user_identities(provider);
create index idx_user_identities_provider_email on user_identities(provider_email) where provider_email is not null;


/* === functions / triggers === */
select attach_archive_trigger('users');
select attach_archive_trigger('user_identities');
