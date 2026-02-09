-- Add down migration script here

/* === indexes === */
drop index if exists idx_user_identities_provider_email;
drop index if exists idx_user_identities_provider;
drop index if exists idx_users_email;
drop index if exists idx_users_usertag;


/* === tables === */
drop table if exists user_identities;
drop table if exists users;
