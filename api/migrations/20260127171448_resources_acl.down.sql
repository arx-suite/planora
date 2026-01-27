-- Add down migration script here

/* === tables === */
drop table resources;
drop table organization_features;
drop table organizations;


/* === indexes === */
drop index idx_resources_org;
drop index idx_resources_parent;
drop index idx_resources_type;


/* === types === */
drop type resource_type;
