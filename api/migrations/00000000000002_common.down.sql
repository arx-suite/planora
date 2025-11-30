-- Add down migration script here

/* === functions / triggers === */
DROP FUNCTION IF EXISTS attach_archive_trigger;

DROP FUNCTION IF EXISTS archive_deleted_row;


/* === tables === */
DROP TABLE IF EXISTS deleted_record;
