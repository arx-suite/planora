-- Add down migration script here

/* === tables === */
drop table if not exists features;
drop table if not exists plan_resources;
drop table if not exists plan_quotas;
drop table if not exists plans;


/* === types === */
drop type resource_unit;
