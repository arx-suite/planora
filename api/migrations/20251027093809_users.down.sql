-- Add down migration script here

/* === indexes === */
DROP INDEX IF EXISTS idx_user_identities_provider;

DROP INDEX IF EXISTS idx_app_users_email;

DROP INDEX IF EXISTS idx_app_users_user_tag;


/* === tables === */
DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS user_identities;
