-- Add down migration script here

/* === indexes === */
drop index idx_user_identities_provider_email;
drop index idx_user_identities_provider;
drop index idx_users_email;
drop index idx_users_usertag;


/* === tables === */
drop table user_sessions;
drop table user_identities;
drop table users;


/* === types === */
drop type session_status;
drop type user_status;
