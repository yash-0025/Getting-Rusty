--  Create Bookmarks table in SQLite
CREATE TABLE IF NOT EXISTS bookmarks (
-- SQLites auto incrementing 64 bit integer ID . Maps directly to Rust i64
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
-- No Not Null Means this column can contain NULL. values. In rust , any database columns that can be NULL must be represented as an Option<T> 
    description TEXT,
-- Stores comma-separated tags eg rust , async web
    tags TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);



