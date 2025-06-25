-- 1. define function
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN
        new.updated_at := ''now'';
        return new;
    END;
' LANGUAGE 'plpgsql';