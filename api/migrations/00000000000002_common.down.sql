-- Add down migration script here

/* === functions / triggers === */
drop function if exists attach_archive_trigger;
drop function if exists archive_deleted_row;


/* === tables === */
drop table if exists deleted_record;
