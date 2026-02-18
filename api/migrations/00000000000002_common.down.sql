-- Add down migration script here

/* === functions / triggers === */
drop function attach_archive_trigger;
drop function archive_deleted_row;


/* === tables === */
drop table deleted_record;
