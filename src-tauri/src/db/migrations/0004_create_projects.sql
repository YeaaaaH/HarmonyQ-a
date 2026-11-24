CREATE TABLE projects (
    id           TEXT PRIMARY KEY NOT NULL,
    name         TEXT NOT NULL,
    outcome      TEXT NOT NULL DEFAULT '',
    status       TEXT NOT NULL DEFAULT 'active',
    resume_note  TEXT NOT NULL DEFAULT '',
    hue          INTEGER NOT NULL,
    created_at   INTEGER NOT NULL,
    updated_at   INTEGER NOT NULL,
    completed_at INTEGER
);

ALTER TABLE items ADD COLUMN project_id TEXT REFERENCES projects (id) ON DELETE SET NULL;
ALTER TABLE items ADD COLUMN completed_at INTEGER;

CREATE INDEX idx_items_project_id ON items (project_id);
