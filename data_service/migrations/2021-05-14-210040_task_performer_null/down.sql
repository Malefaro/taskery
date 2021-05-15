-- This file should undo anything in `up.sql`

ALTER TABLE tasks ALTER COLUMN performer_id SET NOT NULL;