-- Add up migration script here

/* === tables === */
create table if not exists deleted_record (
    id uuid primary key default gen_random_uuid(),
    original_table text not null,
    original_id uuid not null,
    deleted_at timestamptz not null default now(),
    data jsonb not null
);


/* === functions / triggers === */
create or replace function archive_deleted_row()
returns trigger
language plpgsql
as $$
declare
    key_name text;
    key_value uuid;
begin
    select a.attname into key_name
    from pg_index i
    join pg_attribute a on a.attrelid = i.indrelid
    and a.attnum = any (i.indkey)
    where i.indrelid = TG_RELID and i.indisprimary
    limit 1;

    execute format('select ($1).%I', key_name) into key_value
    using OLD;

    insert into deleted_record (original_table, original_id, data)
    values (TG_TABLE_NAME, key_value, to_jsonb(OLD));
    return OLD;
end;
$$;

create or replace function attach_archive_trigger(tablename text)
returns void
language plpgsql
as $$
begin
    execute format('create trigger trg_%I_deleted_record
        before delete on %I
        for each row
        execute function archive_deleted_row()', tablename, tablename);
end;
$$;
