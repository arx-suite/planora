-- Add down migration script here

/* === tables === */
drop table tasks;
drop table projects;


/* === types === */
drop type task_priority;
drop type task_status;
drop type project_priority;
drop type project_visibility;
drop type project_status;
