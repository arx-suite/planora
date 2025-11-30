-- Add up migration script here

/* === tables === */
CREATE TABLE IF NOT EXISTS deleted_record (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    original_table text NOT NULL,
    original_id uuid NOT NULL,
    deleted_at timestamptz NOT NULL DEFAULT now(),
    data jsonb NOT NULL
);


/* === functions / triggers === */
CREATE OR REPLACE FUNCTION archive_deleted_row ()
    RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
DECLARE
    key_name text;
    key_value uuid;
BEGIN
    SELECT
        a.attname INTO key_name
    FROM
        pg_index i
        JOIN pg_attribute a ON a.attrelid = i.indrelid
            AND a.attnum = ANY (i.indkey)
    WHERE
        i.indrelid = TG_RELID
        AND i.indisprimary
    LIMIT 1;
    EXECUTE format('SELECT ($1).%I', key_name) INTO key_value
    USING OLD;
    INSERT INTO deleted_record (original_table, original_id, data)
        VALUES (TG_TABLE_NAME, key_value, to_jsonb (OLD));
    RETURN OLD;
END;
$$;

CREATE OR REPLACE FUNCTION attach_archive_trigger (tablename text)
    RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER trg_%I_deleted_record
         BEFORE DELETE ON %I
         FOR EACH ROW
         EXECUTE FUNCTION archive_deleted_row()', tablename, tablename);
END;
$$;
