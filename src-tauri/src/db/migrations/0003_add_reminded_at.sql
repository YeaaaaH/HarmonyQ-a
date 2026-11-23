-- Set once a reminder for the current due date/time has been shown; cleared when they change.
ALTER TABLE items ADD COLUMN reminded_at INTEGER;
