ALTER TABLE items ADD COLUMN due_date TEXT;
ALTER TABLE items ADD COLUMN due_time TEXT;
ALTER TABLE items ADD COLUMN note TEXT NOT NULL DEFAULT '';
ALTER TABLE items ADD COLUMN priority TEXT NOT NULL DEFAULT 'normal';

CREATE INDEX idx_items_due_date ON items (due_date);
