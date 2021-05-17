-- This file should undo anything in `up.sql`

ALTER TABLE tasks ALTER COLUMN resolved DROP NOT NULL;