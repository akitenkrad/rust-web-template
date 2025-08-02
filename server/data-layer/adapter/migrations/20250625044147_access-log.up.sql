-- 1. create tables
CREATE TABLE IF NOT EXISTS access_log (
    log_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    site_id UUID NOT NULL,
    log TEXT NOT NULL,
    created_at TIMESTAMP(3)
    WITH
        TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
        updated_at TIMESTAMP(3)
    WITH
        TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
)
;

-- 2. create triggers
CREATE
OR REPLACE TRIGGER access_log_set_updated_at_trigger BEFORE
UPDATE ON access_log FOR EACH ROW EXECUTE FUNCTION set_updated_at ()
;