-- Add up migration script here
--1. drop triggers
DROP TRIGGER IF EXISTS access_log_set_updated_at_trigger ON access_log;

--2. drop tables
DROP TABLE IF EXISTS access_log;