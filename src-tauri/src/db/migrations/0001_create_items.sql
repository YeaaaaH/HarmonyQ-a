CREATE TABLE items (
    id         TEXT PRIMARY KEY NOT NULL,
    title      TEXT NOT NULL,
    status     TEXT NOT NULL DEFAULT 'open',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_items_created_at ON items (created_at);
